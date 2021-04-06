//! ADC pad configuration

use crate::consts::Unsigned;

/// Type number for ADC1
pub type ADC1 = crate::consts::U1;
/// Type number for ADC2
pub type ADC2 = crate::consts::U2;

/// Describes an ADC input pin
///
/// ADC pins are specialized GPIO pins. Some pads may be used in both `ADC1`
/// and `ADC2`, so implementations will indicate their compatibility by
/// supplying an identifier in place of `ADCx`.
pub trait Pin<U: Unsigned>: super::gpio::Pin {
    /// The input pin identifier
    ///
    /// Starts at `U0`, and increments up.
    type Input: super::consts::Unsigned;
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
    super::configure(
        pin,
        super::Config::modify().set_pull_keep(super::PullKeep::Disabled),
    );
}
