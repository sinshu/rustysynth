#![allow(dead_code)]

#[non_exhaustive]
pub(crate) struct Reverb {
    cfs_l: Vec<CombFilter>,
    cfs_r: Vec<CombFilter>,
    apfs_l: Vec<AllPassFilter>,
    apfs_r: Vec<AllPassFilter>,

    gain: f32,
    room_size: f32,
    room_size1: f32,
    damp: f32,
    damp1: f32,
    wet: f32,
    wet1: f32,
    wet2: f32,
    width: f32,
}

impl Reverb {
    const FIXED_GAIN: f32 = 0.015;
    const SCALE_WET: f32 = 3.0;
    const SCALE_DAMP: f32 = 0.4;
    const SCALE_ROOM: f32 = 0.28;
    const OFFSET_ROOM: f32 = 0.7;
    const INITIAL_ROOM: f32 = 0.5;
    const INITIAL_DAMP: f32 = 0.5;
    const INITIAL_WET: f32 = 1.0 / Reverb::SCALE_WET;
    const INITIAL_WIDTH: f32 = 1.0;
    const STEREO_SPREAD: usize = 23;

    const CF_TUNING_L1: usize = 1116;
    const CF_TUNING_R1: usize = 1116 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L2: usize = 1188;
    const CF_TUNING_R2: usize = 1188 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L3: usize = 1277;
    const CF_TUNING_R3: usize = 1277 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L4: usize = 1356;
    const CF_TUNING_R4: usize = 1356 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L5: usize = 1422;
    const CF_TUNING_R5: usize = 1422 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L6: usize = 1491;
    const CF_TUNING_R6: usize = 1491 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L7: usize = 1557;
    const CF_TUNING_R7: usize = 1557 + Reverb::STEREO_SPREAD;
    const CF_TUNING_L8: usize = 1617;
    const CF_TUNING_R8: usize = 1617 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L1: usize = 556;
    const APF_TUNING_R1: usize = 556 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L2: usize = 441;
    const APF_TUNING_R2: usize = 441 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L3: usize = 341;
    const APF_TUNING_R3: usize = 341 + Reverb::STEREO_SPREAD;
    const APF_TUNING_L4: usize = 225;
    const APF_TUNING_R4: usize = 225 + Reverb::STEREO_SPREAD;

    pub(crate) fn new(sample_rate: i32) -> Self {
        let mut cfs_l: Vec<CombFilter> = Vec::new();
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L1));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L2));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L3));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L4));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L5));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L6));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L7));
        cfs_l.push(CombFilter::new(Reverb::CF_TUNING_L8));

        let mut cfs_r: Vec<CombFilter> = Vec::new();
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R1));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R2));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R3));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R4));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R5));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R6));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R7));
        cfs_r.push(CombFilter::new(Reverb::CF_TUNING_R8));

        let mut apfs_l : Vec<AllPassFilter> = Vec::new();
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L1));
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L2));
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L3));
        apfs_l.push(AllPassFilter::new(Reverb::APF_TUNING_L4));

        let mut apfs_r : Vec<AllPassFilter> = Vec::new();
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R1));
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R2));
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R3));
        apfs_r.push(AllPassFilter::new(Reverb::APF_TUNING_R4));

        let gain: f32 = 0.0;
        let room_size: f32 = Reverb::INITIAL_ROOM;
        let room_size1: f32 = 0.0;
        let damp: f32 = Reverb::INITIAL_DAMP;
        let damp1: f32 = 0.0;
        let wet: f32 = Reverb::INITIAL_WET;
        let wet1: f32 = 0.0;
        let wet2: f32 = 0.0;
        let width: f32 = Reverb::INITIAL_WIDTH;

        Self {
            cfs_l: cfs_l,
            cfs_r: cfs_r,
            apfs_l: apfs_l,
            apfs_r: apfs_r,
            gain: gain,
            room_size: room_size,
            room_size1: room_size1,
            damp: damp,
            damp1: damp1,
            wet: wet,
            wet1: wet1,
            wet2: wet2,
            width: width
        }
    }
}

#[non_exhaustive]
struct CombFilter {
    
}

impl CombFilter {
    fn new(buffer_size: usize) -> Self {
        Self {
        }
    }
}

#[non_exhaustive]
struct AllPassFilter {
    
}

impl AllPassFilter {
    fn new(buffer_size: usize) -> Self {
        Self {
        }
    }
}
