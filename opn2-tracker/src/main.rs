//! Legacy attempt at converting OPN2 VGM data into a manipulable AST.

mod opn2_state;
mod opn2_track;

pub use opn2_state::*;
pub use opn2_track::*;

/*
use opn2::{Channel, Opn2Command};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Notes {
    C = 644,
    Cs = 681,
    D = 722,
    Ds = 765,
    E = 810,
    F = 858,
    Fs = 910,
    G = 964,
    Gs = 1021,
    A = 1081,
    As = 1146,
    B = 1214,
}

const MAJOR_NOTES: [Notes; 7] = [
    Notes::C,
    Notes::D,
    Notes::E,
    Notes::F,
    Notes::G,
    Notes::A,
    Notes::B,
];

const INTO_SANDYS_CITY: [Notes; 16] = [
    Notes::D,
    Notes::F,
    Notes::F,
    Notes::D,
    Notes::As,
    Notes::As,
    Notes::D,
    Notes::A,
    Notes::A,
    Notes::D,
    Notes::G,
    Notes::G,
    Notes::F,
    Notes::G,
    Notes::F,
    Notes::E,
];

pub fn play_into_sandys_city(
    opn2_sender: &OPN2CommandSender,
    channel: Channel,
    channel_state: ChannelState,
) -> Result<(), Box<dyn Error>> {
    KeyOnOff::new(Channel1, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel2, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel3, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel4, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel5, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel6, Operators::OPERATOR_NONE).apply(opn2_sender)?;

    Opn2State::default()
        .with_channel(Channel1, channel_state)
        .apply(opn2_sender)?;

    let octave_notes = [2, 2, 3, 3].iter().flat_map(|octave| {
        INTO_SANDYS_CITY
            .iter()
            .map(move |note| Frequency::from((octave << 11) | *note as u16))
    });

    for frequency in octave_notes {
        frequency.apply(opn2_sender, channel)?;
        KeyOnOff::new(channel, Operators::OPERATOR_ALL).apply(opn2_sender)?;
        std::thread::sleep(Duration::from_millis(200));
        KeyOnOff::new(channel, Operators::OPERATOR_NONE).apply(opn2_sender)?;
        std::thread::sleep(Duration::from_millis(100));
    }

    KeyOnOff::new(Channel1, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel2, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel3, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel4, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel5, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel6, Operators::OPERATOR_NONE).apply(opn2_sender)?;

    Ok(())
}

pub fn play_test_program(
    opn2_sender: &OPN2CommandSender,
    channel: Channel,
    mut channel_state: ChannelState,
) -> Result<(), Box<dyn Error>> {
    KeyOnOff::new(Channel1, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel2, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel3, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel4, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel5, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel6, Operators::OPERATOR_NONE).apply(opn2_sender)?;

    channel_state.apply(opn2_sender, channel)?;

    let octave_notes = [0, 1, 2, 3, 4, 5, 6].iter().flat_map(|octave| {
        MAJOR_NOTES
            .iter()
            .map(move |note| Frequency::from((octave << 11) | *note as u16))
    });

    for frequency in octave_notes {
        frequency.apply(opn2_sender, channel)?;
        KeyOnOff::new(channel, Operators::OPERATOR_ALL).apply(opn2_sender)?;
        std::thread::sleep(Duration::from_millis(10));
        KeyOnOff::new(channel, Operators::OPERATOR_NONE).apply(opn2_sender)?;
        std::thread::sleep(Duration::from_millis(5));
    }

    KeyOnOff::new(Channel1, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel2, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel3, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel4, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel5, Operators::OPERATOR_NONE).apply(opn2_sender)?;
    KeyOnOff::new(Channel6, Operators::OPERATOR_NONE).apply(opn2_sender)?;

    Ok(())
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ChannelVoices {
    pub channel_1: Vec<ChannelState>,
    pub channel_2: Vec<ChannelState>,
    pub channel_3: Vec<ChannelState>,
    pub channel_4: Vec<ChannelState>,
    pub channel_5: Vec<ChannelState>,
    pub channel_6: Vec<ChannelState>,
}

fn extract_voices() -> Result<ChannelVoices, Box<dyn Error>> {
    let commands = VgmFile::parse(TEST_VGM)?
        .into_iter()
        .filter(|command| match command {
            VgmCommand::WriteYm2612(_, _, _) => true,
            VgmCommand::GameGearPsgStereo(_) => false,
            VgmCommand::WritePsg(_) => false,
            VgmCommand::Wait(_) => true,
        })
        .flat_map(TryInto::<Opn2Command>::try_into);

    let mut channel_1_voices: Vec<ChannelState> = vec![];
    let mut channel_2_voices: Vec<ChannelState> = vec![];
    let mut channel_3_voices: Vec<ChannelState> = vec![];
    let mut channel_4_voices: Vec<ChannelState> = vec![];
    let mut channel_5_voices: Vec<ChannelState> = vec![];
    let mut channel_6_voices: Vec<ChannelState> = vec![];

    let mut state = Opn2State::default();

    for command in commands {
        if let Opn2Command::KeyOnOff(key_on_off) = command {
            if key_on_off.get_operators() == Operators::OPERATOR_NONE {
                match key_on_off.get_channel() {
                    Channel::Channel1 => {
                        channel_1_voices.push(*state.get_channel(Channel::Channel1))
                    }
                    Channel::Channel2 => {
                        channel_2_voices.push(*state.get_channel(Channel::Channel2))
                    }
                    Channel::Channel3 => {
                        channel_3_voices.push(*state.get_channel(Channel::Channel3))
                    }
                    Channel::Channel4 => {
                        channel_4_voices.push(*state.get_channel(Channel::Channel4))
                    }
                    Channel::Channel5 => {
                        channel_5_voices.push(*state.get_channel(Channel::Channel5))
                    }
                    Channel::Channel6 => {
                        channel_6_voices.push(*state.get_channel(Channel::Channel6))
                    }
                }
            }
        } else {
            state.process(command);
        }
    }

    let unique_voice = |lhs: &mut ChannelState, rhs: &mut ChannelState| {
        lhs.get_feedback() == rhs.get_feedback()
            && lhs.get_algorithm() == rhs.get_algorithm()
            && lhs.get_operator(Operator::Operator1) == rhs.get_operator(Operator::Operator1)
            && lhs.get_operator(Operator::Operator2) == rhs.get_operator(Operator::Operator2)
            && lhs.get_operator(Operator::Operator3) == rhs.get_operator(Operator::Operator3)
            && lhs.get_operator(Operator::Operator4) == rhs.get_operator(Operator::Operator4)
    };

    let has_total_level = |state: &ChannelState| {
        state
            .get_operator(Operator::Operator1)
            .get_total_level()
            .ne(&127u8.into())
    };

    let process = |mut channel_voices: Vec<ChannelState>| {
        channel_voices = channel_voices.into_iter().filter(has_total_level).collect();
        channel_voices.sort_unstable();
        channel_voices.dedup_by(unique_voice);
        channel_voices
    };

    let channel_1 = process(channel_1_voices);
    let channel_2 = process(channel_2_voices);
    let channel_3 = process(channel_3_voices);
    let channel_4 = process(channel_4_voices);
    let channel_5 = process(channel_5_voices);
    let channel_6 = process(channel_6_voices);

    Ok(ChannelVoices {
        channel_1,
        channel_2,
        channel_3,
        channel_4,
        channel_5,
        channel_6,
    })
}

fn extract_channel(commands: &[Opn2Command], channel: Channel) -> Vec<Opn2Command> {
    let channel_commands = commands
        .iter()
        .filter(|command| match command.get_channel() {
            Some(c) => channel == c,
            None => true,
        });

    let mut commands: Vec<Opn2Command> = vec![];
    for command in channel_commands.into_iter() {
        match command {
            Opn2Command::Wait(duration) => {
                if let Opn2Command::Wait(existing_duration) = commands.last_mut().unwrap() {
                    *existing_duration += *duration
                } else {
                    commands.push(*command);
                }
            }
            _ => commands.push(*command),
        }
    }

    commands
}

fn play_converted_vgm(opn2_stream: &OPN2Stream) -> Result<(), Box<dyn Error>> {
    println!("Loading {}\n", TEST_VGM);
    let vgm = VGMFile::parse(TEST_VGM)?;
    println!("{}", vgm.gd3);

    println!("Converting to OPN2Command...\n");
    let commands: Vec<OPN2Command> = vec![OPN2Command::SetClockRate(vgm.header.ym2612_clock)]
        .into_iter()
        .chain(
            vgm.into_iter()
                .filter(|command| match command {
                    VGMCommand::WriteYM2612(_, _, _) => true,
                    VGMCommand::GameGearPSGStereo(_) => false,
                    VGMCommand::WritePSG(_) => false,
                    VGMCommand::Wait(_) => true,
                })
                .map(Into::<OPN2Command>::into),
        )
        .collect();

    println!("Extracting channels...\n");
    let channel_1 = extract_channel(&commands, Channel::Channel1);
    let channel_2 = extract_channel(&commands, Channel::Channel2);
    let channel_3 = extract_channel(&commands, Channel::Channel3);
    let channel_4 = extract_channel(&commands, Channel::Channel4);
    let channel_5 = extract_channel(&commands, Channel::Channel5);
    let channel_6 = extract_channel(&commands, Channel::Channel6);

    println!("Creating channel tracks...\n");
    let track_1 = OPN2Track::from(channel_1);
    let track_2 = OPN2Track::from(channel_2);
    let track_3 = OPN2Track::from(channel_3);
    let track_4 = OPN2Track::from(channel_4);
    let track_5 = OPN2Track::from(channel_5);
    let track_6 = OPN2Track::from(channel_6);

    println!("Composing master track...\n");
    let master_track = OPN2Track::default()
        .append(track_1)
        .append(track_2)
        .append(track_3)
        .append(track_4)
        .append(track_5)
        .append(track_6);

    println!("Converting to RawOPN2Command...\n");
    let commands: Vec<RawOPN2Command> = master_track.into();

    println!("Buffering...\n");
    for command in commands {
        opn2_stream.get_command_tx().send_raw(command)?;
    }

    opn2_stream.play().expect("Failed to play stream");
    loop {
        match opn2_stream.get_event_rx().recv() {
            Ok(event) => match event {
                opn2::OPN2Event::PlaybackStart => println!("Playing...\n"),
                opn2::OPN2Event::PlaybackFinish => break,
                _ => (),
            },
            Err(err) => panic::panic_any(err),
        }
    }

    println!("Playback complete.\n");

    Ok(())
}

fn play_vgm(opn2_stream: &OPN2Stream) -> Result<(), Box<dyn Error>> {
    println!("Loading {}", TEST_VGM);
    let vgm = VGMFile::parse(TEST_VGM)?;
    println!("{}", vgm.gd3);

    let commands: Vec<RawOPN2Command> = vec![];

    for command in vgm.into_iter() {
        match command {
            vgm::VGMCommand::WriteYM2612(port, address, data) => {
                opn2_stream.get_command_tx().send(port, address, data)
            }
            vgm::VGMCommand::GameGearPSGStereo(_) => Ok(()),
            vgm::VGMCommand::WritePSG(_) => Ok(()),
            vgm::VGMCommand::Wait(samples) => opn2_stream.get_command_tx().wait(samples),
        }?;
    }

    opn2_stream.play().expect("Failed to play stream");
    loop {
        std::thread::sleep(Duration::from_secs_f64(1.0));
    }
}

use opn2::grand_piano;
pub fn test_grand_piano_voice(opn2_sender: &OPN2CommandSender) -> Result<(), Box<dyn Error>> {
    let grand_piano = grand_piano();
    println!("Testing Grand Piano Voice: {:#?}", grand_piano);
    play_into_sandys_city(opn2_sender, Channel::Channel1, grand_piano)?;
    Ok(())
}

pub fn test_extracted_voices(opn2_sender: &OPN2CommandSender) -> Result<(), Box<dyn Error>> {
    let ChannelVoices {
        channel_1,
        channel_2,
        channel_3,
        channel_4,
        channel_5,
        channel_6,
    } = extract_voices()?;

    for (i, voice) in channel_1.into_iter().enumerate() {
        println!("Testing Channel 1 Voice {}: {:#?}", i, voice);
        play_test_program(&opn2_sender, Channel::Channel1, voice)?;
    }

    for (i, voice) in channel_2.into_iter().enumerate() {
        println!("Testing Channel 2 Voice {}: {:#?}", i, voice);
        play_test_program(&opn2_sender, Channel::Channel2, voice)?;
    }

    for (i, voice) in channel_3.into_iter().enumerate() {
        println!("Testing Channel 3 Voice {}: {:#?}", i, voice);
        play_test_program(&opn2_sender, Channel::Channel3, voice)?;
    }

    for (i, voice) in channel_4.into_iter().enumerate() {
        println!("Testing Channel 4 Voice {}: {:#?}", i, voice);
        play_test_program(&opn2_sender, Channel::Channel4, voice)?;
    }

    for (i, voice) in channel_5.into_iter().enumerate() {
        println!("Testing Channel 5 Voice {}: {:#?}", i, voice);
        play_test_program(&opn2_sender, Channel::Channel5, voice)?;
    }

    for (i, voice) in channel_6.into_iter().enumerate() {
        println!("Testing Channel 6 Voice {}: {:#?}", i, voice);
        play_test_program(&opn2_sender, Channel::Channel6, voice)?;
    }

    Ok(())
}
*/

fn main() {

}