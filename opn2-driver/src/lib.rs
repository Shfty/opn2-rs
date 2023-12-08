//! Wrapper to clock an OPN2, sample / resample its output, and output an iterator of 16-bit stereo PCM data

use std::ops::{Deref, DerefMut};

use opn2::Opn2Instruction;
use opn2_trait::Opn2Trait;

pub const CLOCK_RATE: u32 = 7670453;
pub const SAMPLE_RATE: u32 = 44100;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Opn2Driver<T>
where
    T: Opn2Trait,
{
    chip: T,

    clock_rate: u32,
    sample_rate: u32,
    gain: i16,

    commands: Vec<Opn2Instruction>,
    play_head: usize,

    busy_clocks: u16,
    busy_samples: u16,

    sample_idx: usize,
    prev_sample_idx: usize,
}

impl<T> Default for Opn2Driver<T>
where
    T: Opn2Trait + Default,
{
    fn default() -> Self {
        Opn2Driver {
            chip: Default::default(),

            clock_rate: CLOCK_RATE,
            sample_rate: SAMPLE_RATE,
            gain: 20,

            commands: Default::default(),
            play_head: 0,

            busy_clocks: 0,
            busy_samples: 0,
            sample_idx: 0,
            prev_sample_idx: 0,
        }
    }
}

impl<T> Deref for Opn2Driver<T>
where
    T: Opn2Trait,
{
    type Target = Vec<Opn2Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}

impl<T> DerefMut for Opn2Driver<T>
where
    T: Opn2Trait,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

// Public API
impl<T> Opn2Driver<T>
where
    T: Opn2Trait + Default,
{
    /// Chip is running a VCLK / 144 = MCLK / 7 / 144
    pub const CLOCK_RATIO: usize = 144;
    pub const CLOCKS_PER_FM_CYCLE: usize = Self::CLOCK_RATIO / 6;

    pub fn new(commands: Vec<Opn2Instruction>) -> Self {
        Opn2Driver {
            commands,
            ..Default::default()
        }
    }

    pub fn clock_rate(&self) -> u32 {
        self.clock_rate
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn gain(&self) -> i16 {
        self.gain
    }

    pub fn play_head(&self) -> usize {
        self.play_head
    }

    pub fn set_clock_rate(&mut self, clock_rate: u32) {
        self.clock_rate = clock_rate
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate
    }

    pub fn set_gain(&mut self, gain: i16) {
        self.gain = gain
    }

    pub fn set_play_head(&mut self, play_head: usize) {
        self.play_head = play_head.clamp(0, self.commands.len() - 1)
    }

    pub fn is_playing(&mut self) -> bool {
        self.is_busy() || self.play_head < self.commands.len() - 1
    }

    pub fn samples(&mut self, count: usize) -> impl Iterator<Item = (i16, i16)> + '_ {
        let scale = self.scale();
        let count_scaled = (count as f32 * scale) as usize;
        let gain = self.gain;

        resample(
            std::iter::from_fn(move || Some(self.sample()))
                .take(count_scaled)
                .map(apply_gain(gain)),
            count,
        )
    }
}

// Private API
impl<T> Opn2Driver<T>
where
    T: Opn2Trait + Default,
{
    fn scale(&self) -> f32 {
        let clocks_per_sample = self.clock_rate() as f32 / self.sample_rate as f32;
        clocks_per_sample / Opn2Driver::<T>::CLOCK_RATIO as f32
    }

    fn sample(&mut self) -> (i16, i16) {
        let idx = (self.sample_idx as f32 / self.scale()) as usize;
        if idx != self.prev_sample_idx {
            self.advance_play_head();
            if self.busy_samples > 0 {
                self.busy_samples -= 1;
            }
        }
        self.prev_sample_idx = idx;
        self.sample_idx = self.sample_idx.wrapping_add(1);

        let mut out_sample = (0, 0);
        for _ in 0..Self::CLOCKS_PER_FM_CYCLE {
            let (l, r) = self.clock_chip();
            out_sample.0 += l;
            out_sample.1 += r;
        }
        out_sample
    }

    fn clock_chip(&mut self) -> (i16, i16) {
        if self.busy_clocks > 0 {
            self.busy_clocks -= 1;
        }

        self.chip.clock()
    }

    fn is_busy(&mut self) -> bool {
        let read = self.chip.read(4000);
        let busy = read >> 7 != 0;
        busy || self.busy_clocks > 0 || self.busy_samples > 0
    }

    fn advance_play_head(&mut self) {
        if self.commands.is_empty() {
            return;
        }

        if self.play_head == self.commands.len() - 1 {
            return;
        }

        if self.is_busy() {
            return;
        }

        match self.commands[self.play_head] {
            Opn2Instruction::SetClockRate(clock_rate) => self.clock_rate = clock_rate,
            Opn2Instruction::Write(port, data) => {
                self.chip.write(port, data);
                self.busy_clocks += 2;
            }
            Opn2Instruction::Wait(samples) => {
                let samples: u16 = samples.into();
                self.busy_samples += samples
            }
        }

        self.play_head += 1;
    }
}

/// Return a closure that multiplies the given 16-bit stereo PCM sample by a given gain factor
fn apply_gain(gain: i16) -> impl Fn((i16, i16)) -> (i16, i16) {
    move |(l, r): (i16, i16)| {
        let l = l.saturating_mul(gain);
        if l == i16::MIN || l == i16::MAX {
            eprintln!("Left channel clipping");
        }

        let r = r.saturating_mul(gain);
        if r == i16::MIN || r == i16::MAX {
            eprintln!("Right channel clipping");
        }

        (l, r)
    }
}

/// Resample an iterator of 16-bit stereo PCM data to a given size
fn resample(
    samples: impl Iterator<Item = (i16, i16)>,
    count: usize,
) -> impl Iterator<Item = (i16, i16)> {
    let opn2_samples = samples.collect::<Vec<_>>();

    (0..count).map(move |out_sample_idx| {
        // Calculate where this sample lies along the output waveform
        let total_progress = out_sample_idx as f32 / count as f32;

        // Map into the OPN2 waveform
        let opn2_sample_progress = total_progress * opn2_samples.len() as f32;
        let opn2_sample_idx = opn2_sample_progress as usize;
        let opn2_sample_substep = opn2_sample_progress % 1.0;

        // Fetch OPN2 sample
        let (mut l, mut r) = opn2_samples[opn2_sample_idx];

        // For intermediate values, interpolate toward the next sample
        if opn2_sample_substep > 0.0 && opn2_samples.len() > opn2_sample_idx + 1 {
            let (l1, r1) = opn2_samples[opn2_sample_idx + 1];
            let (dl, dr) = (l1 - l, r1 - r);
            l += (dl as f32 * opn2_sample_substep) as i16;
            r += (dr as f32 * opn2_sample_substep) as i16;
        }

        (l, r)
    })
}
