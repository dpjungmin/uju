use std::time::Duration;

/// Tracks when a given period has elapsed.
pub struct PeriodicTimer {
    period: Duration,
    accumulated: Duration,
    triggered_this_cycle: bool,
}

impl PeriodicTimer {
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            accumulated: Duration::ZERO,
            triggered_this_cycle: false,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.accumulated += dt;

        if self.accumulated >= self.period {
            self.accumulated -= self.period;

            if self.accumulated >= self.period {
                self.accumulated = Duration::ZERO;
            }

            self.triggered_this_cycle = true;
        } else {
            self.triggered_this_cycle = false;
        }
    }

    pub fn triggered(&self) -> bool {
        self.triggered_this_cycle
    }
}
