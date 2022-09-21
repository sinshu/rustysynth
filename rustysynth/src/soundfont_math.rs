#[allow(unused)]
pub(crate) struct SoundFontMath
{
}

impl SoundFontMath
{
    pub(crate) fn timecents_to_seconds(x: f32) -> f32
    {
        2_f32.powf((1_f32 / 1200_f32) * x)
    }

    pub(crate) fn cents_to_hertz(x: f32) -> f32
    {
        8.176_f32 * 2_f32.powf((1_f32 / 1200_f32) * x)
    }

    pub(crate) fn cents_to_multiplying_factor(x: f32) -> f32
    {
        2_f32.powf((1_f32 / 1200_f32) * x)
    }
}
