use std::error;

use crate::generator::Generator;
use crate::generator_type::GeneratorType;
use crate::zone::Zone;
use crate::sample_header::SampleHeader;

fn set_parameter(gs: &mut [i16; GeneratorType::COUNT], generator: &Generator)
{
    let index = generator.generator_type as usize;

    // Unknown generators should be ignored.
    if index < gs.len()
    {
        gs[index] = generator.value as i16;
    }
}

pub struct InstrumentRegion
{
    gs: [i16; GeneratorType::COUNT],
}

impl InstrumentRegion
{
    fn new(name: &String, global: &Zone, local: &Zone, samples: &Vec<SampleHeader>) -> Result<Self, Box<dyn error::Error>>
    {
        let mut gs: [i16; GeneratorType::COUNT] = [0; GeneratorType::COUNT];
        gs[GeneratorType::INITIAL_FILTER_CUTOFF_FREQUENCY as usize] = 13500;
        gs[GeneratorType::DELAY_MODULATION_LFO as usize] = -12000;
        gs[GeneratorType::DELAY_VIBRATO_LFO as usize] = -12000;
        gs[GeneratorType::DELAY_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::ATTACK_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::HOLD_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::DECAY_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::RELEASE_MODULATION_ENVELOPE as usize] = -12000;
        gs[GeneratorType::DELAY_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::ATTACK_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::HOLD_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::DECAY_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::RELEASE_VOLUME_ENVELOPE as usize] = -12000;
        gs[GeneratorType::KEY_RANGE as usize] = 0x7F00;
        gs[GeneratorType::VELOCITY_RANGE as usize] = 0x7F00;
        gs[GeneratorType::KEY_NUMBER as usize] = -1;
        gs[GeneratorType::VELOCITY as usize] = -1;
        gs[GeneratorType::SCALE_TUNING as usize] = 100;
        gs[GeneratorType::OVERRIDING_ROOT_KEY as usize] = -1;

        for generator in global.generators.iter()
        {
            set_parameter(&mut gs, generator);
        }

        for generator in local.generators.iter()
        {
            set_parameter(&mut gs, generator);
        }

        let id = gs[GeneratorType::SAMPLE_ID as usize] as usize;
        if id >= samples.len()
        {
            return Err(format!("The instrument '{name}' contains an invalid sample ID '{id}'.").into());
        }

        Ok(InstrumentRegion
        {
            gs: gs,
        })
    }

    pub(crate) fn create(name: &String, zones: &[Zone], samples: &Vec<SampleHeader>) -> Result<Vec<InstrumentRegion>, Box<dyn error::Error>>
    {
        // Is the first one the global zone?
        if zones[0].generators.len() == 0 || zones[0].generators.last().unwrap().generator_type != GeneratorType::SAMPLE_ID
        {
            // The first one is the global zone.
            let global = &zones[0];

            // The global zone is regarded as the base setting of subsequent zones.
            let count = zones.len() - 1;
            let mut regions: Vec<InstrumentRegion> = Vec::new();
            for i in 0..count
            {
                regions.push(InstrumentRegion::new(name, global, &zones[i + 1], &samples)?);
            }

            Ok(regions)
        }
        else
        {
            // No global zone.
            let count = zones.len();
            let mut regions: Vec<InstrumentRegion> = Vec::new();
            for i in 0..count
            {
                regions.push(InstrumentRegion::new(name, &Zone::empty(), &zones[i], &samples)?);
            }

            Ok(regions)
        }
    }
}
