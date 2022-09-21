use std::error;

use crate::generator::Generator;
use crate::generator_type::GeneratorType;
use crate::zone::Zone;
use crate::instrument::Instrument;

fn set_parameter(gs: &mut [i16; GeneratorType::COUNT], generator: &Generator)
{
    let index = generator.generator_type as usize;

    // Unknown generators should be ignored.
    if index < gs.len()
    {
        gs[index] = generator.value as i16;
    }
}

pub struct PresetRegion
{
    gs: [i16; GeneratorType::COUNT],
}

impl PresetRegion
{
    fn new(name: &String, global: &Zone, local: &Zone, samples: &Vec<Instrument>) -> Result<Self, Box<dyn error::Error>>
    {
        let mut gs: [i16; GeneratorType::COUNT] = [0; GeneratorType::COUNT];
        gs[GeneratorType::KEY_RANGE as usize] = 0x7F00;
        gs[GeneratorType::VELOCITY_RANGE as usize] = 0x7F00;

        for generator in global.generators.iter()
        {
            set_parameter(&mut gs, generator);
        }

        for generator in local.generators.iter()
        {
            set_parameter(&mut gs, generator);
        }

        let id = gs[GeneratorType::INSTRUMENT as usize] as usize;
        if id >= samples.len()
        {
            return Err(format!("The preset '{name}' contains an invalid instrument ID '{id}'.").into());
        }

        Ok(PresetRegion
        {
            gs: gs,
        })
    }

    pub(crate) fn create(name: &String, zones: &[Zone], instruments: &Vec<Instrument>) -> Result<Vec<PresetRegion>, Box<dyn error::Error>>
    {
        // Is the first one the global zone?
        if zones[0].generators.len() == 0 || zones[0].generators.last().unwrap().generator_type != GeneratorType::SAMPLE_ID
        {
            // The first one is the global zone.
            let global = &zones[0];

            // The global zone is regarded as the base setting of subsequent zones.
            let count = zones.len() - 1;
            let mut regions: Vec<PresetRegion> = Vec::new();
            for i in 0..count
            {
                regions.push(PresetRegion::new(name, global, &zones[i + 1], &instruments)?);
            }

            Ok(regions)
        }
        else
        {
            // No global zone.
            let count = zones.len();
            let mut regions: Vec<PresetRegion> = Vec::new();
            for i in 0..count
            {
                regions.push(PresetRegion::new(name, &Zone::empty(), &zones[i], &instruments)?);
            }

            Ok(regions)
        }
    }
}
