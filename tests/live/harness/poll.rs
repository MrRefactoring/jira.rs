use std::future::Future;
use std::time::Duration;

use jira::Error;

/// Waits for state the API reaches on its own schedule.
///
/// Jira is eventually consistent about indexing, permission propagation and asynchronous deletes, so a read taken the
/// instant after a write is a coin toss. This loops on the *value* rather than on a thrown error, which is what
/// separates waiting for consistency from retrying a failure.
pub async fn poll_until<F, Fut, T>(description: &str, mut attempt: F) -> T
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Option<T>>,
{
    const ATTEMPTS: u32 = 60;
    const INTERVAL: Duration = Duration::from_millis(500);

    for _ in 0..ATTEMPTS {
        if let Some(value) = attempt().await {
            return value;
        }

        tokio::time::sleep(INTERVAL).await;
    }

    panic!("[live] gave up waiting for {description} after {ATTEMPTS} attempts");
}

/// Waits for a read to start refusing, which is how an asynchronous delete finishes.
///
/// Jira acknowledges a delete before it has finished making it true, so the read straight after it can still answer
/// the resource in full. It is the same lag as the one after a write, seen from the other side, and every case that
/// deletes something and then asserts it is gone is exposed to it — two of them failed on a GitHub runner, where the
/// round trip is longer than it is from a laptop, having passed locally minutes before.
pub async fn await_refused<F, Fut, T>(description: &str, mut attempt: F) -> Error
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    poll_until(description, move || {
        let call = attempt();

        async move { call.await.err() }
    })
    .await
}
