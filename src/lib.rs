//! `imxrt-iomuxc` is a library for configuring i.MX RT processor pads. Using its API, you can
//!
//! - configure a pad for a peripheral, and specify its electrical properties.
//! - manage pad objects as ownable resources.
//! - statically constrain pads to work with peripherals.
//!
//! As an end user, you're expected to use `imxrt-iomuxc` through a hardware abstraction layer
//! (HAL) or board support package (BSP). Specifically, you should have access to pad structs and
//! objects, and you should be able to configure pads with the [`configure`] APIs.
//!
//! As a library developer who writes HALs or hardware drivers, you may use the `imxrt-iomuxc`
//! pin traits in your APIs to statically ensure pad-peripheral compatibility. See the design
//! guidance for examples on how to achieve this.
//!
//! # Definitions
//!
//! A 'pad' is the physical input / output on an i.MX RT processor.
//! Pads may be configured for various functions. A pad may act as a UART pin, an I2C
//! pin, or other types of pins. A 'pin' is a pad that's configured for a functional
//! purpose. The traits let us say which pad can be used for which peripheral pin.
//!
//! # Features
//!
//! Processor pads, and their pin implementations, are enabled with optional feature flags. For
//! example, the `imxrt1060` feature flag exposes an `imxrt1060` module that defines all i.MX
//! RT 1060 processor pads. Users and integrators are responsible for making sure an enabled
//! feature makes sense for their system.
//!
//! # Design Guidance
//!
//! For recommendations on how you can use these traits, see the module-level documentation. The
//! rest of this section describes general guidance for designing APIs with these traits.
//!
//! ## Matching pads and peripherals
//!
//! The pin traits allow you to restrict the pads and peripherals that comprise a peripheral. This
//! lets you catch invalid peripheral configurations at compile time.
//!
//! In the example below, we implement a hypothetical `lpuart_new` function, which is responsible
//! for preparing a LPUART peripheral. To properly configure the peripheral, we need the two
//! pads that represent a peripheral's TX and RX pins. The implementation will use the
//! `imxrt_iomuxc::lpuart::prepare()` function to prepare the pins.
//!
//! Note the trait bounds on `lpuart_new`. The usage requires that
//!
//! - the user provides one TX and one RX pin
//! - the modules for each pin match
//!
//! ```no_run
//! use imxrt_iomuxc as iomuxc;
//! use iomuxc::lpuart::{Pin, Tx, Rx};
//!
//! # struct Lpuart<const N: u8>;
//! /// Creates a UART peripheral from the TX and RX pads
//! fn lpuart_new<T, R, const N: u8>(mut tx: T, mut rx: R) -> Lpuart<N>
//! where
//!     T: Pin<Direction = Tx, Module = iomuxc::consts::Const<N>>,
//!     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! {
//!     iomuxc::lpuart::prepare(&mut tx);
//!     iomuxc::lpuart::prepare(&mut rx);
//!     // Prepare the rest of the peripheral, and return it...
//!     # Lpuart
//! }
//!
//! # let gpio_ad_b0_13 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_13::new() };
//! # let gpio_ad_b0_12 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_12::new() };
//! // GPIO_AD_B0_13 and GPIO_AD_B0_12 are a suitable pair of UART pins
//! lpuart_new(gpio_ad_b0_12, gpio_ad_b0_13);
//! ```
//!
//! Specifically, the trait bounds guard against non-UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::lpuart::{Pin, Tx, Rx};
//! # struct Lpuart<const N: u8>;
//! # fn lpuart_new<T, R, const N: u8>(mut tx: T, mut rx: R) -> Lpuart<N>
//! # where
//! #     T: Pin<Direction = Tx, Module = iomuxc::consts::Const<N>>,
//! #     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! # {
//! #     iomuxc::lpuart::prepare(&mut tx);
//! #     iomuxc::lpuart::prepare(&mut rx);
//! #     Lpuart
//! # }
//! # let gpio_ad_b0_10 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_10::new() };
//! # let gpio_ad_b0_11 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_11::new() };
//! // Neither pad is a valid UART pin
//! lpuart_new(gpio_ad_b0_10, gpio_ad_b0_11);
//! ```
//!
//! It also guards against mismatched UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::lpuart::{Pin, Tx, Rx};
//! # struct Lpuart<const N: u8>;
//! # fn lpuart_new<T, R, const N: u8>(mut tx: T, mut rx: R) -> Lpuart<N>
//! # where
//! #     T: Pin<Direction = Tx, Module = iomuxc::consts::Const<N>>,
//! #     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! # {
//! #     iomuxc::lpuart::prepare(&mut tx);
//! #     iomuxc::lpuart::prepare(&mut rx);
//! #     Lpuart
//! # }
//! # let gpio_ad_b0_13 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_13::new() };
//! # let gpio_ad_b1_02 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b1::GPIO_AD_B1_02::new() };
//! // GPIO_AD_B1_02 is a UART2 TX pin, but GPIO_AD_B0_13 is a UART1 RX pin
//! lpuart_new(gpio_ad_b1_02, gpio_ad_b0_13);
//! ```
//!
//! ## Type-Erased Pads
//!
//! At the expense of requiring `unsafe`, users may favor type-erased pads over strongly-typed pads.
//! When creating APIs that consume strongly-typed pads, or pads that conform to peripheral pin interfaces,
//! consider supporting an `unsafe` API to create the peripheral without requiring the strongly-typed pads.
//! The API will expect that the user is responsible for manually configuring the type-erased pad.
//!
//! ```no_run
//! use imxrt_iomuxc::{self as iomuxc, ErasedPad, lpuart::{Pin, Tx, Rx}};
//! # use imxrt_iomuxc::imxrt1060::gpio_ad_b0::{GPIO_AD_B0_13, GPIO_AD_B0_12};
//! # pub struct Lpuart<const N: u8>;
//!
//! impl<const N: u8> Lpuart<N> {
//!     pub fn new<T, R>(mut tx: T, mut rx: R, /* ... */) -> Self
//!     where
//!         T: Pin<Direction = Tx, Module = iomuxc::consts::Const<N>>,
//!         R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//!     {
//!         imxrt_iomuxc::lpuart::prepare(&mut tx);
//!         imxrt_iomuxc::lpuart::prepare(&mut rx);
//!         // ...
//!         # Lpuart
//!     }
//!
//!     pub fn with_erased_pads(tx: ErasedPad, rx: ErasedPad, /* ... */) -> Self {
//!         // ...
//!         # Lpuart
//!     }
//! }
//!
//! // Preferred: create a LPUART peripheral with strongly-typed pads...
//! let gpio_ad_b0_13 = unsafe { GPIO_AD_B0_13::new() };
//! let gpio_ad_b0_12 = unsafe { GPIO_AD_B0_12::new() };
//! let uart1 = Lpuart::<1>::new(gpio_ad_b0_12, gpio_ad_b0_13);
//!
//! // Optional: create a LPUART peripheral from type-erased pads...
//! let gpio_ad_b0_13 = unsafe { GPIO_AD_B0_13::new() };
//! let gpio_ad_b0_12 = unsafe { GPIO_AD_B0_12::new() };
//!
//! let mut rx_pad = gpio_ad_b0_13.erase();
//! let mut tx_pad = gpio_ad_b0_12.erase();
//!
//! // User is responsible for configuring the pad,
//! // since we can't call `prepare()` on the pad...
//! unsafe {
//!     // Daisy registers and values aren't attached
//!     // to erased pads, so we have to reference this
//!     // manually.
//!     <GPIO_AD_B0_13 as imxrt_iomuxc::lpuart::Pin>::DAISY.map(|daisy| daisy.write());
//!     <GPIO_AD_B0_12 as imxrt_iomuxc::lpuart::Pin>::DAISY.map(|daisy| daisy.write());
//! }
//! imxrt_iomuxc::alternate(&mut tx_pad, 2);
//! imxrt_iomuxc::alternate(&mut rx_pad, 2);
//! imxrt_iomuxc::clear_sion(&mut tx_pad);
//! imxrt_iomuxc::clear_sion(&mut rx_pad);
//! // Pads are configured for LPUART settings
//! let uart1 = Lpuart::<1>::with_erased_pads(tx_pad, rx_pad);
//! ```

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
pub mod adc;
mod config;
#[macro_use]
pub mod flexpwm;
#[macro_use]
pub mod lpi2c;
#[macro_use]
pub mod lpspi;
#[macro_use]
pub mod lpuart;
#[macro_use]
pub mod sai;
#[macro_use]
pub mod usdhc;

