//! USDHC pad configuration

/// Type tag for the command pin
pub enum Cmd {}
/// Type tag for the clock pin
pub enum Clk {}
/// Type tag for the write protect pin
pub enum Wp {}
/// Type tag for the card detection pin
pub enum CdB {}
/// Type tag for the data0 pin
pub enum Data0 {}
/// Type tag for the data1 pin
pub enum Data1 {}
/// Type tag for the data2 pin
pub enum Data2 {}
/// Type tag for the data3 pin
pub enum Data3 {}

/// A uSDHC pin signal
mod private {
    pub trait Signal {}
    impl Signal for super::Cmd {}
    impl Signal for super::Clk {}
    impl Signal for super::Wp {}
    impl Signal for super::CdB {}
    impl Signal for super::Data0 {}
    impl Signal for super::Data1 {}
    impl Signal for super::Data2 {}
    impl Signal for super::Data3 {}
}
use private::Signal;

use crate::Config;

/// A uSDHC pin
pub trait Pin: super::IOMUX {
    /// The alternate value for the uSDHC pin
    const ALT: u32;
    /// The daisy register which will select the pad
    const DAISY: Option<super::Daisy>;
    /// Pin configuration
    ///
    /// Applied during pin preparation, overwriting any
    /// other user-provided configuration.
    const CONFIG: Config;
    /// Pin direction
    type Signal: Signal;
    /// uSDHC module; `U1` for `uSDHC1`
    type Module: super::consts::Unsigned;
}

/// Prepare a uSDHC pin
///
/// If you do not call `prepare()` on your uSDHC pin, it might not work as a uSDHC
/// pin.
///
/// # Safety
///
/// `prepare()` inherits all the unsafety that comes from the `IOMUX` supertrait.
/// In particular, we cannot be sure that the implementation's pointers are correct.
/// It may also write a daisy configuration that's incorrect.
pub fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    super::configure(pin, P::CONFIG);
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() }
    }
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! usdhc {
    (module: $module:ty, alt: $alt:expr, pad: $pad:ty, signal: $signal:ty, keeper: $keeper:expr, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            const DAISY: Option<Daisy> = $daisy;
            const CONFIG: crate::Config = crate::Config::zero()
                .set_speed(crate::Speed::Fast)
                .set_drive_strength(crate::DriveStrength::R0_7)
                .set_pull_keeper($keeper);
            type Signal = $signal;
            type Module = $module;
        }
    };
}
