use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::Poll;

use futures_lite::Stream;
use tokio_util::sync::{WaitForCancellationFuture, WaitForCancellationFutureOwned};

use crate::{Context, ContextTracker};

/// This type is used to make the inner enum [`ContextRefInner`] private.
pub struct ContextRef<'a> {
    inner: ContextRefInner<'a>,
}

impl From<Context> for ContextRef<'_> {
    fn from(ctx: Context) -> Self {
        ContextRef {
            inner: ContextRefInner::Owned {
                fut: ctx.token.cancelled_owned(),
                tracker: ctx.tracker,
            },
        }
    }
}

impl<'a> From<&'a Context> for ContextRef<'a> {
    fn from(ctx: &'a Context) -> Self {
        ContextRef {
            inner: ContextRefInner::Ref {
                fut: ctx.token.cancelled(),
            },
        }
    }
}

pin_project_lite::pin_project! {
    /// A reference to a context which implements [`Future`] and can be polled.
    /// Can either be owned or borrowed.
    ///
    /// Create by using the [`From`] implementations.
    #[project = ContextRefInnerProj]
    enum ContextRefInner<'a> {
        Owned {
            #[pin] fut: WaitForCancellationFutureOwned,
            tracker: ContextTracker,
        },
        Ref {
            #[pin] fut: WaitForCancellationFuture<'a>,
        },
    }
}

impl std::future::Future for ContextRefInner<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            ContextRefInnerProj::Owned { fut, .. } => fut.poll(cx),
            ContextRefInnerProj::Ref { fut } => fut.poll(cx),
        }
    }
}

pin_project_lite::pin_project! {
    /// A future with a context attached to it.
    ///
    /// This future will be cancelled when the context is done.
    pub struct FutureWithContext<'a, F> {
        #[pin]
        future: F,
        #[pin]
        ctx: ContextRefInner<'a>,
        _marker: std::marker::PhantomData<&'a ()>,
    }
}

impl<F: Future> Future for FutureWithContext<'_, F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.project();

        match (this.ctx.poll(cx), this.future.poll(cx)) {
            (_, Poll::Ready(v)) => std::task::Poll::Ready(Some(v)),
            (Poll::Ready(_), Poll::Pending) => std::task::Poll::Ready(None),
            _ => std::task::Poll::Pending,
        }
    }
}

pub trait ContextFutExt<Fut> {
    /// Wraps a future with a context and cancels the future when the context is
    /// done.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use scuffle_context::{Context, ContextFutExt};
    /// # tokio_test::block_on(async {
    /// let (ctx, handler) = Context::new();
    ///
    /// tokio::spawn(async {
    ///    // Do some work
    ///    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    /// }.with_context(ctx));
    ///
    /// // Will stop the spawned task and cancel all associated futures.
    /// handler.cancel();
    /// # });
    /// ```
    fn with_context<'a>(self, ctx: impl Into<ContextRef<'a>>) -> FutureWithContext<'a, Fut>
    where
        Self: Sized;
}

impl<F: IntoFuture> ContextFutExt<F::IntoFuture> for F {
    fn with_context<'a>(self, ctx: impl Into<ContextRef<'a>>) -> FutureWithContext<'a, F::IntoFuture>
    where
        F: IntoFuture,
    {
        FutureWithContext {
            future: self.into_future(),
            ctx: ctx.into().inner,
            _marker: std::marker::PhantomData,
        }
    }
}

pin_project_lite::pin_project! {
    /// A stream with a context attached to it.
    ///
    /// This stream will be cancelled when the context is done.
    pub struct StreamWithContext<'a, F> {
        #[pin]
        stream: F,
        #[pin]
        ctx: ContextRefInner<'a>,
        _marker: std::marker::PhantomData<&'a ()>,
    }
}

impl<F: Stream> Stream for StreamWithContext<'_, F> {
    type Item = F::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match (this.ctx.poll(cx), this.stream.poll_next(cx)) {
            (_, Poll::Ready(v)) => std::task::Poll::Ready(v),
            (Poll::Ready(_), Poll::Pending) => std::task::Poll::Ready(None),
            _ => std::task::Poll::Pending,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

pub trait ContextStreamExt<Stream> {
    /// Wraps a stream with a context and stops the stream when the context is
    /// done.
    fn with_context<'a>(self, ctx: impl Into<ContextRef<'a>>) -> StreamWithContext<'a, Stream>
    where
        Self: Sized;
}

impl<F: Stream> ContextStreamExt<F> for F {
    fn with_context<'a>(self, ctx: impl Into<ContextRef<'a>>) -> StreamWithContext<'a, F> {
        StreamWithContext {
            stream: self,
            ctx: ctx.into().inner,
            _marker: std::marker::PhantomData,
        }
    }
}
