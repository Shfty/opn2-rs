use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    time::Duration,
};

use opn2::{Opn2Command, Opn2Instruction};

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Opn2Track(BTreeMap<Duration, Vec<Opn2Command>>);

impl Opn2Track {
    pub fn append(mut self, other: Opn2Track) -> Opn2Track {
        let other: BTreeMap<Duration, Vec<Opn2Command>> = other.into();
        for (timestamp, mut commands) in other.into_iter() {
            self.entry(timestamp).or_default().append(&mut commands)
        }
        self
    }
}

impl From<Opn2Track> for BTreeMap<Duration, Vec<Opn2Command>> {
    fn from(track: Opn2Track) -> Self {
        track.0
    }
}

impl From<BTreeMap<Duration, Vec<Opn2Command>>> for Opn2Track {
    fn from(track: BTreeMap<Duration, Vec<Opn2Command>>) -> Self {
        Opn2Track(track)
    }
}

impl Deref for Opn2Track {
    type Target = BTreeMap<Duration, Vec<Opn2Command>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Opn2Track {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Opn2Command>> for Opn2Track {
    fn from(commands: Vec<Opn2Command>) -> Self {
        let mut timestamp = Duration::new(0, 0);
        let mut timeline: BTreeMap<Duration, Vec<Opn2Command>> = BTreeMap::new();
        for command in commands {
            match command {
                Opn2Command::Wait(duration) => timestamp += duration,
                command => timeline.entry(timestamp).or_default().push(command),
            }
        }
        Opn2Track(timeline)
    }
}

impl From<Opn2Track> for Vec<Opn2Instruction> {
    fn from(val: Opn2Track) -> Self {
        let mut timestamp: Duration = Duration::new(0, 0);
        let mut commands: Vec<Opn2Instruction> = vec![];

        for (ts, cs) in val.0 {
            let mut cs: Vec<Opn2Instruction> = cs
                .into_iter()
                .flat_map(Into::<Vec<Opn2Instruction>>::into)
                .collect();

            commands.append(&mut cs);

            let delta = ts - timestamp;
            commands.push(Opn2Instruction::Wait(delta.into()));
            timestamp += delta;
        }

        commands
    }
}
