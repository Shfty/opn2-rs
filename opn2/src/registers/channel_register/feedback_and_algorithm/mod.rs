mod algorithm;
mod feedback;

pub use algorithm::*;
pub use feedback::*;

use crate::registers::Register;

use super::ChannelRegister;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FeedbackAndAlgorithm {
    feedback: Feedback,
    algorithm: Algorithm,
}

impl FeedbackAndAlgorithm {
    pub fn new(feedback: Feedback, algorithm: Algorithm) -> Self {
        FeedbackAndAlgorithm {
            feedback,
            algorithm,
        }
    }
}

// Getters
impl FeedbackAndAlgorithm {
    pub fn get_feedback(&self) -> Feedback {
        self.feedback
    }

    pub fn get_algorithm(&self) -> Algorithm {
        self.algorithm
    }
}

// Setters
impl FeedbackAndAlgorithm {
    pub fn set_feedback<T>(&mut self, feedback: T)
    where
        T: Into<Feedback>,
    {
        self.feedback = feedback.into();
    }

    pub fn set_algorithm<T>(&mut self, algorithm: T)
    where
        T: Into<Algorithm>,
    {
        self.algorithm = algorithm.into()
    }
}

impl From<u8> for FeedbackAndAlgorithm {
    fn from(fb_al: u8) -> Self {
        let feedback = fb_al >> 3;
        let algorithm = fb_al & 0b111;
        FeedbackAndAlgorithm::new(feedback.into(), algorithm.into())
    }
}

impl From<FeedbackAndAlgorithm> for u8 {
    fn from(val: FeedbackAndAlgorithm) -> Self {
        let feedback: u8 = val.feedback.into();
        let algorithm: u8 = val.algorithm.into();
        (feedback << 3) | algorithm
    }
}

impl Register for FeedbackAndAlgorithm {
    const BASE_ADDRESS: u8 = 0xB0;
}

impl ChannelRegister for FeedbackAndAlgorithm {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
