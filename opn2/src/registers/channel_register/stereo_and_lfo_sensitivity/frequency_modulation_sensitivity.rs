#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum FrequencyModulationSensitivity {
    Ht0 = 0,
    Ht3_4 = 1,
    Ht6_7 = 2,
    Ht10 = 3,
    Ht14 = 4,
    Ht20 = 5,
    Ht40 = 6,
    Ht80 = 7,
}

impl From<u8> for FrequencyModulationSensitivity {
    fn from(ams: u8) -> Self {
        match ams {
            0 => FrequencyModulationSensitivity::Ht0,
            1 => FrequencyModulationSensitivity::Ht3_4,
            2 => FrequencyModulationSensitivity::Ht6_7,
            3 => FrequencyModulationSensitivity::Ht10,
            4 => FrequencyModulationSensitivity::Ht14,
            5 => FrequencyModulationSensitivity::Ht20,
            6 => FrequencyModulationSensitivity::Ht40,
            7 => FrequencyModulationSensitivity::Ht80,
            _ => panic!("Invalid frequency modulation sensitivity"),
        }
    }
}

impl Default for FrequencyModulationSensitivity {
    fn default() -> Self {
        FrequencyModulationSensitivity::Ht0
    }
}
