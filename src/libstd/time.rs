//! 时间量化
//!
//! Example:
//!
//! ```
//! use std::time::Duration;
//!
//! let five_seconds = Duration::new(5, 0);
//! // 两种声明是等价的
//! assert_eq!(Duration::new(5, 0), Duration::from_secs(5));
//! ```

#![stable(feature = "time", since = "1.3.0")]

use crate::cmp;
use crate::error::Error;
use crate::fmt;
use crate::ops::{Add, AddAssign, Sub, SubAssign};
use crate::sys::time;
use crate::sys_common::mutex::Mutex;
use crate::sys_common::FromInner;

#[stable(feature = "time", since = "1.3.0")]
pub use core::time::Duration;

/// 一种对单调不递减的时钟的量化。它是不透明的，只跟 `Duration` 在一起才有用处
///
/// 瞬间（Instants）总是保证不小于之前创建的测量值，通常在基准测试或者测量操作时间时有用
///
/// 然而要注意，这种瞬间不保证是稳定的。换句话说，底层时钟的每次摆动时间可能不一样长（即，有些秒比其他的长一些）。瞬间
///可能向前跃进，或者经历时间膨胀（加速或者减速），但是必然不会倒退。
///
/// 瞬间是中不透明的类型，只能跟其他的比较。没有任何方法能获取“此瞬间的秒数”。只允许测量两个瞬间直接的时间间隔（或者比较两个瞬间）
///
/// 此`Instant` 结构体的大小可能随着目标操作系统发生变化。
///
/// 例如:
///
/// ```no_run
/// use std::time::{Duration, Instant};
/// use std::thread::sleep;
///
/// fn main() {
///    let now = Instant::now();
///
///    // we sleep for 2 seconds
///    sleep(Duration::new(2, 0));
///    // it prints '2'
///    println!("{}", now.elapsed().as_secs());
/// }
/// ```
///
/// # Underlying System calls
/// Currently, the following system calls are being used to get the current time using `now()`:
///
/// |  Platform |               System call                                            |
/// |:---------:|:--------------------------------------------------------------------:|
/// | Cloud ABI | [clock_time_get (Monotonic Clock)]                                   |
/// | SGX       | [`insecure_time` usercall]. More information on [timekeeping in SGX] |
/// | UNIX      | [clock_time_get (Monotonic Clock)]                                   |
/// | Darwin    | [mach_absolute_time]                                                 |
/// | VXWorks   | [clock_gettime (Monotonic Clock)]                                    |
/// | WASI      | [__wasi_clock_time_get (Monotonic Clock)]                            |
/// | Windows   | [QueryPerformanceCounter]                                            |
///
/// [QueryPerformanceCounter]: https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter
/// [`insecure_time` usercall]: https://edp.fortanix.com/docs/api/fortanix_sgx_abi/struct.Usercalls.html#method.insecure_time
/// [timekeeping in SGX]: https://edp.fortanix.com/docs/concepts/rust-std/#codestdtimecode
/// [__wasi_clock_time_get (Monotonic Clock)]: https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md#clock_time_get
/// [clock_gettime (Monotonic Clock)]: https://linux.die.net/man/3/clock_gettime
/// [mach_absolute_time]: https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/KernelProgramming/services/services.html
/// [clock_time_get (Monotonic Clock)]: https://github.com/NuxiNL/cloudabi/blob/master/cloudabi.txt
///
/// **Disclaimer:** These system calls might change over time.
///
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[stable(feature = "time2", since = "1.8.0")]
pub struct Instant(time::Instant);

