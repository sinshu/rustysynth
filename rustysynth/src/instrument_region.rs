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
}
