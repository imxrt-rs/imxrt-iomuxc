//! SPI pad configurations

/// A SPI signal
pub trait Signal: private::Sealed {}

/// A tag that indicates a SPI clock pad
pub enum Sck {}
/// A tag that indicates a SPI data out pad
pub enum Sdo {}
/// A tag that indicates a SPI data in pad
pub enum Sdi {}
/// A tag that indicates a SPI chip select pad
pub enum Pcs0 {}

impl Signal for Sck {}
impl Signal for Sdo {}
impl Signal for Sdi {}
impl Signal for Pcs0 {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::Sck {}
    impl Sealed for super::Sdo {}
    impl Sealed for super::Sdi {}
    impl Sealed for super::Pcs0 {}
}

/// A SPI pin
pub trait Pin: super::Iomuxc {
    /// Alternate value for this pin
    const ALT: u32;
    /// Daisy register
    const DAISY: Option<super::Daisy>;
    /// SPI signal
    type Signal: Signal;
    /// SPI module; `U3` for `SPI3`
    type Module: super::consts::Unsigned;
}

/// Prepare a SPI pin
///
/// If you do not call `prepare()` on your SPI pin, it might work as
/// a SPI pin.
///
/// # Safety
///
/// `prepare()` inherits all the unsafety that comes from the `IOMUX` supertrait.
pub fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() };
    }
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! spi {
    (module: $module:ty, alt: $alt:expr, pad: $pad:ty, signal: $signal:ty, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            const DAISY: Option<Daisy> = $daisy;
            type Signal = $signal;
            type Module = $module;
        }
    };
}
