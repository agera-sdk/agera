use futures::Future;

/// Takes a sequence of futures as input and returns
/// a single [`Future`]. The returned future completes with
/// a group (_v_, _i_, _rem_), where _v_ is the output from the first
/// completed future, _i_ is the index of the first completed future
/// from the given iterator and _rem_ are the remaining futures.
/// 
/// # Panics
/// 
/// Panics if the iterator specified contains no futures.
/// 
/// # Example
/// 
/// ```
/// use agera::futures;
/// # async fn f() {
/// let (value, index, remaining) = futures::race(list_of_futures).await;
/// # }
/// ```
/// 
pub async fn race<I, IteratorFuture>(iterable: I) -> (IteratorFuture::Output, usize, Vec<IteratorFuture>)
where
    I: IntoIterator<Item = IteratorFuture>,
    IteratorFuture: Future + Unpin,
{
    let (v, i, remaining) = futures::future::select_all(iterable).await;
    (v, i, remaining)
}

pub async fn all<I, IteratorFuture>(iterable: I) -> Vec<IteratorFuture::Output>
where
    I: IntoIterator<Item = IteratorFuture>,
    IteratorFuture: Future + Unpin,
{
    futures::future::join_all(iterable).await
}

/// Marks an asynchronous function as not implementing `Send`.
pub(crate) macro no_send {
    () => {
        ::futures::future::ready(std::marker::PhantomData::<*const ()>::default()).await;
    },
}