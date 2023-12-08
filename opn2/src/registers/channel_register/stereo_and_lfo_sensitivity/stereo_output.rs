#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StereoOutput {
    left: bool,
    right: bool,
}

impl StereoOutput {
    pub fn new(left: bool, right: bool) -> Self {
        StereoOutput { left, right }
    }

    pub fn get(&self) -> (bool, bool) {
        (self.left, self.right)
    }
}

impl Default for StereoOutput {
    fn default() -> Self {
        StereoOutput {
            left: true,
            right: true,
        }
    }
}
