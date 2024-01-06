//! I2C pad configuration

/// Tag that indicates the SCL signal
pub enum Scl {}
/// Tag that indicates the SDA signal
pub enum Sda {}

/// An I2C signal; one of `SCL` or `SDA`
pub trait Signal: private::Sealed {}

impl Signal for Scl {}
impl Signal for Sda {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::Scl {}
    impl Sealed for super::Sda {}
}

/// An I2C pin
pub trait Pin: super::Iomuxc {
    /// Alternate value for this pin
    const ALT: u32;
    /// Daisy register
    const DAISY: Option<super::Daisy>;
    /// I2C Signal
    type Signal: Signal;
    /// I2C module; `U2` for `I2C2`
    type Module: super::consts::Unsigned;
}

/// Prepare an I2C pin
///
/// If you do not call `prepare()` on your I2C pin, it might not work as a I2C
/// pin.
///
/// In most build configurations, `prepare()` emits code to enable an open drain.
/// This code is correct for all 10xx MCUs.
///
/// If *only* the 1170 MCU feature is enabled, then `prepare()` *does not* enable
/// an open drain. For more information on this limitation, see
/// [the issue tracker](https://github.com/imxrt-rs/imxrt-iomuxc/issues/28).
///
/// If multiple MCU features are enabled, then the `prepare()` assumes that the
/// target MCU is a 10xx MCU. This code may execute on an 1170 MCU, but it may not
/// work as expected.
///
/// `prepare()` will not touch any other bits in the PAD_CTL register.
pub fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    crate::config::set_open_drain(pin);
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() };
    }
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! i2c {
    (module: $module:ty, alt: $alt:expr, pad: $pad:ty, signal: $signal:ty, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            const DAISY: Option<Daisy> = $daisy;
            type Signal = $signal;
            type Module = $module;
        }
    };
}
