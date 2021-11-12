//! ADC pad configuration

use crate::consts::Unsigned;

/// Type number for ADC1
pub type Adc1 = crate::consts::U1;
/// Type number for ADC2
pub type Adc2 = crate::consts::U2;

/// Describes an ADC input pin
///
/// ADC pins are specialized GPIO pins. Some pads may be used in both `Adc1`
/// and `Adc2`, so implementations will indicate their compatibility by
/// supplying an identifier in place of `ADCx`.
pub trait Pin<U: Unsigned>: super::gpio::Pin {
    /// The input pin identifier
    ///
    /// Starts at `0`, and increments up.
    const INPUT: u32;
}

/// Prepare an ADC pin
///
/// Due to a requirement in the ADC module, `prepare` will disable the pull/keeper
/// on the pin. The configuration change will not affect any other settings.
pub fn prepare<U: Unsigned, P: Pin<U>>(pin: &mut P) {
    // See the note in the ADC section of the reference manual
    // (using iMXRT1060, rev 2). ADC input signals connect to
    // GPIO, and we need to disable the keeper to prevent signal
    // jumps.
    super::alternate(pin, <P as super::gpio::Pin>::ALT);
    super::configure(pin, super::Config::modify().set_pull_keeper(None));
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! adc {
    (module: $module:ty, pad: $pad:ty, input: $input:expr) => {
        impl Pin<$module> for $pad {
            const INPUT: u32 = $input;
        }
    };
}
