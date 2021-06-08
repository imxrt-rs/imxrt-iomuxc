//! SAI / I2S pad configurations
//!
//! # Examples
//!
//! Accept a transfer pin in a SAI driver. Change `TxDataSignal` to `RxDataSignal` for
//! the inverse operation.
//!
//! ```
//! use imxrt_iomuxc::sai::{Pin, TxDataSignal};
//! use imxrt_iomuxc::consts::{U1, Unsigned};
//!
//! struct SAI<U> {
//!     /* Driver details... */
//!     # _u: core::marker::PhantomData<U>,
//! }
//!
//! type SAI1 = SAI<U1>;
//!
//! impl<U: Unsigned> SAI<U> {
//!     fn add_tx_pin<P>(&mut self, pin: P)
//!     where
//!         P: Pin<U>,
//!         <P as Pin<U>>::Signal: TxDataSignal,
//!     {
//!         let tx_offset: usize = <P::Signal as TxDataSignal>::Index::to_usize();
//!         assert_eq!(tx_offset, 1);
//!         // ...
//!     }
//! }
//!
//! let mut sai1: SAI1 = // Create SAI1 driver...
//!     # SAI { _u: core::marker::PhantomData };
//! let sd_b1_02 = // 1060 SAI1 TX_DATA01 pin...
//!     # unsafe { imxrt_iomuxc::imxrt1060::sd_b1::SD_B1_02::new() };
//!
//!
//! sai1.add_tx_pin(sd_b1_02);
//! ```

/// An SAI pin signal
pub trait Signal: Sealed {}
/// An SAI TX data signal
pub trait TxDataSignal: Signal {
    /// Data pin index; the `3` in `TX_DATA03`
    type Index: super::consts::Unsigned;
}
/// An SAI RX data signal
pub trait RxDataSignal: Signal {
    /// Data pin index; the `1` in `RX_DATA01`
    type Index: super::consts::Unsigned;
}

pub(crate) mod private {
    pub trait Sealed {}
}
use private::Sealed;

/// A tag that indicates a SAI TX bit clock pad
pub enum TxBclk {}
/// A tag that indicates a SAI TX frame sync pad
pub enum TxSync {}
/// A tag that indicates a SAI RX bit clock pad
pub enum RxBclk {}
/// A tag that indicates a SAI RX frame sync pad
pub enum RxSync {}
/// A tag that indicates a SAI MCLK pad
pub enum Mclk {}

/// A SAI TX data pin
pub enum TxData {}
/// A SAI RX data pin
pub enum RxData {}

impl Signal for TxBclk {}
impl Signal for TxSync {}
impl Signal for RxBclk {}
impl Signal for RxSync {}
impl Signal for Mclk {}

impl Signal for TxData {}
impl TxDataSignal for TxData {
    type Index = super::consts::U0;
}
impl Signal for RxData {}
impl RxDataSignal for RxData {
    type Index = super::consts::U0;
}

impl Sealed for TxBclk {}
impl Sealed for TxSync {}
impl Sealed for RxBclk {}
impl Sealed for RxSync {}
impl Sealed for Mclk {}
impl Sealed for TxData {}
impl Sealed for RxData {}

/// A pin that can be used for a SAI peripheral
///
/// `SAIx` is a type number, like `U2`, which indicates 'SAI2'.
pub trait Pin<SAIx: crate::consts::Unsigned>: super::Iomuxc {
    /// The alternate value for the UART pin
    const ALT: u32;
    /// The daisy register which will select the pad
    const DAISY: Option<super::Daisy>;
    /// The SAI signal
    type Signal: Signal;
}

/// Prepare a pad to be used as a SAI pin
pub fn prepare<SAIx: crate::consts::Unsigned, P: Pin<SAIx>>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() };
    }
}

/// Defines an SAI pin
#[allow(unused)] // Used in chip-specific modules...
macro_rules! sai {
    (module: $m:ty, alt: $alt:expr, pad: $pad:ty, signal: $signal:ty, daisy: $daisy:expr) => {
        impl Pin<$m> for $pad {
            const ALT: u32 = $alt;
            type Signal = $signal;
            const DAISY: Option<Daisy> = $daisy;
        }
    };
}
