//! SEMC pad configurations

/// A SEMC signal
pub trait Signal: private::Sealed {}

pub enum Data0 {}
pub enum Data1 {}
pub enum Data2 {}
pub enum Data3 {}
pub enum Data4 {}
pub enum Data5 {}
pub enum Data6 {}
pub enum Data7 {}
pub enum Data8 {}
pub enum Data9 {}
pub enum Data10 {}
pub enum Data11 {}
pub enum Data12 {}
pub enum Data13 {}
pub enum Data14 {}
pub enum Data15 {}
pub enum Dm0 {}
pub enum Dm1 {}
pub enum Addr0 {}
pub enum Addr1 {}
pub enum Addr2 {}
pub enum Addr3 {}
pub enum Addr4 {}
pub enum Addr5 {}
pub enum Addr6 {}
pub enum Addr7 {}
pub enum Addr8 {}
pub enum Addr9 {}
pub enum Addr10 {}
pub enum Addr11 {}
pub enum Addr12 {}
pub enum Ba0 {}
pub enum Ba1 {}
pub enum Cas {}
pub enum Ras {}
pub enum We{}
pub enum Cke {}
pub enum Clk {}
pub enum Cs0 {}
pub enum Csx0 {}
pub enum Csx1 {}
pub enum Csx2 {}
pub enum Csx3 {}
pub enum Rdy {}
pub enum Dqs {}

impl Signal for Data0 {}
impl Signal for Data1 {}
impl Signal for Data2 {}
impl Signal for Data3 {}
impl Signal for Data4 {}
impl Signal for Data5 {}
impl Signal for Data6 {}
impl Signal for Data7 {}
impl Signal for Data8 {}
impl Signal for Data9 {}
impl Signal for Data10 {}
impl Signal for Data11 {}
impl Signal for Data12 {}
impl Signal for Data13 {}
impl Signal for Data14 {}
impl Signal for Data15 {}
impl Signal for Dm0 {}
impl Signal for Dm1 {}
impl Signal for Addr0 {}
impl Signal for Addr1 {}
impl Signal for Addr2 {}
impl Signal for Addr3 {}
impl Signal for Addr4 {}
impl Signal for Addr5 {}
impl Signal for Addr6 {}
impl Signal for Addr7 {}
impl Signal for Addr8 {}
impl Signal for Addr9 {}
impl Signal for Addr10 {}
impl Signal for Addr11 {}
impl Signal for Addr12 {}
impl Signal for Ba0 {}
impl Signal for Ba1 {}
impl Signal for Cas {}
impl Signal for Ras {}
impl Signal for We{}
impl Signal for Cke {}
impl Signal for Clk {}
impl Signal for Cs0 {}
impl Signal for Csx0 {}
impl Signal for Csx1 {}
impl Signal for Csx2 {}
impl Signal for Csx3 {}
impl Signal for Rdy {}
impl Signal for Dqs {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::Data0 {}
    impl Sealed for super::Data1 {}
    impl Sealed for super::Data2 {}
    impl Sealed for super::Data3 {}
    impl Sealed for super::Data4 {}
    impl Sealed for super::Data5 {}
    impl Sealed for super::Data6 {}
    impl Sealed for super::Data7 {}
    impl Sealed for super::Data8 {}
    impl Sealed for super::Data9 {}
    impl Sealed for super::Data10 {}
    impl Sealed for super::Data11 {}
    impl Sealed for super::Data12 {}
    impl Sealed for super::Data13 {}
    impl Sealed for super::Data14 {}
    impl Sealed for super::Data15 {}
    impl Sealed for super::Dm0 {}
    impl Sealed for super::Dm1 {}
    impl Sealed for super::Addr0 {}
    impl Sealed for super::Addr1 {}
    impl Sealed for super::Addr2 {}
    impl Sealed for super::Addr3 {}
    impl Sealed for super::Addr4 {}
    impl Sealed for super::Addr5 {}
    impl Sealed for super::Addr6 {}
    impl Sealed for super::Addr7 {}
    impl Sealed for super::Addr8 {}
    impl Sealed for super::Addr9 {}
    impl Sealed for super::Addr10 {}
    impl Sealed for super::Addr11 {}
    impl Sealed for super::Addr12 {}
    impl Sealed for super::Ba0 {}
    impl Sealed for super::Ba1 {}
    impl Sealed for super::Cas {}
    impl Sealed for super::Ras {}
    impl Sealed for super::We{}
    impl Sealed for super::Cke {}
    impl Sealed for super::Clk {}
    impl Sealed for super::Cs0 {}
    impl Sealed for super::Csx0 {}
    impl Sealed for super::Csx1 {}
    impl Sealed for super::Csx2 {}
    impl Sealed for super::Csx3 {}
    impl Sealed for super::Rdy {}
    impl Sealed for super::Dqs {}
}

/// A SEMC pin
pub trait Pin: super::Iomuxc {
    const ALT: u32;
    type Signal: Signal;
    type Module: super::consts::Unsigned;
}

/// Prepare a SEMC pin
pub fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::clear_sion(pin);
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! semc {
    (module: $module:ty, alt: $alt:expr, pad: $pad:ty, signal: $signal:ty) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            type Signal = $signal;
            type Module = $module;
        }
    };
}