use core::ptr;

pub use config::{
    configure, Config, DriveStrength, Hysteresis, OpenDrain, PullKeeper, SlewRate, Speed,
};

/// Re-export of top-level components, without the chip-specific modules.
///
/// `prelude` is to help HAL implementors re-export the `imxrt-iomuxc` APIs
/// as a single module.
///
/// ```
/// // Your crate's module:
/// pub mod iomuxc {
///     // Re-export common modules and types
///     pub use imxrt_iomuxc::prelude::*;
///     // Conditionally re-export chip-specific pads
///     #[cfg(feature = "imxrt1060")]
///     pub use imxrt_iomuxc::imxrt1060::*;
/// }
/// ```
pub mod prelude {
    pub use crate::config::{
        configure, Config, DriveStrength, Hysteresis, OpenDrain, PullKeeper, SlewRate, Speed,
    };

    pub use crate::{
        adc, alternate, ccm, clear_sion, consts, flexpwm, gpio, lpi2c, lpspi, lpuart, sai,
        set_sion, usdhc, Daisy, ErasedPad, Pad, WrongPadError,
    };
}

/// Type-level constants and traits.
pub mod consts {
    /// A type-level constant.
    ///
    /// You can pattern match these in trait constraints. See the package documentation for
    /// examples.
    #[derive(Debug)]
    pub enum Const<const N: u8> {}
    #[doc(hidden)]
    pub trait Unsigned {
        const USIZE: usize;
        fn to_usize() -> usize {
            Self::USIZE
        }
    }
    impl<const N: u8> Unsigned for Const<N> {
        const USIZE: usize = N as usize;
    }
    macro_rules! ux {
        ($($Ux:ident => $N:literal,)+) => {
            $(pub type $Ux = Const<$N>;)+
        };
    }
    ux! {
        U0 => 0, U1 => 1, U2 => 2, U3 => 3, U4 => 4,
        U5 => 5, U6 => 6, U7 => 7, U8 => 8, U9 => 9,
        U10 => 10, U11 => 11, U12 => 12, U13 => 13, U14 => 14,
        U15 => 15, U16 => 16, U17 => 17, U18 => 18, U19 => 19,
        U20 => 20, U21 => 21, U22 => 22, U23 => 23, U24 => 24,
        U25 => 25, U26 => 26, U27 => 27, U28 => 28, U29 => 29,
        U30 => 30, U31 => 31, U32 => 32, U33 => 33, U34 => 34,
        U35 => 35, U36 => 36, U37 => 37, U38 => 38, U39 => 39,
        U40 => 40, U41 => 41,
    }
}

