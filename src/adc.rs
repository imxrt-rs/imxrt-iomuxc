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
    // Set the GPIO alternate value.
    //
    // A general approach would add a constraint on the generic P for
    // gpio::Pin<Q> (where Q is some other constant). This works, but it
    // burdens those who design higher-level APIs, since they also need
    // the constraint on their APIs. Hard-coding the alternate here is easier.
    //
    // Although not tested, it's also thought to be correct for ADC peripherals
    // on the 1170. The ADC peripherals are in the M7 WAKEUP domain, and the alts
    // for configuring the GPIOs for this domain are all '5'. If this call were
    // to use the GPIO alternate for the CM4's LPSR domain (alt 10, for instance),
    // the ADC pin may be misconfigured.
    super::alternate(pin, 5);
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! adc {
    (module: $module:expr, pad: $pad:ty, input: $input:expr) => {
        impl Pin<$module> for $pad {
            const INPUT: u32 = $input;
        }
    };
}