/// 一种对系统时钟的量化，在跟外部实体，例如文件系统或者其他进程通信的时候很有用。
///
/// 与 [`Instant`] 类型不同，这个时间**不是单调的**。也就是说你可以在文件系统里保存一个文件，再保存另一个文件，然而第二个文件的 `SystemTime` 比
/// 第一个早。换句话说，现实时间中比较晚发生的操作，可能有更早的`SystemTime`！
///
/// 因此，比较两个 `SystemTime` 实例，获得它们之间时间间距的时候，得到的不是一个可靠的 [`Duration`]，而是一个 [`Result`]，这表示需要要处理可能发生的时间漂移
///
/// 虽然 `SystemTime` 没法直接检视，但是这个模块提供了 [`UNIX_EPOCH `]作为时间锚点，可以用以了解一个 `SystemTime`。通过计算到
/// 这个固定时间点的时间间隔， SystemTime 可以转换成人类可读的时间，或者一些时间的字符串表示
///
/// `SystemTime` 结构体所占的大小可能根据操作系统不同而变化
///
/// [`Instant`]: ../../std/time/struct.Instant.html
/// [`Result`]: ../../std/result/enum.Result.html
/// [`Duration`]: ../../std/time/struct.Duration.html
/// [`UNIX_EPOCH`]: ../../std/time/constant.UNIX_EPOCH.html
///
/// 例如 :
///
/// ```no_run
/// use std::time::{Duration, SystemTime};
/// use std::thread::sleep;
///
/// fn main() {
///    let now = SystemTime::now();
///
///    // we sleep for 2 seconds
///    sleep(Duration::new(2, 0));
///    match now.elapsed() {
///        Ok(elapsed) => {
///            // it prints '2'
///            println!("{}", elapsed.as_secs());
///        }
///        Err(e) => {
///            // an error occurred!
///            println!("Error: {:?}", e);
///        }
///    }
/// }
/// ```
///
/// # Underlying System calls
/// Currently, the following system calls are being used to get the current time using `now()`:
///
/// |  Platform |               System call                                            |
/// |:---------:|:--------------------------------------------------------------------:|
/// | Cloud ABI | [clock_time_get (Realtime Clock)]                                    |
/// | SGX       | [`insecure_time` usercall]. More information on [timekeeping in SGX] |
/// | UNIX      | [clock_gettime (Realtime Clock)]                                     |
/// | DARWIN    | [gettimeofday]                                                       |
/// | VXWorks   | [clock_gettime (Realtime Clock)]                                     |
/// | WASI      | [__wasi_clock_time_get (Realtime Clock)]                             |
/// | Windows   | [GetSystemTimeAsFileTime]                                            |
///
/// [clock_time_get (Realtime Clock)]: https://github.com/NuxiNL/cloudabi/blob/master/cloudabi.txt
/// [gettimeofday]: http://man7.org/linux/man-pages/man2/gettimeofday.2.html
/// [clock_gettime (Realtime Clock)]: https://linux.die.net/man/3/clock_gettime
/// [__wasi_clock_time_get (Realtime Clock)]: https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md#clock_time_get
/// [GetSystemTimeAsFileTime]: https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime
///
/// **Disclaimer:** These system calls might change over time.
///
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[stable(feature = "time2", since = "1.8.0")]
pub struct SystemTime(time::SystemTime);

/// 一种从 `SystemTime` 的 `duration_since` 和 `elapsed` 方法返回的错误。用来表示系统时间在反方向上走了多远
///
/// # Examples
///
/// ```no_run
/// use std::thread::sleep;
/// use std::time::{Duration, SystemTime};
///
/// let sys_time = SystemTime::now();
/// sleep(Duration::from_secs(1));
/// let new_sys_time = SystemTime::now();
/// match sys_time.duration_since(new_sys_time) {
///     Ok(_) => {}
///     Err(e) => println!("SystemTimeError difference: {:?}", e.duration()),
/// }
/// ```
#[derive(Clone, Debug)]
#[stable(feature = "time2", since = "1.8.0")]
pub struct SystemTimeError(Duration);

impl Instant {
    /// 返回对应现在（now）的瞬间。
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Instant;
    ///
    /// let now = Instant::now();
    /// ```
    #[stable(feature = "time2", since = "1.8.0")]
    pub fn now() -> Instant {
        let os_now = time::Instant::now();

        // And here we come upon a sad state of affairs. The whole point of
        // `Instant` is that it's monotonically increasing. We've found in the
        // wild, however, that it's not actually monotonically increasing for
        // one reason or another. These appear to be OS and hardware level bugs,
        // and there's not really a whole lot we can do about them. Here's a
        // taste of what we've found:
        //
        // * #48514 - OpenBSD, x86_64
        // * #49281 - linux arm64 and s390x
        // * #51648 - windows, x86
        // * #56560 - windows, x86_64, AWS
        // * #56612 - windows, x86, vm (?)
        // * #56940 - linux, arm64
        // * https://bugzilla.mozilla.org/show_bug.cgi?id=1487778 - a similar
        //   Firefox bug
        //
        // It seems that this just happens a lot in the wild.
        // We're seeing panics across various platforms where consecutive calls
        // to `Instant::now`, such as via the `elapsed` function, are panicking
        // as they're going backwards. Placed here is a last-ditch effort to try
        // to fix things up. We keep a global "latest now" instance which is
        // returned instead of what the OS says if the OS goes backwards.
        //
        // To hopefully mitigate the impact of this, a few platforms are
        // whitelisted as "these at least haven't gone backwards yet".
        if time::Instant::actually_monotonic() {
            return Instant(os_now);
        }

        static LOCK: Mutex = Mutex::new();
        static mut LAST_NOW: time::Instant = time::Instant::zero();
        unsafe {
            let _lock = LOCK.lock();
            let now = cmp::max(LAST_NOW, os_now);
            LAST_NOW = now;
            Instant(now)
        }
    }

