/*!
Work with timing and ticking.
*/

pub use std::time::Duration;
use std::{ops::{Add, AddAssign, Sub, SubAssign}, sync::{Arc, RwLock}};
use crate::{platforms::{if_native_platform, if_browser}, common::*};

mod target;

/// A measurement of a monotonically nondecreasing clock. Opaque and useful only with `Duration`.
/// 
/// Instants are always guaranteed to be no less than any previously measured
/// instant when created.
/// 
/// Instants are opaque types that can only be compared to one another. There is
/// no method to get "the number of seconds" from an instant. Instead, it only
/// allows measuring the duration between two instants (or comparing two
/// instants).
/// 
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant {
    inner: target::Instant,
}

impl Instant {
    /// Returns the elapsed time since `other` or zero
    /// if the `self` instant is earlier than `other`.
    pub fn since(&self, other: Instant) -> Duration {
        self.inner.since(other.inner)
    }

    /// Returns the current instant from the host environment.
    pub fn now() -> Instant {
        Self { inner: target::Instant::now() }
    }

    /// Adds a duration to the instant, returning a new instant.
    /// `None` is returned if the result is earlier or later than
    /// the range that `Instant` can represent.
    pub fn try_add(&self, duration: Duration) -> Option<Instant> {
        Some(Self { inner: self.inner.try_add(duration)? })
    }

