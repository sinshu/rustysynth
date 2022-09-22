#[allow(unused)]
#[non_exhaustive]
pub(crate) struct SoundFontMath
{
}

impl SoundFontMath
{
    pub(crate) const NON_AUDIBLE: f32 = 1.0e-3_f32;
    pub(crate) const LOG_NON_AUDIBLE: f32 = -6.90775527898_f32;

    pub(crate) fn max(x: f32, y: f32) -> f32
    {
        if x > y
        {
            return x;
        }
        else
        {
            return y;
        }
    }

    pub(crate) fn clamp(value: f32, min: f32, max: f32) -> f32
    {
        if value < min
        {
            return min;
        }
        else if value > max
        {
            return max;
        }
        else
        {
            return value;
        }
    }

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

    pub(crate) fn exp_cutoff(x: f64) -> f64
    {
        if x < SoundFontMath::LOG_NON_AUDIBLE as f64
        {
            return 0_f64;
        }
        else
        {
            return x.exp();
        }
    }
}
