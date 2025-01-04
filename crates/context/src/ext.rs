use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::Poll;

use futures_lite::Stream;

use crate::ContextRef;

pub trait ContextFutExt<Fut> {
    /// Wraps a future with a context, allowing the future to be cancelled when
    /// the context is done
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
            ctx: ctx.into(),
            _marker: std::marker::PhantomData,
        }
    }
}

pub trait ContextStreamExt<Stream> {
    /// Wraps a stream with a context, allowing the stream to be stopped when
    /// the context is done
    fn with_context<'a>(self, ctx: impl Into<ContextRef<'a>>) -> StreamWithContext<'a, Stream>
    where
        Self: Sized;
}

impl<F: Stream> ContextStreamExt<F> for F {
    fn with_context<'a>(self, ctx: impl Into<ContextRef<'a>>) -> StreamWithContext<'a, F> {
        StreamWithContext {
            stream: self,
            ctx: ctx.into(),
            _marker: std::marker::PhantomData,
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
        ctx: ContextRef<'a>,
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

pin_project_lite::pin_project! {
    /// A stream with a context attached to it.
    ///
    /// This stream will be cancelled when the context is done.
    pub struct StreamWithContext<'a, F> {
        #[pin]
        stream: F,
        #[pin]
        ctx: ContextRef<'a>,
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