#[cfg(feature = "imxrt1010")]
#[cfg_attr(docsrs, doc(cfg(feature = "imxrt1010")))]
pub mod imxrt1010;

#[cfg(feature = "imxrt1060")]
#[cfg_attr(docsrs, doc(cfg(feature = "imxrt1060")))]
pub mod imxrt1060;

#[cfg(feature = "imxrt1170")]
#[cfg_attr(docsrs, doc(cfg(feature = "imxrt1170")))]
pub mod imxrt1170;

/// An IOMUXC-capable pad which can support I/O multiplexing
///
/// # Safety
///
/// This should only be implemented on types that return pointers to static
/// memory.
pub unsafe trait Iomuxc: private::Sealed {
    /// Returns the absolute address of the multiplex register.
    #[doc(hidden)]
    fn mux(&mut self) -> *mut u32;
    /// Returns the absolute address of the pad configuration register.
    #[doc(hidden)]
    fn pad(&mut self) -> *mut u32;
}

mod private {
    pub trait Sealed {}
}

const SION_BIT: u32 = 1 << 4;

/// Set the SION bit in a pad's MUX register
///
/// Users who are using strongly-typed pads should not call `set_sion()` directly.
/// Instead, `set_sion()` will be used in a peripheral's `prepare()` function as needed,
/// so that you don't have to call it.
///
/// However, you should use `set_sion()` if you're using any type-erased pads, since those
/// pads cannot be used with a peripheral's `prepare()` function.
#[inline(always)]
pub fn set_sion<I: Iomuxc>(pad: &mut I) {
    // Safety:
    //
    // Pointer reads and writes are unsafe. But, because we control
    // all IOMUXC implementations, we know that the returned pointers
    // are vaild, aligned, and initialized (MMIO memory).
    //
    // The interface design ensures that all pads, type I, are unique
    // owners of MMIO memory. Users would have to use unsafe code to violate
    // that guarantee.
    //
    // By taking a mutable reference, the caller has to ensure atomicity of this
    // read-modify-write operation (or, violate the requirement with more unsafe
    // code).
    unsafe {
        let mut mux = ptr::read_volatile(pad.mux());
        mux |= SION_BIT;
        ptr::write_volatile(pad.mux(), mux);
    }
}

