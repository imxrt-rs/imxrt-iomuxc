//! ADC pad configuration

pub const ADC1: u8 = 1;
pub const ADC2: u8 = 2;

/// Describes an ADC input pin
///
/// Some pads may be used in both `ADC1` and `ADC2`, so implementations
/// indicate their compatibility by supplying a constant `N`.
pub trait Pin<const N: u8>: super::Iomuxc {
    /// The input pin identifier
    ///
    /// Starts at `0`, and increments up.
    const INPUT: u32;
}

/// Prepare an ADC pin
///
/// Due to a requirement in the ADC module, `prepare` will disable the pull/keeper
/// on the pin. The configuration change will not affect any other settings.
pub fn prepare<P: Pin<N>, const N: u8>(pin: &mut P) {
    // See the note in the ADC section of the reference manual
    // (using iMXRT1060, rev 2). ADC input signals connect to
    // GPIO, and we need to disable the keeper to prevent signal
    // jumps.
    super::configure(pin, super::Config::modify().set_pull_keeper(None));
    // Not putting the ADC into the GPIO alternate. Reference
    // manuals indicate that the alt (mode) doesn't matter. We're
    // expecting that the GPIO input path is implicit, regardless
    // of the alt.
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! adc {
    (module: $module:expr, pad: $pad:ty, input: $input:expr) => {
        impl Pin<$module> for $pad {
            const INPUT: u32 = $input;
        }
    };
}
