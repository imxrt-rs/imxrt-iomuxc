//! UART pad configuration

/// Type tag for the transfer pin
pub enum Tx {}
/// Type tag for the receive pin
pub enum Rx {}

/// A pin direction, either transfer or receive
pub trait Direction: private::Sealed {}

impl Direction for Tx {}
impl Direction for Rx {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::Tx {}
    impl Sealed for super::Rx {}
}

/// A UART pin
pub trait Pin: super::Iomuxc {
    /// The alternate value for the UART pin
    const ALT: u32;
    /// The daisy register which will select the pad
    const DAISY: Option<super::Daisy>;
    /// Pin direction
    type Direction: Direction;
    /// UART module; `U3` for `UART3`
    type Module: super::consts::Unsigned;
}

/// Prepare a UART pin
///
/// If you do not call `prepare()` on your UART pin, it might not work as a UART
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
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() };
    }
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! uart {
    (module: $module:ty, alt: $alt:expr, pad: $pad:ty, direction: $direction:ty, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            const DAISY: Option<Daisy> = $daisy;
            type Direction = $direction;
            type Module = $module;
        }
    };
}
