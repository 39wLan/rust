#![stable(feature = "futures_api", since = "1.36.0")]

use crate::marker::Unpin;
use crate::ops;
use crate::pin::Pin;
use crate::task::{Context, Poll};

/// `future`代表异步计算
///
/// A future 是可能尚未完成计算的值。 这种“异步值”使线程有可能在等待值变为可用时继续执行有用的工作
///
/// # `poll` 方法
///
/// `future`核心方法，`poll`，试图促使`future`成最终值。如果值未准备好，则此方法不会阻塞。相反，在可能的情况下通过`poll`
///再次唤醒当前任务。传递给`poll` 方法的`context`可以提供[`Waker`]，作为唤醒当前任务的句柄
///
/// 使用`future`时，通常不会直接调用`poll`，而是直接 `.await`值
///
/// [`Waker`]: ../task/struct.Waker.html
#[doc(spotlight)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[stable(feature = "futures_api", since = "1.36.0")]
#[lang = "future_trait"]
pub trait Future {
    /// 完成时产生的值类型
    #[stable(feature = "futures_api", since = "1.36.0")]
    type Output;

    /// 尝试将`future`解析为最终值，如果该值尚不可用，请注册当前任务以进行唤醒。
    ///
    /// # 返回值
    ///
    /// 该函数返回：
    ///
    /// - [`Poll::Pending`] 如果未来还没有准备好
    /// - [`Poll::Ready(val)`] 如果成功完成，`future`将结果`val`返回
    ///
    /// `future`一旦结束，就不应该再`poll`
    ///
    /// When a future is not ready yet, `poll` returns `Poll::Pending` and
    /// stores a clone of the [`Waker`] copied from the current [`Context`].
    /// This [`Waker`] is then woken once the future can make progress.
    /// For example, a future waiting for a socket to become
    /// readable would call `.clone()` on the [`Waker`] and store it.
    /// When a signal arrives elsewhere indicating that the socket is readable,
    /// [`Waker::wake`] is called and the socket future's task is awoken.
    /// Once a task has been woken up, it should attempt to `poll` the future
    /// again, which may or may not produce a final value.
    ///
    /// Note that on multiple calls to `poll`, only the [`Waker`] from the
    /// [`Context`] passed to the most recent call should be scheduled to
    /// receive a wakeup.
    ///
    /// # Runtime characteristics
    ///
    /// Futures alone are *inert*; they must be *actively* `poll`ed to make
    /// progress, meaning that each time the current task is woken up, it should
    /// actively re-`poll` pending futures that it still has an interest in.
    ///
    /// The `poll` function is not called repeatedly in a tight loop -- instead,
    /// it should only be called when the future indicates that it is ready to
    /// make progress (by calling `wake()`). If you're familiar with the
    /// `poll(2)` or `select(2)` syscalls on Unix it's worth noting that futures
    /// typically do *not* suffer the same problems of "all wakeups must poll
    /// all events"; they are more like `epoll(4)`.
    ///
    /// An implementation of `poll` should strive to return quickly, and should
    /// not block. Returning quickly prevents unnecessarily clogging up
    /// threads or event loops. If it is known ahead of time that a call to
    /// `poll` may end up taking awhile, the work should be offloaded to a
    /// thread pool (or something similar) to ensure that `poll` can return
    /// quickly.
    ///
    /// # Panics
    ///
    /// Once a future has completed (returned `Ready` from `poll`), calling its
    /// `poll` method again may panic, block forever, or cause other kinds of
    /// problems; the `Future` trait places no requirements on the effects of
    /// such a call. However, as the `poll` method is not marked `unsafe`,
    /// Rust's usual rules apply: calls must never cause undefined behavior
    /// (memory corruption, incorrect use of `unsafe` functions, or the like),
    /// regardless of the future's state.
    ///
    /// [`Poll::Pending`]: ../task/enum.Poll.html#variant.Pending
    /// [`Poll::Ready(val)`]: ../task/enum.Poll.html#variant.Ready
    /// [`Context`]: ../task/struct.Context.html
    /// [`Waker`]: ../task/struct.Waker.html
    /// [`Waker::wake`]: ../task/struct.Waker.html#method.wake
    #[stable(feature = "futures_api", since = "1.36.0")]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

#[stable(feature = "futures_api", since = "1.36.0")]
impl<F: ?Sized + Future + Unpin> Future for &mut F {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        F::poll(Pin::new(&mut **self), cx)
    }
}

#[stable(feature = "futures_api", since = "1.36.0")]
impl<P> Future for Pin<P>
where
    P: Unpin + ops::DerefMut<Target: Future>,
{
    type Output = <<P as ops::Deref>::Target as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::get_mut(self).as_mut().poll(cx)
    }
}