    /// 返回到此瞬间经过的时间段
    ///
    /// # Panics
    ///
    /// 如果 `earlier` 比 `self` 晚，这个方法会恐慌。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::{Duration, Instant};
    /// use std::thread::sleep;
    ///
    /// let now = Instant::now();
    /// sleep(Duration::new(1, 0));
    /// let new_now = Instant::now();
    /// println!("{:?}", new_now.duration_since(now));
    /// ```
    #[stable(feature = "time2", since = "1.8.0")]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.checked_sub_instant(&earlier.0).expect("supplied instant is later than self")
    }

    /// 返回到此瞬间经过的时间段，或者在彼瞬间比此瞬间更晚时返回 None。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::{Duration, Instant};
    /// use std::thread::sleep;
    ///
    /// let now = Instant::now();
    /// sleep(Duration::new(1, 0));
    /// let new_now = Instant::now();
    /// println!("{:?}", new_now.checked_duration_since(now));
    /// println!("{:?}", now.checked_duration_since(new_now)); // None
    /// ```
    #[stable(feature = "checked_duration_since", since = "1.39.0")]
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        self.0.checked_sub_instant(&earlier.0)
    }

    /// 返回到此瞬间经过的时间段，或者在彼瞬间比此瞬间更晚时返回零。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::{Duration, Instant};
    /// use std::thread::sleep;
    ///
    /// let now = Instant::now();
    /// sleep(Duration::new(1, 0));
    /// let new_now = Instant::now();
    /// println!("{:?}", new_now.saturating_duration_since(now));
    /// println!("{:?}", now.saturating_duration_since(new_now)); // 0ns
    /// ```
    #[stable(feature = "checked_duration_since", since = "1.39.0")]
    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or(Duration::new(0, 0))
    }

    /// 返回从此瞬间创建开始到现在经过的时间段。
    ///
    /// # Panics
    ///
    /// 如果当前时间比此瞬间要早，这个函数会恐慌。这在此 `Instant` 是合成来的的时候可能会发生。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread::sleep;
    /// use std::time::{Duration, Instant};
    ///
    /// let instant = Instant::now();
    /// let three_secs = Duration::from_secs(3);
    /// sleep(three_secs);
    /// assert!(instant.elapsed() >= three_secs);
    /// ```
    #[stable(feature = "time2", since = "1.8.0")]
    pub fn elapsed(&self) -> Duration {
        Instant::now() - *self
    }

    /// 如果 `self + duration` 能表示成 `Instant`（也就是说和在底层数据结构能表示的范围内），那么
    ///返回 `Some(t)`，这里的 `t` 是` self + duration` 代表的时间；否则返回 `None。`
    #[stable(feature = "time_checked_add", since = "1.34.0")]
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.0.checked_add_duration(&duration).map(Instant)
    }

    /// 如果 `self - duration` 能表示成 `Instant`（也就是说和在底层数据结构能表示的范围内），那么返回 Some(t)，这里
    ///的 `t` 是 `self - duration` 代表的时间；否则返回 `None`。
    #[stable(feature = "time_checked_add", since = "1.34.0")]
    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        self.0.checked_sub_duration(&duration).map(Instant)
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl Add<Duration> for Instant {
    type Output = Instant;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure. See [`checked_add`] for a version without panic.
    ///
    /// [`checked_add`]: ../../std/time/struct.Instant.html#method.checked_add
    fn add(self, other: Duration) -> Instant {
        self.checked_add(other).expect("overflow when adding duration to instant")
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        self.checked_sub(other).expect("overflow when subtracting duration from instant")
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl SystemTime {
    /// 用来创建 `SystemTime` 的一个时间锚点，也用来了解一个 `SystemTime` 落在什么时间上
    ///
    /// ///这个常量，在所有的系统上都定义为参照系统时钟的 “1970-01-01 00:00:00 UTC”。在一个现存
    ///的 `SystemTime` 对象上用 `duration_since` 可以看出到此时间点有多远。`UNIX_EPOCH + duration` 可以
    ///创建 `SystemTime` 实例，来表达另一个固定的时间点
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::SystemTime;
    ///
    /// match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    ///     Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
    ///     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    /// }
    /// ```
    #[stable(feature = "assoc_unix_epoch", since = "1.28.0")]
    pub const UNIX_EPOCH: SystemTime = UNIX_EPOCH;

    /// 返回对应现在（now）的系统时间。
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    ///
    /// let sys_time = SystemTime::now();
    /// ```
    #[stable(feature = "time2", since = "1.8.0")]
    pub fn now() -> SystemTime {
        SystemTime(time::SystemTime::now())
    }

    /// 返回从先前的一个时间点开始，经过的时间段。
    ///
    /// 因为靠前的测量不保证在靠后的测量之前（因为一些反常现象，例如往前往 后调整系统时钟），这个函数可以失败的。 [`Instant`]
    ///可以用来测量时间段，而没有这种失败的风险。
    ///
    /// 如果成功了，返回 [`Ok`]`(`[`Duration`]`)`，这里的时间段代表从给出的点到这个点经过的时间。
    ///
    /// 如果`earlier` 比 `self` 晚，返回一个 [`Err`]，这个错误包含了到 `self` 这个时间有多久。
    ///
    /// [`Ok`]: ../../std/result/enum.Result.html#variant.Ok
    /// [`Duration`]: ../../std/time/struct.Duration.html
    /// [`Err`]: ../../std/result/enum.Result.html#variant.Err
    /// [`Instant`]: ../../std/time/struct.Instant.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::SystemTime;
    ///
    /// let sys_time = SystemTime::now();
    /// let difference = sys_time.duration_since(sys_time)
    ///                          .expect("Clock may have gone backwards");
    /// println!("{:?}", difference);
    /// ```
    #[stable(feature = "time2", since = "1.8.0")]
    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, SystemTimeError> {
        self.0.sub_time(&earlier.0).map_err(SystemTimeError)
    }

    /// 返回此系统时间从创建到现在的时间区别。
    ///
    /// 因为系统时钟容许漂移和更新（即，系统时钟可以倒退），所以这个函数可能会失败。如果成功了，返回 [`Ok`]`(`[`Duration`]`)`，这里的时间段
    ///表示从此时间到当前时间经过的。
    ///
    /// 想要可靠地测量时间，可以用 [`Instant`]。
    ///
    /// 如果 self 比当前时间要早，返回 [`Err`]，此错误包含当前时间距离 `self` 的时间段。
    ///
    /// [`Ok`]: ../../std/result/enum.Result.html#variant.Ok
    /// [`Duration`]: ../../std/time/struct.Duration.html
    /// [`Err`]: ../../std/result/enum.Result.html#variant.Err
    /// [`Instant`]: ../../std/time/struct.Instant.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread::sleep;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let sys_time = SystemTime::now();
    /// let one_sec = Duration::from_secs(1);
    /// sleep(one_sec);
    /// assert!(sys_time.elapsed().unwrap() >= one_sec);
    /// ```
    #[stable(feature = "time2", since = "1.8.0")]
    pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
        SystemTime::now().duration_since(*self)
    }

    /// 如果` self + duration` 能表示成 `SystemTime`（也就是说和在底层数据结构能表示的范围内），那么返回 `Some(t)`，这里的 `t` 是 `self + duration` 代
    ///表的时间；否则返回 `None`。
    #[stable(feature = "time_checked_add", since = "1.34.0")]
    pub fn checked_add(&self, duration: Duration) -> Option<SystemTime> {
        self.0.checked_add_duration(&duration).map(SystemTime)
    }

   /// 如果` self - duration` 能表示成 `SystemTime`（也就是说和在底层数据结构能表示的范围内），那么返回 `Some(t)`，这里的 `t` 是 `self - duration` 代
    ///表的时间；否则返回 `None`。
    #[stable(feature = "time_checked_add", since = "1.34.0")]
    pub fn checked_sub(&self, duration: Duration) -> Option<SystemTime> {
        self.0.checked_sub_duration(&duration).map(SystemTime)
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl Add<Duration> for SystemTime {
    type Output = SystemTime;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure. See [`checked_add`] for a version without panic.
    ///
    /// [`checked_add`]: ../../std/time/struct.SystemTime.html#method.checked_add
    fn add(self, dur: Duration) -> SystemTime {
        self.checked_add(dur).expect("overflow when adding duration to instant")
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl Sub<Duration> for SystemTime {
    type Output = SystemTime;

    fn sub(self, dur: Duration) -> SystemTime {
        self.checked_sub(dur).expect("overflow when subtracting duration from instant")
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl fmt::Debug for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// 用来创建 `SystemTime` 的一个时间锚点，也用来了解一个 `SystemTime` 落在什么时间上。
///
/// 这个常量，在所有的系统上都定义为参照系统时钟的 “1970-01-01 00:00:00 UTC”。在一个现
///存的 [`SystemTime`] 对象上用 `duration_since` 可以看出到此时间点有多远。`UNIX_EPOCH + duration` 可以创建 [`SystemTime`] 实
///例，来表达另一个固定的时间点。
///
/// [`SystemTime`]: ../../std/time/struct.SystemTime.html
///
/// # Examples
///
/// ```no_run
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// match SystemTime::now().duration_since(UNIX_EPOCH) {
///     Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
///     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
/// }
/// ```
#[stable(feature = "time2", since = "1.8.0")]
pub const UNIX_EPOCH: SystemTime = SystemTime(time::UNIX_EPOCH);

impl SystemTimeError {
    /// 返回代表第二个系统时间到第一个系统时间经过的，正的时间段
    ///
    /// `SystemTimeError` 是从 [`SystemTime`] 的 [`duration_since`] 或者 [`elapsed`] 方法，在第二个系统时间比调用方法的 `self` 更晚的时候，返回的错误
    ///
    /// [`duration_since`]: ../../std/time/struct.SystemTime.html#method.duration_since
    /// [`elapsed`]: ../../std/time/struct.SystemTime.html#method.elapsed
    /// [`SystemTime`]: ../../std/time/struct.SystemTime.html
    ///
    /// # 例子
    ///
    /// ```no_run
    /// use std::thread::sleep;
    /// use std::time::{Duration, SystemTime};
    ///
    /// let sys_time = SystemTime::now();
    /// sleep(Duration::from_secs(1));
    /// let new_sys_time = SystemTime::now();
    /// match sys_time.duration_since(new_sys_time) {
    ///     Ok(_) => {}
    ///     Err(e) => println!("SystemTimeError difference: {:?}", e.duration()),
    /// }
    /// ```
    #[stable(feature = "time2", since = "1.8.0")]
    pub fn duration(&self) -> Duration {
        self.0
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl Error for SystemTimeError {
    fn description(&self) -> &str {
        "other time was not earlier than self"
    }
}

#[stable(feature = "time2", since = "1.8.0")]
impl fmt::Display for SystemTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "second time provided was later than self")
    }
}

impl FromInner<time::SystemTime> for SystemTime {
    fn from_inner(time: time::SystemTime) -> SystemTime {
        SystemTime(time)
    }
}

#[cfg(test)]
mod tests {
    use super::{Duration, Instant, SystemTime, UNIX_EPOCH};

    macro_rules! assert_almost_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = ($a, $b);
            if a != b {
                let (a, b) = if a > b { (a, b) } else { (b, a) };
                assert!(a - Duration::new(0, 1000) <= b, "{:?} is not almost equal to {:?}", a, b);
            }
        }};
    }

    #[test]
    fn instant_monotonic() {
        let a = Instant::now();
        let b = Instant::now();
        assert!(b >= a);
    }

    #[test]
    fn instant_elapsed() {
        let a = Instant::now();
        a.elapsed();
    }

    #[test]
    fn instant_math() {
        let a = Instant::now();
        let b = Instant::now();
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        let dur = b.duration_since(a);
        println!("dur: {:?}", dur);
        assert_almost_eq!(b - dur, a);
        assert_almost_eq!(a + dur, b);

        let second = Duration::new(1, 0);
        assert_almost_eq!(a - second + second, a);
        assert_almost_eq!(a.checked_sub(second).unwrap().checked_add(second).unwrap(), a);

        // checked_add_duration will not panic on overflow
        let mut maybe_t = Some(Instant::now());
        let max_duration = Duration::from_secs(u64::max_value());
        // in case `Instant` can store `>= now + max_duration`.
        for _ in 0..2 {
            maybe_t = maybe_t.and_then(|t| t.checked_add(max_duration));
        }
        assert_eq!(maybe_t, None);

        // checked_add_duration calculates the right time and will work for another year
        let year = Duration::from_secs(60 * 60 * 24 * 365);
        assert_eq!(a + year, a.checked_add(year).unwrap());
    }

    #[test]
    fn instant_math_is_associative() {
        let now = Instant::now();
        let offset = Duration::from_millis(5);
        // Changing the order of instant math shouldn't change the results,
        // especially when the expression reduces to X + identity.
        assert_eq!((now + offset) - now, (now - now) + offset);
    }

    #[test]
    #[should_panic]
    fn instant_duration_since_panic() {
        let a = Instant::now();
        (a - Duration::new(1, 0)).duration_since(a);
    }

    #[test]
    fn instant_checked_duration_since_nopanic() {
        let now = Instant::now();
        let earlier = now - Duration::new(1, 0);
        let later = now + Duration::new(1, 0);
        assert_eq!(earlier.checked_duration_since(now), None);
        assert_eq!(later.checked_duration_since(now), Some(Duration::new(1, 0)));
        assert_eq!(now.checked_duration_since(now), Some(Duration::new(0, 0)));
    }

    #[test]
    fn instant_saturating_duration_since_nopanic() {
        let a = Instant::now();
        let ret = (a - Duration::new(1, 0)).saturating_duration_since(a);
        assert_eq!(ret, Duration::new(0, 0));
    }

    #[test]
    fn system_time_math() {
        let a = SystemTime::now();
        let b = SystemTime::now();
        match b.duration_since(a) {
            Ok(dur) if dur == Duration::new(0, 0) => {
                assert_almost_eq!(a, b);
            }
            Ok(dur) => {
                assert!(b > a);
                assert_almost_eq!(b - dur, a);
                assert_almost_eq!(a + dur, b);
            }
            Err(dur) => {
                let dur = dur.duration();
                assert!(a > b);
                assert_almost_eq!(b + dur, a);
                assert_almost_eq!(a - dur, b);
            }
        }

        let second = Duration::new(1, 0);
        assert_almost_eq!(a.duration_since(a - second).unwrap(), second);
        assert_almost_eq!(a.duration_since(a + second).unwrap_err().duration(), second);

        assert_almost_eq!(a - second + second, a);
        assert_almost_eq!(a.checked_sub(second).unwrap().checked_add(second).unwrap(), a);

        let one_second_from_epoch = UNIX_EPOCH + Duration::new(1, 0);
        let one_second_from_epoch2 =
            UNIX_EPOCH + Duration::new(0, 500_000_000) + Duration::new(0, 500_000_000);
        assert_eq!(one_second_from_epoch, one_second_from_epoch2);

        // checked_add_duration will not panic on overflow
        let mut maybe_t = Some(SystemTime::UNIX_EPOCH);
        let max_duration = Duration::from_secs(u64::max_value());
        // in case `SystemTime` can store `>= UNIX_EPOCH + max_duration`.
        for _ in 0..2 {
            maybe_t = maybe_t.and_then(|t| t.checked_add(max_duration));
        }
        assert_eq!(maybe_t, None);

        // checked_add_duration calculates the right time and will work for another year
        let year = Duration::from_secs(60 * 60 * 24 * 365);
        assert_eq!(a + year, a.checked_add(year).unwrap());
    }

    #[test]
    fn system_time_elapsed() {
        let a = SystemTime::now();
        drop(a.elapsed());
    }

    #[test]
    fn since_epoch() {
        let ts = SystemTime::now();
        let a = ts.duration_since(UNIX_EPOCH + Duration::new(1, 0)).unwrap();
        let b = ts.duration_since(UNIX_EPOCH).unwrap();
        assert!(b > a);
        assert_eq!(b - a, Duration::new(1, 0));

        let thirty_years = Duration::new(1, 0) * 60 * 60 * 24 * 365 * 30;

        // Right now for CI this test is run in an emulator, and apparently the
        // aarch64 emulator's sense of time is that we're still living in the
        // 70s.
        //
        // Otherwise let's assume that we're all running computers later than
        // 2000.
        if !cfg!(target_arch = "aarch64") {
            assert!(a > thirty_years);
        }

        // let's assume that we're all running computers earlier than 2090.
        // Should give us ~70 years to fix this!
        let hundred_twenty_years = thirty_years * 4;
        assert!(a < hundred_twenty_years);
    }
}
