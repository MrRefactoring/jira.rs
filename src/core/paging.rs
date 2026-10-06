use std::future::Future;

use futures_util::stream::{self, BoxStream, StreamExt, TryStreamExt};

use crate::core::Result;

pub(crate) struct PageStep<T, P> {
    items: Vec<T>,
    next: Option<P>,
}

impl<T, P> PageStep<T, P> {
    pub(crate) fn new(items: Vec<T>, next: Option<P>) -> Self {
        Self { items, next }
    }
}

impl<T, P: PartialEq> PageStep<T, P> {
    pub(crate) fn cursor(items: Vec<T>, requested: &P, next: Option<P>, last: bool) -> Self {
        Self::new(items, next.filter(|position| !last && position != requested))
    }
}

impl<T> PageStep<T, i64> {
    pub(crate) fn offset(items: Vec<T>, start: i64, last: bool, total: Option<i64>, size: Option<i64>) -> Self {
        let count = items.len() as i64;
        let next = start + count;
        let short = size.is_some_and(|size| count < size);
        Self::new(items, (count > 0 && !last && total.is_none_or(|total| next < total) && !short).then_some(next))
    }
}

pub(crate) fn cursor_from_url(next: Option<String>) -> Result<Option<String>> {
    let Some(next) = next.filter(|next| !next.is_empty()) else {
        return Ok(None);
    };
    let Some((_, query)) = next.split_once('?') else {
        return Ok(None);
    };
    let query = query.split('#').next().unwrap_or_default();
    Ok(form_urlencoded::parse(query.as_bytes())
        .find(|(name, _)| name == "cursor")
        .map(|(_, cursor)| cursor.into_owned())
        .filter(|cursor| !cursor.is_empty()))
}

pub(crate) fn stream_pages<'a, R, P, T, F, Fut>(request: R, first: P, fetch: F) -> BoxStream<'a, Result<T>>
where
    R: Clone + Send + 'a,
    P: Send + 'a,
    T: Send + 'a,
    F: Fn(R, P) -> Fut + Send + 'a,
    Fut: Future<Output = Result<PageStep<T, P>>> + Send + 'a,
{
    stream::try_unfold((request, Some(first), fetch), |(request, position, fetch)| async move {
        let Some(position) = position else {
            return Ok::<_, crate::core::Error>(None);
        };

        let step = fetch(request.clone(), position).await?;
        Ok(Some((stream::iter(step.items.into_iter().map(Ok)), (request, step.next, fetch))))
    })
    .try_flatten()
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pages(all: &'static [u8], size: usize, first: i64) -> BoxStream<'static, Result<u8>> {
        stream_pages((), first, move |(), offset| async move {
            let items = all.iter().skip(offset as usize).take(size).copied().collect();
            Ok(PageStep::offset(items, offset, offset as usize + size >= all.len(), None, None))
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
            Ok(PageStep::offset(Vec::<u8>::new(), offset, false, None, None))
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
                Ok(PageStep::offset(vec![1u8], offset, false, None, None))
            } else {
                Err(crate::core::Error::config("the second page is refused"))
            }
        });
        assert_eq!(read.try_next().await.unwrap(), Some(1));
        assert!(read.try_next().await.is_err());
        assert_eq!(read.try_next().await.unwrap(), None);
    }

    #[test]
    fn stops_at_total() {
        assert!(PageStep::offset(vec![1u8, 2], 0, false, Some(2), None).next.is_none());
    }

    #[test]
    fn stops_at_short_page() {
        assert!(PageStep::offset(vec![1u8], 0, false, None, Some(2)).next.is_none());
    }

    #[test]
    fn stops_at_absent_cursor() {
        assert!(PageStep::cursor(vec![1u8], &None::<String>, None, false).next.is_none());
    }

    #[test]
    fn stops_when_cursor_does_not_advance() {
        let position = Some("same".to_owned());
        assert!(PageStep::cursor(vec![1u8], &position, Some(position.clone()), false).next.is_none());
    }

    #[test]
    fn stops_at_last_page() {
        assert!(PageStep::offset(vec![1u8], 0, true, None, None).next.is_none());
    }

    #[test]
    fn extracts_cursor_from_absolute_and_relative_urls() {
        assert_eq!(
            cursor_from_url(Some("https://api.example.org/v1/users?cursor=next%2Bpage".into())).unwrap(),
            Some("next+page".into())
        );
        assert_eq!(cursor_from_url(Some("/v1/users?limit=2&cursor=second".into())).unwrap(), Some("second".into()));
        assert_eq!(cursor_from_url(Some("?cursor=third".into())).unwrap(), Some("third".into()));
    }

    #[test]
    fn absent_empty_or_missing_cursor_ends_walk() {
        assert_eq!(cursor_from_url(None).unwrap(), None);
        assert_eq!(cursor_from_url(Some(String::new())).unwrap(), None);
        assert_eq!(cursor_from_url(Some("/v1/users".into())).unwrap(), None);
        assert_eq!(cursor_from_url(Some("/v1/users?cursor=".into())).unwrap(), None);
        assert_eq!(cursor_from_url(Some("/v1/users?limit=2".into())).unwrap(), None);
    }

    #[test]
    fn ignores_fragment_after_query() {
        assert_eq!(
            cursor_from_url(Some("/v1/users?cursor=next%2Bpage#section?cursor=wrong".into())).unwrap(),
            Some("next+page".into())
        );
    }
}
