use std::future::Future;
use std::time::Duration;

use crate::core::error::{Error, Result};

/// Opt-in retry for transient transport failures, applied by the client to every request it sends.
///
/// Never retries 4xx (including 401 and 429) or a 5xx other than 502/503/504 — those signal client or server logic,
/// and masking them would hide real regressions. Disabled by default.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Total number of attempts including the first. Default: 1 — no retries.
    pub max_attempts: u32,
    /// Delay before the first retry. Default: 500ms.
    pub initial_delay: Duration,
    /// Exponential backoff multiplier. Default: 2.
    pub backoff_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        RetryConfig {
            max_attempts: 1,
            initial_delay: Duration::from_millis(500),
            backoff_factor: 2.0,
        }
    }
}

impl RetryConfig {
    /// `attempts` tries in total, with the default back-off.
    pub fn attempts(max_attempts: u32) -> Self {
        RetryConfig {
            max_attempts: max_attempts.max(1),
            ..RetryConfig::default()
        }
    }

    pub(crate) fn next_delay(&self, current: Duration) -> Duration {
        current.mul_f64(self.backoff_factor)
    }
}

/// How [`with_retry`] behaves.
#[derive(Debug, Clone)]
pub struct RetryOptions {
    /// Total number of attempts including the first. Default: 3.
    pub max_attempts: u32,
    /// Delay before the first retry. Default: 1s.
    pub initial_delay: Duration,
    /// Exponential backoff multiplier. Default: 2.
    pub backoff_factor: f64,
    /// Also retry a 429, waiting out `Retry-After` when the API sent one.
    ///
    /// Off by default: a rate limit asks you to slow the whole client down, and retrying one call in place papers
    /// over that.
    pub retry_rate_limit: bool,
}

impl Default for RetryOptions {
    fn default() -> Self {
        RetryOptions {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            backoff_factor: 2.0,
            retry_rate_limit: false,
        }
    }
}

/// Runs an operation again while it fails in a way worth retrying.
///
/// Retries a transient transport failure and a 502/503/504 gateway error — the same policy the client applies to its
/// own requests, so wrapping a call does not change which failures count as temporary. Everything else is returned on
/// the first attempt: 401, 403, 404 and 500 describe the request, not the moment.
///
/// It is therefore a transient-failure helper, not a poller — to wait on eventually-consistent state, loop on the
/// value rather than on the error.
///
/// ```no_run
/// # use jira::core::{Client, with_retry, RetryOptions};
/// # async fn example(client: &Client) -> jira::Result<()> {
/// let myself: serde_json::Value =
///     with_retry(|| client.get("/rest/api/3/myself").send(), RetryOptions::default()).await?;
/// # Ok(())
/// # }
/// ```
pub async fn with_retry<F, Fut, T>(mut operation: F, options: RetryOptions) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let max_attempts = options.max_attempts.max(1);
    let mut delay = options.initial_delay;

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                let rate_limited = options.retry_rate_limit && error.is_rate_limit();
                let retryable = rate_limited || error.is_transient();

                if !retryable || attempt == max_attempts {
                    return Err(error);
                }

                let wait = error.retry_after().filter(|_| rate_limited).unwrap_or(delay);

                tokio::time::sleep(wait).await;
                delay = delay.mul_f64(options.backoff_factor);
            }
        }
    }

    Err(Error::config(
        "with_retry exhausted every attempt without producing a result",
    ))
}
