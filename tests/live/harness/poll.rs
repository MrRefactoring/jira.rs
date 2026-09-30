use std::future::Future;
use std::time::Duration;

use jira::Error;

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

pub async fn await_readable<F, Fut, T>(description: &str, mut attempt: F) -> T
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    poll_until(description, move || {
        let call = attempt();

        async move { call.await.ok() }
    })
    .await
}
