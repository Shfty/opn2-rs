use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WaitSamples(pub u16);

impl Deref for WaitSamples {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WaitSamples {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<WaitSamples> for u16 {
    fn from(wait_samples: WaitSamples) -> Self {
        wait_samples.0
    }
}

impl From<u16> for WaitSamples {
    fn from(samples: u16) -> Self {
        WaitSamples(samples)
    }
}

impl From<Duration> for WaitSamples {
    fn from(duration: Duration) -> Self {
        let secs = duration.as_secs_f64();
        let samples = (secs * 44100.0) as u16;
        WaitSamples(samples)
    }
}

impl From<WaitSamples> for Duration {
    fn from(wait_samples: WaitSamples) -> Self {
        Duration::from_secs_f64(wait_samples.0 as f64 / 44100.0)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::WaitSamples;

    #[test]
    fn from_duration() {
        let seconds = Duration::from_secs(1);
        let wait_samples = WaitSamples::from(seconds);
        println!("{:?}", wait_samples);
    }

    #[test]
    fn into_duration() {
        let wait_samples = WaitSamples::from(44100);
        let seconds: Duration = wait_samples.into();
        println!("{:?}", seconds);
    }
}
