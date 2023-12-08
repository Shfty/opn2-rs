pub fn take_2<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> [u8; 2] {
    [
        *iter.next().expect("Failed to read byte 0"),
        *iter.next().expect("Failed to read byte 1"),
    ]
}

pub fn take_4<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> [u8; 4] {
    [
        *iter.next().expect("Failed to read byte 0"),
        *iter.next().expect("Failed to read byte 1"),
        *iter.next().expect("Failed to read byte 2"),
        *iter.next().expect("Failed to read byte 3"),
    ]
}

pub fn parse_ident<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> [char; 4] {
    let bytes = take_4(iter);
    [
        bytes[3] as char,
        bytes[2] as char,
        bytes[1] as char,
        bytes[0] as char,
    ]
}

pub fn parse_u8<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> u8 {
    *iter.next().expect("Failed to read ident byte 0")
}

pub fn shift_fold_u16(acc: u16, next: &u8) -> u16 {
    (acc << 8) | (*next as u16)
}

pub fn parse_u16<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> u16 {
    let bytes = take_2(iter);
    bytes.iter().rev().fold(0, shift_fold_u16)
}

pub fn shift_fold_u32(acc: u32, next: &u8) -> u32 {
    (acc << 8) | (*next as u32)
}

pub fn parse_u32<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> u32 {
    let bytes = take_4(iter);
    bytes.iter().rev().fold(0, shift_fold_u32)
}
