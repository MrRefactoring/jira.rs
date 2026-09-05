use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

type Cleanup = Box<dyn Fn() -> Pin<Box<dyn Future<Output = jira::Result<()>> + Send>> + Send>;

/// How many times a teardown is attempted before the resource is reported as leaked.
const CLEANUP_ATTEMPTS: u32 = 4;

/// A LIFO cleanup stack for live-test resources.
///
/// Suites register teardown closures as they create things and call [`ResourceTracker::cleanup`] at the end. Cleanup
/// runs in reverse creation order — children before parents — and retries each closure a few times, because Jira
/// Cloud deletes are frequently asynchronous and a just-created resource can briefly answer 404 or 409 to its own
/// deletion.
///
/// A teardown is `Fn` rather than `FnOnce` for exactly that reason: a closure that can only run once cannot be
/// retried, and a retry loop around one is a loop that reports success it did not have.
#[derive(Default)]
pub struct ResourceTracker {
    stack: Vec<Cleanup>,
}

impl ResourceTracker {
    pub fn new() -> Self {
        ResourceTracker::default()
    }

    /// Registers a teardown closure. It runs before everything deferred before it.
    pub fn defer<F, Fut>(&mut self, teardown: F)
    where
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = jira::Result<()>> + Send + 'static,
    {
        self.stack.push(Box::new(move || Box::pin(teardown())));
    }

    /// Runs every deferred closure in reverse order, retrying what fails.
    ///
    /// Best-effort by design: deleting an issue needs the *Delete Issues* project permission, and a token without it
    /// must not turn cleanup into a failing run. What is left behind is reported instead, so the leak is visible
    /// rather than silent.
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
                    // Already gone is the outcome the teardown wanted, however it got there.
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
