use std::error::Error;

use crate::generator::Generator;
use crate::zone_info::ZoneInfo;

#[non_exhaustive]
pub(crate) struct Zone
{
    pub(crate) generators: Vec<Generator>,
}

impl Zone
{
    pub(crate) fn empty() -> Self
    {
        Self
        {
            generators: Vec::new(),
        }
    }

    fn new(info: &ZoneInfo, generators: &Vec<Generator>) -> Self
    {
        let mut segment: Vec<Generator> = Vec::new();

        for i in 0..info.generator_count
        {
            segment.push(generators[(info.generator_index + i) as usize]);
        }

        Self
        {
            generators: segment,
        }
    }

    pub(crate) fn create(infos: &Vec<ZoneInfo>, generators: &Vec<Generator>) -> Result<Vec<Zone>, Box<dyn Error>>
    {
        if infos.len() <= 1
        {
            return Err(format!("No valid zone was found.").into());
        }

        // The last one is the terminator.
        let count = infos.len() - 1;

        let mut zones: Vec<Zone> = Vec::new();
        for i in 0..count
        {
            zones.push(Zone::new(&infos[i], &generators));
        }

        Ok(zones)
    }
}
