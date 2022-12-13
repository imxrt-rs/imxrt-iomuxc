//! Can pad configurations

/// A Can signal
pub trait Signal: private::Sealed {}

/// A tag that indicates a Can TX pad
pub enum Tx {}
/// A tag that indicates a Can Rx pad
pub enum Rx {}

impl Signal for Tx {}
impl Signal for Rx {}
mod private {
    pub trait Sealed {}
    impl Sealed for super::Tx {}
    impl Sealed for super::Rx {}
}

/// A Can pin
pub trait Pin: super::IOMUX {
    /// Alternate value for this pin
    const ALT: u32;
    /// Daisy register
    const DAISY: Option<super::Daisy>;
    /// CAN signal
    type Signal: Signal;
    /// CAN module; `U1` for `CAN1`
    type Module: super::consts::Unsigned;
}

/// Prepare a Can pin
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
macro_rules! can {
    (module: $module:ty, alt: $alt:expr, pad: $pad:ty, signal: $signal:ty, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            const DAISY: Option<Daisy> = $daisy;
            type Signal = $signal;
            type Module = $module;
        }
    };
}
