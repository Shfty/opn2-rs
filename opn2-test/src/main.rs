//! Test program for playing back OPN2 VGM via an [`OPN2Driver`].
//! Also includes integration tests for verifying cycle accuracy against [`opn2-nuked`].

#[cfg(test)]
mod integration_tests;

use std::{
    error::Error,
    sync::{Arc, RwLock},
    time::Duration,
};

use opn2::Opn2Instruction;
use opn2_cpal::opn2_stream;
use opn2_driver::Opn2Driver;
use vgm::VgmFile;

use cpal::traits::StreamTrait;

use opn2_rs::{chips::Ym3438, opn2::Opn2 as Opn2Rs};

//const TEST_VGM: &str = "vgm/data/transparent_obstacle.vgm";
//const TEST_VGM: &str = "vgm/data/lightning_strikes_again.vgm";
//const TEST_VGM: &str = "vgm/data/evil_destroyer.vgm";
const TEST_VGM: &str = "vgm/data/attack_sharply.vgm";
//const TEST_VGM: &str = "vgm/data/simmer_down.vgm";
//const TEST_VGM: &str = "vgm/data/metal_squad.vgm";

//const TEST_VGM: &str = "vgm/data/laughter.vgm";
//const TEST_VGM: &str = "vgm/data/ooze.vgm";
//const TEST_VGM: &str = "vgm/data/emerald_hill_zone.vgm";

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Loading {}", TEST_VGM);
    let vgm = VgmFile::parse(TEST_VGM).expect("Failed to parse VGM");
    println!("{}", vgm.gd3);

    let clock_rate = vgm.header.ym2612_clock;
    let commands: Vec<Opn2Instruction> = vgm.into();

    let mut opn2 = Opn2Driver::<Opn2Rs<Ym3438>>::default();
    opn2.set_clock_rate(clock_rate);
    opn2.extend(commands);

    let opn2 = Arc::new(RwLock::new(opn2));
    let stream = opn2_stream(opn2.clone())?;

    cpal::traits::StreamTrait::play(&stream).expect("Failed to play stream");
    loop {
        if !opn2.write().unwrap().is_playing() {
            //break;
        }
        std::thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
    }

    Ok(())
}
