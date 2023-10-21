/*!
When the Rialight runtime is targetting the browser.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}, fmt::Debug};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearTimeout(token: i32);
}

#[wasm_bindgen(module = "browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = waitInJSPromise)]
    fn wait_in_js_promise(ms: f64) -> js_sys::Promise;

    #[wasm_bindgen(js_name = nonAnimationTicker)]
    fn non_animation_interval(closure: &Closure<dyn FnMut(f64)>, ms: f64) -> web_sys::AbortController;
    #[wasm_bindgen(js_name = animationTicker)]
    fn animation_interval(closure: &Closure<dyn FnMut(f64)>, ms: f64) -> web_sys::AbortController;

    // JSTicker

    pub type JSTicker;

    #[wasm_bindgen(constructor)]
    fn new(for_animation: bool, ms: f64) -> JSTicker;

    #[wasm_bindgen(method, js_name = tickInJSPromise)]
    fn tick_in_js_promise(this: &JSTicker) -> js_sys::Promise;
}

impl Debug for JSTicker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("JSTicker()")
    }
}

impl JSTicker {
    async fn tick_in_future(&self) -> Duration {
        let delta = wasm_bindgen_futures::JsFuture::from(self.tick_in_js_promise()).await;
        Duration::from_millis(unsafe { delta.unwrap().as_f64().unwrap().to_int_unchecked::<u64>() })
    }
}

pub async fn wait(duration: Duration) {
    let ms: u32 = duration.as_millis().try_into().expect("Developer has given too large period for wait duration");
    wasm_bindgen_futures::JsFuture::from(wait_in_js_promise(ms.into())).await.unwrap();
}

pub async fn wait_until(instant: crate::timer::Instant) {
    wait(instant.since(crate::timer::Instant::now())).await;
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant {
    epoch_ms: u128,
}

impl Instant {
    pub fn since(&self, other: Instant) -> Duration {
        *self - other
    }

    pub fn now() -> Self {
        let epoch_ms: u64 = unsafe { js_sys::Date::now().to_int_unchecked() };
        Self {
            epoch_ms: epoch_ms.into(),
        }
    }

    pub fn try_add(&self, duration: Duration) -> Option<Instant> {
        Some(Instant { epoch_ms: self.epoch_ms.checked_add(duration.as_millis())? })
    }

    pub fn try_subtract(&self, duration: Duration) -> Option<Instant> {
        Some(Instant { epoch_ms: self.epoch_ms.checked_sub(duration.as_millis())? })
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Instant { epoch_ms: self.epoch_ms.checked_add(rhs.as_millis()).expect("Overflow when adding duration to instant") }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.epoch_ms = self.epoch_ms.checked_add(rhs.as_millis()).expect("Overflow when adding duration to instant");
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Instant { epoch_ms: self.epoch_ms.checked_sub(rhs.as_millis()).expect("Overflow when subtracting duration from instant") }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Self::Output {
        Duration::from_millis(if self.epoch_ms < rhs.epoch_ms { 0 } else { (self.epoch_ms - rhs.epoch_ms).try_into().unwrap_or(u64::MAX) })
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.epoch_ms = self.epoch_ms.checked_sub(rhs.as_millis()).expect("Overflow when subtracting duration from instant");
    }
}

/*
pub async fn timeout<F: Future<Output = ()> + Send + 'static>(duration: Duration, future: F) -> Result<(), crate::timer::ElapsedError> {
    let (_, i) = future::race([
        future,
        wait(duration),
    ]).await;

    match i {
        0 => Ok(()),
        1 => Err(crate::timer::ElapsedError),
    }
}
*/

#[derive(Debug)]
pub struct Ticker {
    pub for_animation: bool,
    pub period: Duration,
    pub start: crate::timer::Instant,
    pub ticker: Option<JSTicker>,
}

impl Ticker {
    pub async fn tick(&mut self) -> Duration {
        match self.ticker.as_ref() {
            Some(ticker) => ticker.tick_in_future().await,
            None => {
                // Initial tick
                wait_until(self.start).await;
                let ms: u32 = self.period.as_millis().try_into().expect("Developer has given too large period for interval");
                self.ticker = Some(JSTicker::new(self.for_animation, ms.into()));
                return Duration::from_millis(0);
            },
        }
    }
}