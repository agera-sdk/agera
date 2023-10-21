/*!
The Rialight runtime uses the asynchronous Tokio runtime internally
for any platform other than the browser.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};
use crate::common::*;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant(pub tokio::time::Instant);

impl Instant {
    pub fn since(&self, other: Instant) -> Duration {
        self.0.duration_since(other.0)
    }

    pub fn now() -> Instant {
        Self(tokio::time::Instant::now())
    }

    pub fn try_add(&self, duration: Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_add(duration)?))
    }

    pub fn try_subtract(&self, duration: Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_sub(duration)?))
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 = self.0 + rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Self::Output {
        self.0 - rhs.0
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 = self.0 - rhs;
    }
}

#[derive(Debug)]
pub struct Ticker(pub tokio::time::Interval);

impl Ticker {
    pub async fn tick(&mut self) -> Duration {
        future::no_send!();
        let last_tick_instant = tokio::time::Instant::now();
        self.0.tick().await;
        tokio::time::Instant::now() - last_tick_instant
    }
}

impl Drop for Ticker {
    fn drop(&mut self) {
    }
}