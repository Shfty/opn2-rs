#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum AmplitudeModulationSensitivity {
    Db0 = 0,
    Db1_4 = 1,
    Db5_9 = 2,
    Db11_8 = 3,
}

impl From<u8> for AmplitudeModulationSensitivity {
    fn from(ams: u8) -> Self {
        match ams {
            0 => AmplitudeModulationSensitivity::Db0,
            1 => AmplitudeModulationSensitivity::Db1_4,
            2 => AmplitudeModulationSensitivity::Db5_9,
            3 => AmplitudeModulationSensitivity::Db11_8,
            _ => panic!("Invalid amplitude modulation sensitivity"),
        }
    }
}

impl Default for AmplitudeModulationSensitivity {
    fn default() -> Self {
        AmplitudeModulationSensitivity::Db0
    }
}