/// Clear the SION bit in a pad's MUX register
///
/// Users who are using strongly-typed pads should not call `clear_sion()` directly.
/// Instead, `clear_sion()` will be used in a peripheral's `prepare()` function as needed,
/// so that you don't have to call it.
///
/// However, you should use `clear_sion()` if you're using any type-erased pads, since those
/// pads cannot be used with a peripheral's `prepare()` function.
#[inline(always)]
pub fn clear_sion<I: Iomuxc>(pad: &mut I) {
    // Safety: same justification as set_sion
    unsafe {
        let mut mux = ptr::read_volatile(pad.mux());
        mux &= !SION_BIT;
        ptr::write_volatile(pad.mux(), mux);
    }
}

/// Set an alternate value for the pad
///
/// Users who are using strongly-typed pads should not call `alternate()` directly.
/// Instead, `alternate()` will be used in a peripheral's `prepare()` function as needed,
/// so that you don't have to call it.
///
/// However, you should use `alternate()` if you're using any type-erased pads, since those
/// pads cannot be used with a peripheral's `prepare()` function.
#[inline(always)]
pub fn alternate<I: Iomuxc>(pad: &mut I, alt: u32) {
    const ALT_MASK: u32 = 0b1111;
    // Safety: same justification as set_sion. Argument extends to
    // pad values and alternate values.
    unsafe {
        let mut mux = ptr::read_volatile(pad.mux());
        mux = (mux & !ALT_MASK) | (alt & ALT_MASK);
        ptr::write_volatile(pad.mux(), mux);
    }
}

/// An i.MXT RT pad
///
/// The `Base` is the pad tag, like `GPIO_AD_B0`. The `Offset` is the
/// constant (type) that describes the pad number.
///
/// `Pad`s have no size.
#[derive(Debug)]
pub struct Pad<const MUX: u32, const PAD: u32> {
    // Block auto-implement of Send / Sync. We'll manually implement
    // the traits.
    _not_send_sync: ::core::marker::PhantomData<*const ()>,
}

impl<const MUX: u32, const PAD: u32> Pad<MUX, PAD> {
    /// Creates a handle to the pad
    ///
    /// # Safety
    ///
    /// `new()` may be called anywhere, by anyone. This could lead to multiple objects that
    /// mutate the same memory. Consider calling `new()` once, near startup, then passing objects
    /// and references throughout your program.
    #[inline(always)]
    pub const unsafe fn new() -> Self {
        Self {
            _not_send_sync: ::core::marker::PhantomData,
        }
    }
    /// Cast the MUX address.
    const fn mux() -> *mut u32 {
        MUX as *mut u32
    }
    /// Cast the PAD address.
    const fn pad() -> *mut u32 {
        PAD as *mut u32
    }
}

unsafe impl<const MUX: u32, const PAD: u32> Send for Pad<MUX, PAD> {}

impl<const MUX: u32, const PAD: u32> Pad<MUX, PAD> {
    /// Erase the pad's type, returning an `ErasedPad`
    #[inline(always)]
    pub const fn erase(self) -> ErasedPad {
        ErasedPad {
            mux: Self::mux(),
            pad: Self::pad(),
        }
    }

    /// Set the alternate value for this pad.
    ///
    /// Performs a read-modify-write on the pad's mux register to set the
    /// alternate value to `alt`.
    ///
    /// # Safety
    ///
    /// This function performs a read-modify-write operation on peripheral
    /// memory. It could race with other calls that modify this pad's mux register.
    /// For a safer interface, see [`alternate()`](crate::alternate()).
    #[inline(always)]
    pub unsafe fn set_alternate(alt: u32) {
        let mut pad = Self::new();
        alternate(&mut pad, alt);
    }

