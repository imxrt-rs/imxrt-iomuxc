//! PWM pad configuration

/// A PWM output identified; one of `A` or `B`
pub trait Output: private::Sealed {}
/// PWM output A
pub enum A {}
/// PWM output B
pub enum B {}

impl Output for A {}
impl Output for B {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::A {}
    impl Sealed for super::B {}
}

/// A PWM pin
pub trait Pin: super::Iomuxc {
    /// The alternate mode for the PWM pin
    const ALT: u32;
    /// The output identifier
    type Output: Output;
    /// The daisy register which will select the pad
    const DAISY: Option<super::Daisy>;
    /// The PWM module; `U2` is `PWM2`
    type Module: super::consts::Unsigned;
    /// The PWM submodule; `U3` for `PWM2_SM3`
    type Submodule: super::consts::Unsigned;
}

/// Prepare a PWM pin
///
/// # Safety
///
/// `prepare()` inherits all the unsafety of the `IOMUX` supertrait.
pub fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() };
    }
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! pwm {
    (module: $module:ty, submodule: $submodule:ty, alt: $alt:expr, pad: $pad:ty, output: $output:ty, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            type Output = $output;
            const DAISY: Option<Daisy> = $daisy;
            type Module = $module;
            type Submodule = $submodule;
        }
    };
}
