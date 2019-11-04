#![stable(feature = "futures_api", since = "1.36.0")]

//! 用于处理异步任务的类型和特质

mod poll;
#[stable(feature = "futures_api", since = "1.36.0")]
pub use self::poll::Poll;

mod wake;
#[stable(feature = "futures_api", since = "1.36.0")]
pub use self::wake::{Context, Waker, RawWaker, RawWakerVTable};