    /// Set the pad's SION bit.
    ///
    /// Performs a read-modify-write on the pad's mux register to set the SION
    /// bit.
    ///
    /// # Safety
    ///
    /// This function performs a read-modify-write operation on peripheral
    /// memory. It could race with other calls that modify this pad's mux register.
    /// For a safer interface, see [`set_sion()`](crate::set_sion()).
    #[inline(always)]
    pub unsafe fn set_sion() {
        let mut pad = Self::new();
        set_sion(&mut pad);
    }

    /// Clear the pad's SION bit.
    ///
    /// Performs a read-modify-write on the pad's mux register to Clear the SION
    /// bit.
    ///
    /// # Safety
    ///
    /// This function performs a read-modify-write operation on peripheral
    /// memory. It could race with other calls that modify this pad's mux register.
    /// For a safer interface, see [`clear_sion()`](crate::clear_sion()).
    #[inline(always)]
    pub unsafe fn clear_sion() {
        let mut pad = Self::new();
        clear_sion(&mut pad);
    }

    /// Set the pad's configuration.
    ///
    /// # Safety
    ///
    /// This function performs a read-modify-write operation on peripheral memory.
    /// It could race with any other function that modifies this pad's registers.
    /// For a safer interface, see [`configure()`](crate::configure()).
    #[inline(always)]
    pub unsafe fn configure(config: Config) {
        let mut pad = Self::new();
        configure(&mut pad, config);
    }
}

impl<const MUX: u32, const PAD: u32> private::Sealed for Pad<MUX, PAD> {}

unsafe impl<const MUX: u32, const PAD: u32> crate::Iomuxc for Pad<MUX, PAD> {
    #[inline(always)]
    fn mux(&mut self) -> *mut u32 {
        Self::mux()
    }

    #[inline(always)]
    fn pad(&mut self) -> *mut u32 {
        Self::pad()
    }
}

/// A pad that has its type erased
///
/// `ErasedPad` moves the pad state to run time, rather than compile time.
/// The type may provide more flexibility for some APIs. Each `ErasedPad` is
/// two pointers large.
///
/// `ErasedPad` may be converted back into their strongly-typed analogs using
/// `TryFrom` and `TryInto` conversions.
///
/// ```no_run
/// use imxrt_iomuxc as iomuxc;
/// # type GPIO_AD_B0_03 = iomuxc::Pad<0xDEAD, 0xBEEF>;
/// let gpio_ad_b0_03 = unsafe { GPIO_AD_B0_03::new() };
/// let mut erased = gpio_ad_b0_03.erase();
///
/// // Erased pads may be manually manipulated
/// iomuxc::alternate(&mut erased, 7);
/// iomuxc::set_sion(&mut erased);
///
/// // Try to convert the erased pad back to its strongly-typed counterpart
/// use core::convert::TryFrom;
/// let gpio_ad_b0_03 = GPIO_AD_B0_03::try_from(erased).unwrap();
/// ```
#[derive(Debug)]
pub struct ErasedPad {
    mux: *mut u32,
    pad: *mut u32,
}

impl private::Sealed for ErasedPad {}

unsafe impl crate::Iomuxc for ErasedPad {
    #[inline(always)]
    fn mux(&mut self) -> *mut u32 {
        self.mux
    }

    #[inline(always)]
    fn pad(&mut self) -> *mut u32 {
        self.pad
    }
}

unsafe impl Send for ErasedPad {}

/// An error that indicates the conversion from an `ErasedPad` to a
/// strongly-typed pad failed.
///
/// Failure happens when trying to convert an `ErasedPad` into the incorrect
/// pad. The error indicator wraps the pad that failed to convert.
#[derive(Debug)]
pub struct WrongPadError(pub ErasedPad);

impl<const MUX: u32, const PAD: u32> ::core::convert::TryFrom<ErasedPad> for Pad<MUX, PAD> {
    type Error = WrongPadError;
    fn try_from(erased_pad: ErasedPad) -> Result<Self, Self::Error> {
        if erased_pad.mux == Self::mux() && erased_pad.pad == Self::pad() {
            Ok(unsafe { Self::new() })
        } else {
            Err(WrongPadError(erased_pad))
        }
    }
}

