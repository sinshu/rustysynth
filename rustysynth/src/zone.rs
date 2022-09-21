use std::error;

use crate::generator::Generator;
use crate::zone_info::ZoneInfo;

pub(crate) struct Zone
{
    pub(crate) generators: Vec<Generator>,
}

impl Zone
{
    fn empty() -> Self
    {
        Zone
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

        Zone
        {
            generators: segment,
        }
    }

    pub(crate) fn create(infos: &Vec<ZoneInfo>, generators: &Vec<Generator>) -> Result<Vec<Zone>, Box<dyn error::Error>>
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
