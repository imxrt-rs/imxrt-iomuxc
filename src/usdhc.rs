//! USDHC pad configuration

/// Type tag for the reset pin
pub enum Cmd {}
/// Type tag for the reset pin
pub enum Clk {}
/// Type tag for the data0 pin
pub enum Data0 {}
/// Type tag for the data1 pin
pub enum Data1 {}
/// Type tag for the data2 pin
pub enum Data2 {}
/// Type tag for the data3 pin
pub enum Data3 {}

/// A pin signal
mod private {
    pub trait Sealed {}
    impl Sealed for super::Cmd {}
    impl Sealed for super::Clk {}
    impl Sealed for super::Data0 {}
    impl Sealed for super::Data1 {}
    impl Sealed for super::Data2 {}
    impl Sealed for super::Data3 {}
}

/// A uSDHC pin
pub trait Pin: super::Iomuxc {
    /// The alternate value for the uSDHC pin
    const ALT: u32;
    /// The daisy register which will select the pad
    const DAISY: Option<super::Daisy>;
    /// Pin direction
    type Signal: Signal;
    /// UART module; `U1` for `uSDHC1`
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
    super::clear_sion(pin);
    unsafe { P::DAISY.write() };
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! usdhc {
    (module: $module:ty, alt: $alt:expr, pad: $pad:ty, signal: $signal:ty, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            const DAISY: Daisy = $daisy;
            type Signal = $signal;
            type Module = $module;
        }
    };
}