    /// Subtracts a duration from the instant, returning a new instant.
    /// `None` is returned if the result is earlier or later than
    /// the range that `Instant` can represent.
    pub fn try_subtract(&self, duration: Duration) -> Option<Instant> {
        Some(Self { inner: self.inner.try_subtract(duration)? })
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Self { inner: self.inner + rhs }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.inner += rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Self { inner: self.inner - rhs }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Self::Output {
        self.inner - rhs.inner
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.inner -= rhs;
    }
}

/// Ticker returned by [`ticker`],
/// [`ticker_at`], [`animation_ticker`] and
/// [`animation_ticker_at`].
#[derive(Debug)]
pub struct Ticker {
    inner: target::Ticker,
}

impl Ticker {
    /// Completes when the next instant in the ticker has been reached,
    /// yielding the time elapsed since the last tick.
    pub async fn tick(&mut self) -> Duration {
        self.inner.tick().await
    }
}

/// Asynchronously waits until `duration` has elapsed.
///
/// Equivalent to `wait_until(Instant::now() + duration)`.
/// 
/// No work is performed while awaiting on the wait future to complete. This
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
/// 
/// To run something regularly on a schedule, see ticker functions in this module.
/// 
/// The maximum duration for a wait is 68719476734 milliseconds (approximately 2.2 years).
/// 
/// # Cancellation
///
/// Canceling a wait being awaited for via the `.await` operator is not possible.
/// Use [`free_timeout`] for such a purpose.
/// 
/// # Examples
/// 
/// Wait 100ms and print "100 ms have elapsed".
/// 
/// ```
/// use agera::timer::*;
///
/// async fn example_fn() {
///     wait(Duration::from_millis(100)).await;
///     println!("100 ms have elapsed");
/// }
/// ```
/// 
pub async fn wait(duration: Duration) {
    if_native_platform! {{
        future::no_send!();
        tokio::time::sleep(duration).await;
    }}
    if_browser! {{
        target::browser::wait(duration).await;
    }}
}

/// Asynchronously waits until `deadline` is reached.
///
/// No work is performed while awaiting on the wait future to complete. This
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
///
/// To run something regularly on a schedule, see ticker functions in this module.
///
/// The maximum duration for a wait is 68719476734 milliseconds (approximately 2.2 years).
///
/// # Cancellation
///
/// Canceling a wait being awaited for via the `.await` operator is not possible.
/// Use [`free_timeout`] for such a purpose.
/// 
/// # Examples
/// 
/// Wait 100ms and print "100 ms have elapsed".
/// 
/// ```
/// use agera::timer::*;
///
/// async fn example_fn() {
///     wait(Instant::now() + Duration::from_millis(100)).await;
///     println!("100 ms have elapsed");
/// }
/// ```
/// 
pub async fn wait_until(deadline: Instant) {
    if_native_platform! {{
        future::no_send!();
        tokio::time::sleep_until(deadline.inner.0).await;
    }}
    if_browser! {{
        target::browser::wait(deadline.since(Instant::now())).await;
    }}
}

/// Creates a new [`Ticker`] that yields with ticker of `period`. The first
/// tick completes immediately.
///
/// An ticker will tick indefinitely.
/// 
/// # Animation tickers
/// 
/// For animation tickers, you might want to use [`animation_ticker`]
/// instead of `ticker`.
/// 
/// # Cancellation
///
/// An ticker is disposed when its variable is dropped.
/// Use [`free_interval!`] if you need an ticker that runs
/// separately and can be cancelled dynamically.
///
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
/// 
/// ```
/// use agera::timer::*;
///
/// async fn example_fn() {
///     let mut ticker = ticker(Duration::from_millis(10));
///     ticker.tick().await; // ticks immediately
///     ticker.tick().await; // ticks after 10ms
///     ticker.tick().await; // ticks after 10ms
///
///     // approximately 20ms have elapsed.
/// }
/// ```
/// 
/// A simple example using `ticker` to execute a task every two seconds.
///
/// The difference between `ticker` and [`wait`] is that an [`Ticker`]
/// measures the time since the last tick, which means that [`.tick().await`]
/// may wait for a shorter time than the duration specified for the ticker
/// if some time has passed between calls to [`.tick().await`].
///
/// If the tick in the example below was replaced with [`wait`], the task
/// would only be executed once every three seconds, and not every two
/// seconds.
///
/// ```
/// use agera::timer::*;
///
/// async fn task_that_takes_a_second() {
///     println!("hello");
///     wait(Duration::from_secs(1)).await
/// }
///
/// async fn example() {
///     let mut ticker = ticker(Duration::from_secs(2));
///     for _i in 0..5 {
///         ticker.tick().await;
///         task_that_takes_a_second().await;
///     }
/// }
/// ```
/// 
/// [`.tick().await`]: Ticker::tick
///
pub fn ticker(period: Duration) -> Ticker {
    if_native_platform! {{
        return Ticker {
            inner: target::native::Ticker(tokio::time::interval(period)),
        };
    }}
    if_browser! {{
        assert!(period.as_millis() != 0, "agera::timer::ticker() must be called with non-zero period");
        return Ticker {
            inner: target::browser::Ticker {
                for_animation: false,
                period,
                start: Instant::now(),
                ticker: None,
            },
        };
    }}
}

/// Creates a new [`Ticker`] that yields with ticker of `period` with the
/// first tick completing at `start`.
///
/// # Animation tickers
/// 
/// For animation tickers, you might want to use [`animation_ticker_at`]
/// instead of `ticker_at`.
/// 
/// # Cancellation
///
/// An ticker is disposed when its variable is dropped.
/// Use [`free_interval!`] if you need an ticker that runs
/// separately and can be cancelled dynamically.
/// 
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
///
/// ```
/// use agera::timer::*;
///
/// async fn example() {
///     let start = Instant::now() + Duration::from_millis(50);
///     let mut ticker = ticker_at(start, Duration::from_millis(10));
///
///     ticker.tick().await; // ticks after 50ms
///     ticker.tick().await; // ticks after 10ms
///     ticker.tick().await; // ticks after 10ms
///
///     // approximately 70ms have elapsed.
/// }
/// ```
/// 
pub fn ticker_at(start: Instant, period: Duration) -> Ticker {
    if_native_platform! {{
        return Ticker {
            inner: target::native::Ticker(tokio::time::interval_at(start.inner.0, period)),
        };
    }}
    if_browser! {{
        assert!(period.as_millis() != 0, "agera::timer::ticker_at() must be called with non-zero period");
        return Ticker {
            inner: target::browser::Ticker {
                for_animation: false,
                period,
                start: start,
                ticker: None,
            },
        };
    }}
}

/// Creates a new [`Ticker`] that yields with ticker of `period`. The first
/// tick completes immediately, meant for animations.
///
/// An ticker will tick indefinitely.
/// 
/// # Cancellation
///
/// An ticker is disposed when its variable is dropped.
/// Use [`free_animation_interval`] if you need an ticker that runs
/// separately and can be cancelled dynamically.
///
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
/// 
/// ```
/// use agera::timer::*;
///
/// async fn example_fn() {
///     let mut ticker = animation_ticker(Duration::from_millis(10));
///     ticker.tick().await; // ticks immediately
///     ticker.tick().await; // ticks after 10ms
///     ticker.tick().await; // ticks after 10ms
///
///     // approximately 20ms have elapsed.
/// }
/// ```
/// 
/// [`.tick().await`]: Ticker::tick
///
pub fn animation_ticker(period: Duration) -> Ticker {
    if_native_platform! {{
        return Ticker {
            inner: target::native::Ticker(tokio::time::interval(period)),
        };
    }}
    if_browser! {{
        assert!(period.as_millis() != 0, "agera::timer::ticker() must be called with non-zero period");
        return Ticker {
            inner: target::browser::Ticker {
                for_animation: true,
                period,
                start: Instant::now(),
                ticker: None,
            },
        };
    }}
}

/// Creates a new [`Ticker`] that yields with ticker of `period` with the
/// first tick completing at `start`, meant for animations.
///
/// # Cancellation
///
/// An ticker is disposed when its variable is dropped.
/// Use [`free_animation_interval`] if you need an ticker that runs
/// separately and can be cancelled dynamically.
/// 
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
///
/// ```
/// use agera::timer::*;
///
/// async fn example() {
///     let start = Instant::now() + Duration::from_millis(50);
///     let mut ticker = animation_ticker_at(start, Duration::from_millis(10));
///
///     ticker.tick().await; // ticks after 50ms
///     ticker.tick().await; // ticks after 10ms
///     ticker.tick().await; // ticks after 10ms
///
///     // approximately 70ms have elapsed.
/// }
/// ```
/// 
pub fn animation_ticker_at(start: Instant, period: Duration) -> Ticker {
    if_native_platform! {{
        return Ticker {
            inner: target::native::Ticker(tokio::time::interval_at(start.inner.0, period)),
        };
    }}
    if_browser! {{
        assert!(period.as_millis() != 0, "agera::timer::ticker_at() must be called with non-zero period");
        return Ticker {
            inner: target::browser::Ticker {
                for_animation: true,
                period,
                start: start,
                ticker: None,
            },
        };
    }}
}

/// Executes an action after some elapsed time. This macro
/// returns a `FreeTimeout` object with a `stop()` method that can
/// be used to stop the execution of the action.
///
/// # Syntax
/// 
/// ```
/// use agera::timer::*;
/// let timeout: FreeTimeout = free_timeout!({
///     // Action
/// }, duration);
/// ```
pub macro free_timeout {
    ($action:block, $duration:expr) => {
        ::agera::timer::free_timeout(Box::new(move || $action))
    },
}

#[doc(hidden)]
pub fn free_timeout(callback: Box<(dyn Fn() + Send + Sync + 'static)>, duration: Duration) -> FreeTimeout {
    let mut stopped = Arc::new(RwLock::new(false));
    future::exec({
        let stopped = Arc::clone(&mut stopped);
        async move {
            wait(duration).await;
            if !*stopped.read().unwrap() {
                callback();
            }
        }
    });
    FreeTimeout {
        stopped,
    }
}

/// A timeout that can be stopped at anytime, returned
/// from the [`free_timeout!`] macro.
/// 
/// To stop the timeout, call `timeout.stop`.
pub struct FreeTimeout {
    // inner: target::FreeTimeout,
    stopped: Arc<RwLock<bool>>,
}

impl FreeTimeout {
    pub fn stop(&self) {
        *self.stopped.write().unwrap() = true;
    }
}

/// Executes a given function after each period using an animation ticker.
/// This macro returns a `FreeInterval` object with a `stop()` method that can
/// be used to stop the execution of the function and dispose of the ticker.
/// 
/// The callback function receives the elapsed time since the last time
/// it was called by this function.
/// 
/// # Syntax
/// 
/// ```ignore
/// use agera::timer::*;
/// let ticker: FreeInterval = free_animation_interval!(move |delta| {
///     // Action
/// }, period);
/// ```
pub macro free_animation_interval {
    ($function:expr, $period:expr) => {
        ::agera::timer::free_animation_interval(Box::new($function), $period)
    },
}

#[doc(hidden)]
pub fn free_animation_interval(callback: Box<(dyn Fn(Duration) + Send + Sync + 'static)>, period: Duration) -> FreeInterval {
    let mut stopped = Arc::new(RwLock::new(false));
    future::exec({
        let stopped = Arc::clone(&mut stopped);
        async move {
            let mut ticker = animation_ticker(period);
            ticker.tick().await;
            loop {
                let delta = ticker.tick().await;
                if *stopped.read().unwrap() {
                    break;
                }
                callback(delta);
            }
        }
    });
    FreeInterval {
        stopped,
    }
}

/// Executes a given function after each period using a regular ticker.
/// This macro returns a `FreeInterval` object with a `stop()` method that can
/// be used to stop the execution of the function and dispose of the ticker.
/// 
/// The callback function receives the elapsed time since the last time
/// it was called by this function.
/// 
/// # Syntax
/// 
/// ```ignore
/// use agera::timer::*;
/// let ticker: FreeInterval = free_interval!(move |delta| {
///     // Action
/// }, period);
/// ```
pub macro free_interval {
    ($function:expr, $period:expr) => {
        ::agera::timer::free_interval(Box::new($function), $period)
    },
}

#[doc(hidden)]
pub fn free_interval(callback: Box<(dyn Fn(Duration) + Send + Sync + 'static)>, period: Duration) -> FreeInterval {
    let mut stopped = Arc::new(RwLock::new(false));
    future::exec({
        let stopped = Arc::clone(&mut stopped);
        async move {
            let mut ticker = ticker(period);
            ticker.tick().await;
            loop {
                let delta = ticker.tick().await;
                if *stopped.read().unwrap() {
                    break;
                }
                callback(delta);
            }
        }
    });
    FreeInterval {
        stopped,
    }
}

/// An ticker that can be stopped at anytime, returned
/// from the [`free_animation_interval!`] and [`free_interval!`] macros.
/// 
/// To stop the ticker, call `ticker.stop`.
pub struct FreeInterval {
    stopped: Arc<RwLock<bool>>,
}

impl FreeInterval {
    pub fn stop(&self) {
        *self.stopped.write().unwrap() = true;
    }
}