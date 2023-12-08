mod dac;
mod key_on_off;
mod lfo;
mod timer_a;
mod timer_b;
mod timers_and_channel_3_mode;

pub use dac::*;
pub use key_on_off::*;
pub use lfo::*;
pub use timer_a::*;
pub use timer_b::*;
pub use timers_and_channel_3_mode::*;

use super::Register;

pub trait GlobalRegister: Register {
    fn get_data(&self) -> u8;

    /*
    use super::Port;

    fn apply(&self, opn2_sender: &OPN2CommandSender) -> Result<(), Opn2CommandError> {
        opn2_sender.send(Port::Port1.into(), Self::BASE_ADDRESS, self.get_data())?;

        Ok(())
    }
    */
}
