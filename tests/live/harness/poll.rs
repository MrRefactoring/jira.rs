use std::future::Future;
use std::time::Duration;

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
    const ATTEMPTS: u32 = 20;
    const INTERVAL: Duration = Duration::from_millis(500);

    for _ in 0..ATTEMPTS {
        if let Some(value) = attempt().await {
            return value;
        }

        tokio::time::sleep(INTERVAL).await;
    }

    panic!("[live] gave up waiting for {description} after {ATTEMPTS} attempts");
}
