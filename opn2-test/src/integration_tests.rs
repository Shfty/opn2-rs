use opn2::Opn2Instruction;
use opn2_driver::Opn2Driver;
use opn2_nuked::Opn2 as Opn2Nuked;
use opn2_rs::{chips::Ym3438, opn2::Opn2 as Opn2Rs};
use vgm::VgmFile;

const SAMPLE_RATE: usize = 44100;
const SAMPLE_DURATION_SECS: usize = 2;

macro_rules! test_vgm {
    ($test_name:tt, $vgm_file:tt) => {
        #[test]
        fn $test_name() {
            let mut opn2 = Opn2Driver::<Opn2Rs<Ym3438>>::default();
            let mut nuked = Opn2Driver::<Opn2Nuked>::default();

            let vgm = VgmFile::parse($vgm_file).expect("Failed to parse VGM");
            let commands: Vec<Opn2Instruction> = vgm.into();
            opn2.extend(commands.clone());
            nuked.extend(commands);

            for _ in 0..SAMPLE_DURATION_SECS {
                let samples_opn2 = opn2.samples(SAMPLE_RATE);
                let samples_nuked = nuked.samples(SAMPLE_RATE);
                for (sample_opn2, sample_nuked) in samples_opn2.zip(samples_nuked) {
                    assert_eq!(sample_opn2, sample_nuked);
                }
            }
        }
    };
}

test_vgm!(
    cycle_accuracy_transparent_obstacle,
    "../vgm/data/transparent_obstacle.vgm"
);
test_vgm!(
    cycle_accuracy_evil_destroyer,
    "../vgm/data/evil_destroyer.vgm"
);
test_vgm!(
    cycle_accuracy_attack_sharply,
    "../vgm/data/attack_sharply.vgm"
);
test_vgm!(cycle_accuracy_simmer_down, "../vgm/data/simmer_down.vgm");
test_vgm!(cycle_accuracy_metal_squad, "../vgm/data/metal_squad.vgm");
