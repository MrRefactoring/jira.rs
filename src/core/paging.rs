use std::future::Future;

use futures_util::stream::{self, BoxStream, StreamExt, TryStreamExt};

use crate::core::Result;

pub(crate) trait Paged {
    type Item;

    fn into_step(self, requested: i64) -> PageStep<Self::Item>;
}

pub(crate) struct PageStep<T> {
    items: Vec<T>,
    next: Option<i64>,
}

impl<T> PageStep<T> {
    pub(crate) fn new(items: Vec<T>, start: Option<i64>, last: Option<bool>, requested: i64) -> Self {
        let count = items.len() as i64;
        let next = (!last.unwrap_or(false) && count > 0).then(|| start.unwrap_or(requested) + count);

        PageStep { items, next }
    }
}

pub(crate) fn stream_pages<'a, R, P, F, Fut>(request: R, first: i64, fetch: F) -> BoxStream<'a, Result<P::Item>>
where
    R: Clone + Send + 'a,
    P: Paged,
    P::Item: Send + 'a,
    F: Fn(R, i64) -> Fut + Send + 'a,
    Fut: Future<Output = Result<P>> + Send + 'a,
{
    stream::try_unfold((request, Some(first), fetch), |(request, offset, fetch)| async move {
        let Some(offset) = offset else {
            return Ok::<_, crate::core::Error>(None);
        };

        let step = fetch(request.clone(), offset).await?.into_step(offset);

        Ok(Some((stream::iter(step.items.into_iter().map(Ok)), (request, step.next, fetch))))
    })
    .try_flatten()
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Listing {
        items: Vec<u8>,
        start: i64,
        last: bool,
    }

    impl Paged for Listing {
        type Item = u8;

        fn into_step(self, requested: i64) -> PageStep<u8> {
            PageStep::new(self.items, Some(self.start), Some(self.last), requested)
        }
    }

    fn pages(all: &'static [u8], size: usize, first: i64) -> BoxStream<'static, Result<u8>> {
        stream_pages((), first, move |(), offset| async move {
            let start = offset as usize;
            let items = all.iter().skip(start).take(size).copied().collect::<Vec<_>>();

            Ok(Listing { items, start: offset, last: start + size >= all.len() })
        })
    }

    #[tokio::test]
    async fn walks_every_page_from_the_first_to_the_last() {
        let read: Vec<u8> = pages(&[1, 2, 3, 4, 5], 2, 0).try_collect().await.unwrap();

        assert_eq!(read, [1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn starts_where_the_request_pointed() {
        let read: Vec<u8> = pages(&[1, 2, 3, 4, 5], 2, 3).try_collect().await.unwrap();

        assert_eq!(read, [4, 5]);
    }

    #[tokio::test]
    async fn stops_at_an_empty_page_rather_than_asking_forever() {
        let read: Vec<u8> = stream_pages((), 0, |(), offset| async move {
            Ok(Listing { items: Vec::<u8>::new(), start: offset, last: false })
        })
        .try_collect()
        .await
        .unwrap();

        assert!(read.is_empty());
    }

    #[tokio::test]
    async fn stops_at_the_first_failure() {
        let mut read = stream_pages((), 0, |(), offset| async move {
            if offset == 0 {
                Ok(Listing { items: vec![1u8], start: 0, last: false })
            } else {
                Err(crate::core::Error::config("the second page is refused"))
            }
        });

        assert_eq!(read.try_next().await.unwrap(), Some(1));
        assert!(read.try_next().await.is_err());
    }
}
