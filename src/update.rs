use instant::Instant;
use std::time::Duration;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateRate {
    Interval(Duration),
    PerSecond(u32),
    PerMinute(u32),
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateLoop {
    interval: Duration,
    previous: Instant,
    lag: Duration,
}

//

impl Default for UpdateRate {
    fn default() -> Self {
        Self::PerSecond(60)
    }
}

impl UpdateRate {
    pub fn to_interval(self) -> Duration {
        match self {
            UpdateRate::Interval(interval) => interval,
            UpdateRate::PerSecond(count) => Duration::from_secs(1) / count,
            UpdateRate::PerMinute(count) => Duration::from_secs(1) * 60 / count,
        }
    }
}

impl Default for UpdateLoop {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl UpdateLoop {
    pub fn new(rate: UpdateRate) -> Self {
        Self {
            interval: rate.to_interval(),
            previous: Instant::now(),
            lag: Duration::from_secs_f64(0.0),
        }
    }

    /// May update multiple times or not at all
    /// to keep up with the requested update rate
    ///
    /// Returned `delta` is used to smooth animations
    /// between updates. It is a value in range
    /// `0.0..1.0`. Close to `0.0` means that an
    /// update happened just recently. Close to
    /// `1.0` means that an update will happen
    /// soon.
    ///
    /// Example usage with `delta`:
    /// ```ignore
    /// fn update(&mut self) {
    ///     self.position += self.velocity;
    /// }
    ///
    /// fn draw(&mut self, delta: f32) {
    ///     let position = self.position + self.velocity * delta;
    ///     draw_quad(position);
    /// }
    /// ```
    #[inline]
    pub fn update<F>(&mut self, mut f: F) -> f32
    where
        F: FnMut(),
    {
        // main game loop source:
        //  - https://gameprogrammingpatterns.com/game-loop.html
        let elapsed = self.previous.elapsed();
        self.previous = Instant::now();
        self.lag += elapsed;

        // updates
        while self.lag >= self.interval {
            f();
            self.lag -= self.interval;
        }

        self.delta()
    }

    #[inline]
    pub fn delta(&self) -> f32 {
        self.lag.as_secs_f32() / self.interval.as_secs_f32()
    }

    #[inline]
    pub fn will_update(&self) -> bool {
        self.lag + self.previous.elapsed() >= self.interval
    }
}
