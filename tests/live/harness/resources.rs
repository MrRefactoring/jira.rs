use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

type Cleanup = Box<dyn Fn() -> Pin<Box<dyn Future<Output = jira::Result<()>> + Send>> + Send>;

const CLEANUP_ATTEMPTS: u32 = 4;

#[derive(Default)]
pub struct ResourceTracker {
    stack: Vec<Cleanup>,
}

impl ResourceTracker {
    pub fn new() -> Self {
        ResourceTracker::default()
    }

    pub fn defer<F, Fut>(&mut self, teardown: F)
    where
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = jira::Result<()>> + Send + 'static,
    {
        self.stack.push(Box::new(move || Box::pin(teardown())));
    }

    pub async fn cleanup(&mut self) {
        let mut leaked = 0;

        for teardown in std::mem::take(&mut self.stack).into_iter().rev() {
            let mut failure = None;

            for attempt in 0..CLEANUP_ATTEMPTS {
                match teardown().await {
                    Ok(()) => {
                        failure = None;
                        break;
                    }
                    Err(error) if error.is_not_found() => {
                        failure = None;
                        break;
                    }
                    Err(error) => {
                        failure = Some(error);

                        if attempt + 1 < CLEANUP_ATTEMPTS {
                            tokio::time::sleep(Duration::from_millis(500 * u64::from(attempt + 1))).await;
                        }
                    }
                }
            }

            if let Some(error) = failure {
                leaked += 1;
                eprintln!("[live] a resource could not be removed: {error}");
            }
        }

        if leaked > 0 {
            eprintln!("[live] {leaked} resources were left behind; the sweep will collect them");
        }
    }
}
