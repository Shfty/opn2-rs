use std::fmt::Display;

use crate::{parse_ident, parse_u16, parse_u32};

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Gd3 {
    pub version: u32,
    pub track_name_en: String,
    pub track_name_jp: String,
    pub game_name_en: String,
    pub game_name_jp: String,
    pub system_name_en: String,
    pub system_name_jp: String,
    pub track_author_en: String,
    pub track_author_jp: String,
    pub game_release_date: String,
    pub vgm_author: String,
    pub notes: String,
}

impl Display for Gd3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Track Title:\t{}\n", self.track_name_en))?;
        f.write_fmt(format_args!("Game Name:\t{}\n", self.game_name_en))?;
        f.write_fmt(format_args!("System:\t\t{}\n", self.system_name_en))?;
        f.write_fmt(format_args!("Composer:\t{}\n", self.track_author_en))?;
        f.write_fmt(format_args!("Release:\t{}\n", self.game_release_date))?;
        f.write_fmt(format_args!("Version:\t0x{:03x}\n", self.version))?;
        f.write_fmt(format_args!("VGM by:\t\t{}\n", self.vgm_author))?;
        f.write_fmt(format_args!("Notes:\t{}\n", self.notes))?;
        Ok(())
    }
}

impl From<&[u8]> for Gd3 {
    fn from(bytes: &[u8]) -> Self {
        let mut iter = bytes.iter();

        fn parse_string<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> String {
            let mut bytes: Vec<u16> = vec![];
            loop {
                let wchar = parse_u16(iter);
                if wchar == 0 {
                    break;
                }
                else {
                    bytes.push(wchar);
                }
            }

            std::char::decode_utf16(bytes).flatten().collect()
        }

        let iter = &mut iter;

        let _ident = parse_ident(iter);
        let version = parse_u32(iter);
        let _length = parse_u32(iter);

        Gd3 {
            version,
            track_name_en: parse_string(iter),
            track_name_jp: parse_string(iter),
            game_name_en: parse_string(iter),
            game_name_jp: parse_string(iter),
            system_name_en: parse_string(iter),
            system_name_jp: parse_string(iter),
            track_author_en: parse_string(iter),
            track_author_jp: parse_string(iter),
            game_release_date: parse_string(iter),
            vgm_author: parse_string(iter),
            notes: parse_string(iter),
        }
    }
}
