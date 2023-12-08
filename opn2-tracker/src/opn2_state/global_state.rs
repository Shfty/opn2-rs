/*
use dirty_guard::DirtyGuard;

use crate::{
    register::{
        Channel3Mode, DacAmplitude, DacEnable, KeyOnOff, TimerA, TimerALsb, TimerAMsb, TimerB,
        Timers, TimersAndChannel3Mode, Lfo,
    },
};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct GlobalState {
    lfo: DirtyGuard<Lfo>,
    timer_a: DirtyGuard<TimerA>,
    timer_b: DirtyGuard<TimerB>,
    timers_and_channel_3_mode: DirtyGuard<TimersAndChannel3Mode>,
    key_on_off: DirtyGuard<KeyOnOff>,
    dac_enable: DirtyGuard<DacEnable>,
    dac_amplitude: DirtyGuard<DacAmplitude>,
}

// Setters
impl GlobalState {
    pub fn set_lfo<T>(&mut self, lfo: T)
    where
        T: Into<Lfo>,
    {
        self.lfo.write().set(lfo.into());
    }

    pub fn set_timer_a_msb<T>(&mut self, timer_a_msb: T)
    where
        T: Into<TimerAMsb>,
    {
        self.timer_a.write().set_msb(timer_a_msb);
    }

    pub fn set_timer_a_lsb<T>(&mut self, timer_a_lsb: T)
    where
        T: Into<TimerALsb>,
    {
        self.timer_a.write().set_lsb(timer_a_lsb);
    }

    pub fn set_timer_b<T>(&mut self, timer_b: T)
    where
        T: Into<TimerB>,
    {
        self.timer_b.write().set(timer_b.into());
    }

    pub fn set_timers<T>(&mut self, timers: T)
    where
        T: Into<Timers>,
    {
        self.timers_and_channel_3_mode.write().set_timers(timers);
    }

    pub fn set_channel_3_mode<T>(&mut self, channel_3_mode: T)
    where
        T: Into<Channel3Mode>,
    {
        self.timers_and_channel_3_mode
            .write()
            .set_channel_3_mode(channel_3_mode);
    }

    pub fn set_key_on_off<T>(&mut self, key_on_off: T)
    where
        T: Into<KeyOnOff>,
    {
        self.key_on_off.write().set(key_on_off.into());
    }

    pub fn set_dac_enable<T>(&mut self, dac_enable: T)
    where
        T: Into<DacEnable>,
    {
        self.dac_enable.write().set(dac_enable.into());
    }

    pub fn set_dac_amplitude<T>(&mut self, dac_amplitude: T)
    where
        T: Into<DacAmplitude>,
    {
        self.dac_amplitude.write().set(dac_amplitude.into());
    }
}

// Builder-style methods
#[allow(dead_code)]
impl GlobalState {
    pub fn with_lfo<T>(mut self, lfo: T) -> Self
    where
        T: Into<Lfo>,
    {
        self.set_lfo(lfo);
        self
    }

    pub fn with_timer_a_msb<T>(mut self, timer_a_msb: T) -> Self
    where
        T: Into<TimerAMsb>,
    {
        self.set_timer_a_msb(timer_a_msb);
        self
    }

    pub fn with_timer_a_lsb<T>(mut self, timer_a_lsb: T) -> Self
    where
        T: Into<TimerALsb>,
    {
        self.set_timer_a_lsb(timer_a_lsb);
        self
    }

    pub fn with_timer_b<T>(mut self, timer_b: T) -> Self
    where
        T: Into<TimerB>,
    {
        self.set_timer_b(timer_b);
        self
    }

    pub fn with_key_on_off<T>(mut self, key_on_off: T) -> Self
    where
        T: Into<KeyOnOff>,
    {
        self.set_key_on_off(key_on_off);
        self
    }

    pub fn with_dac_enable<T>(mut self, dac_enable: T) -> Self
    where
        T: Into<DacEnable>,
    {
        self.set_dac_enable(dac_enable);
        self
    }

    pub fn with_dac_amplitude<T>(mut self, dac_amplitude: T) -> Self
    where
        T: Into<DacAmplitude>,
    {
        self.set_dac_amplitude(dac_amplitude);
        self
    }

    pub fn with_timers<T>(mut self, timers: T) -> Self
    where
        T: Into<Timers>,
    {
        self.set_timers(timers);
        self
    }

    pub fn with_channel_3_mode<T>(mut self, channel_3_mode: T) -> Self
    where
        T: Into<Channel3Mode>,
    {
        self.set_channel_3_mode(channel_3_mode);
        self
    }
}

// Application
use crate::register::GlobalRegister;
impl GlobalState {
    pub fn apply(&mut self, opn2_sender: &OPN2CommandSender) -> Result<(), Opn2CommandError> {
        if let Some(lfo) = self.lfo.try_read() {
            lfo.apply(opn2_sender)?;
        }

        if let Some(timer_a) = self.timer_a.try_read() {
            timer_a.apply(opn2_sender)?;
        }

        if let Some(timer_b) = self.timer_b.try_read() {
            timer_b.apply(opn2_sender)?;
        }

        if let Some(timers_and_channel_3_mode) = self.timers_and_channel_3_mode.try_read() {
            timers_and_channel_3_mode.apply(opn2_sender)?;
        }

        if let Some(key_on_off) = self.key_on_off.try_read() {
            key_on_off.apply(opn2_sender)?;
        }

        if let Some(dac_enable) = self.dac_enable.try_read() {
            dac_enable.apply(opn2_sender)?;
        }

        if let Some(dac_amplitude) = self.dac_amplitude.try_read() {
            dac_amplitude.apply(opn2_sender)?;
        }

        Ok(())
    }
}
*/