/// A daisy selection
///
/// A daisy chain specifies which pad will be used for a peripheral's
/// input. Call `write()` to commit the settings described by a `Daisy`
/// value.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Daisy {
    reg: *mut u32,
    value: u32,
}

impl Daisy {
    /// Create a new select input that, when utilized, will write
    /// `value` into `reg`
    #[allow(unused)] // Used behind feature flags
    const fn new(reg: *mut u32, value: u32) -> Self {
        Daisy { reg, value }
    }

    /// Commit the settings defined by this `Daisy` value to the hardware
    ///
    /// # Safety
    ///
    /// This modifies a global, processor register, so the typical
    /// rules around mutable static memory apply.
    #[inline(always)]
    pub unsafe fn write(self) {
        ptr::write_volatile(self.reg, self.value);
    }
}

/// GPIO pad configuration
pub mod gpio {
    /// A GPIO pin
    ///
    /// The constant `N` is the associated GPIO module
    /// (a `3` for `GPIO3`).
    pub trait Pin<const N: u8>: super::Iomuxc {
        /// The alternate value for this pad
        const ALT: u32;
        /// The offset; `U13` for `GPIO5_IO13`
        const OFFSET: u32;
    }

    /// Prepare a pad to be used as a GPIO pin
    pub fn prepare<P: Pin<N>, const N: u8>(pin: &mut P) {
        super::alternate(pin, P::ALT);
    }
}

/// CCM pad configuration.
pub mod ccm {
    /// A CCM pin.
    ///
    /// These can be used for observing clock outputs, or for generating
    /// outputs for your PMIC.
    pub trait Pin: super::Iomuxc {
        /// The alternate value for this pad.
        const ALT: u32;
        /// The pin function.
        type Function: Function;
    }

    /// Prepare a pad to be used as a CCM pin.
    pub fn prepare<P: Pin>(pin: &mut P) {
        super::alternate(pin, P::ALT);
    }

    mod private {
        pub trait Sealed {}
    }
    /// A CCM pin function.
    pub trait Function: private::Sealed {}

    /// Observability output.
    pub enum Observable<const N: u8> {}
    impl private::Sealed for Observable<1> {}
    impl private::Sealed for Observable<2> {}
    impl Function for Observable<1> {}
    impl Function for Observable<2> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestBase;

    type TestPad = Pad<0, 0>;

    #[test]
    fn erased_pad_convert_success() {
        let pad = unsafe { TestPad::new() };
        let erased = pad.erase();

        use core::convert::TryFrom;
        TestPad::try_from(erased).expect("This is the test pad");
    }

    #[test]
    fn erased_pad_convert_fail() {
        let pad = unsafe { TestPad::new() };
        let erased = pad.erase();

        use core::convert::TryFrom;
        type OtherPad = Pad<1, 1>;
        OtherPad::try_from(erased).expect_err("This is a different pad");
    }
}

/// ```
/// fn is_send<S: Send>(s: S) {}
/// type GPIO_AD_B0_03 = imxrt_iomuxc::Pad<0xDEAD, 0xBEEF>;
/// is_send(unsafe { GPIO_AD_B0_03::new() }.erase());
/// ```
#[cfg(doctest)]
struct ErasedPadsAreSend;

/// ```
/// fn is_send<S: Send>(s: S) {}
/// type GPIO_AD_B0_03 = imxrt_iomuxc::Pad<0xDEAD, 0xBEEF>;
/// is_send(unsafe { GPIO_AD_B0_03::new() });
/// is_send(unsafe { GPIO_AD_B0_03::new() }.erase());
/// ```
#[cfg(doctest)]
struct PadsAreSend;

/// ```compile_fail
/// fn is_sync<S: Sync>(s: S) {}
/// type GPIO_AD_B0_03 = imxrt_iomuxc::Pad<0xDEAD, 0xBEEF>;
/// is_sync(unsafe { GPIO_AD_B0_03::new() }.erase())
/// ```
#[cfg(doctest)]
struct ErasedPadsAreNotSync;

/// ```compile_fail
/// fn is_sync<S: Sync>(s: S) {}
/// type GPIO_AD_B0_03 = imxrt_iomuxc::Pad<0xDEAD, 0xBEEF>;
/// is_sync(unsafe { GPIO_AD_B0_03::new() })
/// ```
#[cfg(doctest)]
struct PadsAreNotSync;
