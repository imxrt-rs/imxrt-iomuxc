//! Pad configuration

use crate::Iomuxc;
use core::ptr;

/// Applies the configuration `config` for the supplied pad
///
/// `configure` lets you specify the pad's drive strength, speed, pull-up or pull-down
/// resistors, and other configurations. See [`Config`](struct.Config.html)
/// for possible configurations.
///
/// # Example
///
/// ```no_run
/// use imxrt_iomuxc::{configure, Config, OpenDrain, PullKeeper};
/// # use imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_03;
///
/// const CONFIG: Config = Config::zero()
///     .set_open_drain(OpenDrain::Enabled)
///     .set_pull_keeper(Some(PullKeeper::Pullup100k));
///
/// let mut pad = unsafe { GPIO_AD_B0_03::new() };
///
/// configure(&mut pad, CONFIG);
/// ```
#[inline(always)]
pub fn configure<I: Iomuxc>(pad: &mut I, config: Config) {
    // Safety: same justification as set_sion.
    unsafe {
        let cfg = ptr::read_volatile(pad.pad());
        let cfg = (cfg & !config.mask) | config.value;
        ptr::write_volatile(pad.pad(), cfg);
    }
}

const HYSTERESIS_SHIFT: u32 = 16;
const HYSTERESIS_MASK: u32 = 1 << HYSTERESIS_SHIFT;

/// The hysteresis (HYS) bit controls whether a pin acts as a Schmitt trigger,
/// which is a comparator remembering its last input state (hysteresis).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Hysteresis {
    Enabled = 1 << HYSTERESIS_SHIFT,
    Disabled = 0 << HYSTERESIS_SHIFT,
}

const PULLUPDOWN_SHIFT: u32 = 14;
const PULLUPDOWN_MASK: u32 = 0b11 << PULLUPDOWN_SHIFT;

/// Controls signals to select pull-up or pull-down internal resistance strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[deprecated(since = "0.2.0", note = "Use PullKeeper and Config::set_pull_keeper")]
pub enum PullUpDown {
    /// 100KOhm pull Down
    Pulldown100k = 0b00 << PULLUPDOWN_SHIFT,
    /// 47KOhm pull up
    Pullup47k = 0b01 << PULLUPDOWN_SHIFT,
    /// 100KOhm pull up
    Pullup100k = 0b10 << PULLUPDOWN_SHIFT,
    /// 22KOhm pull up
    Pullup22k = 0b11 << PULLUPDOWN_SHIFT,
}

const PULL_KEEP_SELECT_SHIFT: u32 = 13;
const PULL_KEEP_SELECT_MASK: u32 = 1 << PULL_KEEP_SELECT_SHIFT;

/// Control signal to enable internal pull-up/down resistors or pad keeper functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[deprecated(since = "0.2.0", note = "Use PullKeeper and Config::set_pull_keeper")]
pub enum PullKeepSelect {
    /// Keep the previous output value when the output driver is disabled.
    Keeper = 0 << PULL_KEEP_SELECT_SHIFT,
    /// Pull-up or pull-down (determined by [`PullUpDown`](struct.PullUpDown.html) flags).
    Pull = 1 << PULL_KEEP_SELECT_SHIFT,
}

const PULLKEEP_SHIFT: u32 = 12;
const PULLKEEP_MASK: u32 = 1 << PULLKEEP_SHIFT;

/// Enable or disable the pull / keeper functionality
///
/// When the pull/keeper is disabled, `PullKeepSelect` and `PullUpDown` have no functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[deprecated(since = "0.2.0", note = "Use PullKeeper and Config::set_pull_keeper")]
pub enum PullKeep {
    Enabled = 1 << PULLKEEP_SHIFT,
    Disabled = 0 << PULLKEEP_SHIFT,
}

/// Derives a pull/keep configuration from
/// the field-specific enums.
///
/// Used to define the public API.
#[allow(deprecated)]
const fn pull_keeper(select: PullKeepSelect, pull: Option<PullUpDown>) -> u32 {
    PULLKEEP_MASK
        | (select as u32)
        | match pull {
            None => 0u32,
            Some(pull) => pull as u32,
        }
}

const PULL_KEEPER_MASK: u32 = PULLKEEP_MASK | PULLUPDOWN_MASK | PULL_KEEP_SELECT_MASK;

/// The pull up, pull down, or keeper configuration.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[allow(deprecated)]
pub enum PullKeeper {
    /// 100KOhm pull **down**
    Pulldown100k = pull_keeper(PullKeepSelect::Pull, Some(PullUpDown::Pulldown100k)),
    /// 22KOhm pull **up**
    Pullup22k = pull_keeper(PullKeepSelect::Pull, Some(PullUpDown::Pullup22k)),
    /// 47KOhm pull **up**
    Pullup47k = pull_keeper(PullKeepSelect::Pull, Some(PullUpDown::Pullup47k)),
    /// 100KOhm pull **up**
    Pullup100k = pull_keeper(PullKeepSelect::Pull, Some(PullUpDown::Pullup100k)),
    /// Use the keeper, instead of a pull up or pull
    /// down resistor.
    ///
    /// From the reference manual,
    ///
    /// > A simple latch to hold the input value when OVDD is powered down, or the first inverter
    /// > is tri-stated. Input buffer’s keeper is always enabled for all the pads.
    Keeper = pull_keeper(PullKeepSelect::Keeper, None),
}

const OPENDRAIN_SHIFT: u32 = 11;
const OPENDRAIN_MASK: u32 = 1 << OPENDRAIN_SHIFT;

/// Open Drain Enable Field
///
/// If enabled, the output driver drives only logic 0. The drain of the
/// internal transistor is open. It means that logic 1 has to be driven by
/// an external component. This option is essential if connection between
/// the pad and an external component is bi-directional. If disabled, then
/// the output driver drives logic 1 and logic 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OpenDrain {
    Enabled = 1 << OPENDRAIN_SHIFT,
    Disabled = 0 << OPENDRAIN_SHIFT,
}

const SPEED_SHIFT: u32 = 6;
const SPEED_MASK: u32 = 0b11 << SPEED_SHIFT;

/// Sets electrical characteristics of a pin in a given frequency range
///
/// Speed is a selectable bit field that sets electrical characteristics of
/// a pin in a given frequency range. This field provides additional 2-bit
/// slew rate control. These options can either increase the output driver
/// current in the higher frequency range, or reduce the switching noise in
/// the lower frequency range.
///
/// The operational frequency on GPIO pads is dependent on slew rate (SRE),
/// speed (SPEED), and supply voltage (OVDD).
///
/// See Operating Frequency table in the GPIO block guide in the reference
/// manual for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Speed {
    Low = 0b00 << SPEED_SHIFT,
    Medium = 0b01 << SPEED_SHIFT,
    Fast = 0b10 << SPEED_SHIFT,
    Max = 0b11 << SPEED_SHIFT,
}

const DRIVE_STRENGTH_SHIFT: u32 = 3;
const DRIVE_STRENGTH_MASK: u32 = 0b111 << DRIVE_STRENGTH_SHIFT;

/// Drive strength
///
/// The drive strength enable (DSE) can be explained as series resistance between an ideal driver’s
/// output and its load. To achieve maximal transferred power, the impedance of the driver has to
/// match the load impedance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DriveStrength {
    Disabled = 0b000 << DRIVE_STRENGTH_SHIFT,
    /// 150 Ohm @ 3.3V, 260 Ohm@1.8V
    R0 = 0b001 << DRIVE_STRENGTH_SHIFT,
    /// R0 / 2
    R0_2 = 0b010 << DRIVE_STRENGTH_SHIFT,
    /// R0 / 3
    R0_3 = 0b011 << DRIVE_STRENGTH_SHIFT,
    /// R0 / 4
    R0_4 = 0b100 << DRIVE_STRENGTH_SHIFT,
    R0_5 = 0b101 << DRIVE_STRENGTH_SHIFT,
    R0_6 = 0b110 << DRIVE_STRENGTH_SHIFT,
    R0_7 = 0b111 << DRIVE_STRENGTH_SHIFT,
}

const SLEW_RATE_SHIFT: u32 = 0;
const SLEW_RATE_MASK: u32 = 1 << SLEW_RATE_SHIFT;

/// Slew Rate
///
/// This controls how fast the pin toggles between the two logic states.
/// Since rapidly changing states consume more power and generate spikes,
/// it should be enabled only when necessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SlewRate {
    Fast = 1 << SLEW_RATE_SHIFT,
    Slow = 0 << SLEW_RATE_SHIFT,
}

/// A configuration capable of compile-time, `const` configuration:
///
/// ```
/// use imxrt_iomuxc::{Config, SlewRate, Hysteresis};
///
/// const UART_TX_CONFIG: Config = Config::zero()
///     .set_slew_rate(SlewRate::Fast)
///     .set_hysteresis(Hysteresis::Enabled);
/// ```
///
/// Use [`configure()`](fn.configure.html) to set configurations to pads.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    value: u32,
    mask: u32,
}

impl Config {
    /// When we create the zero mask, we set all bits high. But,
    /// the highest usable bit in the pad register is bit 16. We
    /// can use a higher bit to represent if the config is generated
    /// from `zero()`, or if it was generated from `modify()`.
    const ZERO_BIT: u32 = 1 << 31;

    /// Create a `Config` that will zero any unspecified field
    ///
    /// This may *not* represent any register's reset value. A config that's
    /// created using `zero()` will have the effect of writing *all* fields
    /// to the register. Those that are not set explicitly set are written as zero.
    ///
    /// ```no_run
    /// # use imxrt_iomuxc::{Iomuxc, imxrt1060::gpio_ad_b0::GPIO_AD_B0_13};
    /// # let mut gpio_ad_b0_13 = unsafe { GPIO_AD_B0_13::new() };
    /// use imxrt_iomuxc::{
    ///     Config, configure, SlewRate,
    ///     Hysteresis, PullKeeper, DriveStrength
    /// };
    ///
    /// // Set a configuration
    /// configure(
    ///     &mut gpio_ad_b0_13,
    ///     Config::zero()
    ///         .set_slew_rate(SlewRate::Fast)
    ///         .set_drive_strength(DriveStrength::R0_7)
    /// );
    /// // DriveStrength::R0_7 as u32 | SlewRate::Fast as u32
    ///
    /// // Completely change that configuration
    /// configure(
    ///     &mut gpio_ad_b0_13,
    ///     Config::zero()
    ///         .set_hysteresis(Hysteresis::Enabled)
    ///         .set_pull_keeper(Some(PullKeeper::Pullup22k))
    /// );
    /// // Hysteresis::Enabled as u32 | PullKeeper::Pullup22k as u32
    /// //
    /// // Notice that SlewRate and DriveStrength were set to zero.
    /// ```
    pub const fn zero() -> Self {
        Config {
            value: 0u32,
            mask: 0xFFFF_FFFFu32,
        }
    }

    /// Create a `Config` that will only modify the specified fields
    ///
    /// Any field that is is *not* specified in the configuration will not be touched.
    ///
    /// ```no_run
    /// # use imxrt_iomuxc::{Iomuxc, imxrt1060::gpio_ad_b0::GPIO_AD_B0_13};
    /// # let mut gpio_ad_b0_13 = unsafe { GPIO_AD_B0_13::new() };
    /// use imxrt_iomuxc::{Config, configure, SlewRate, DriveStrength, Hysteresis};
    ///
    /// // Assume a starting value in the register...
    /// configure(
    ///     &mut gpio_ad_b0_13,
    ///     Config::zero()
    ///         .set_slew_rate(SlewRate::Fast)
    ///         .set_drive_strength(DriveStrength::R0_7)
    /// );
    /// // DriveStrength::R0_7 as u32 | SlewRate::Fast as u32
    ///
    /// // Now change the slew rate, and add hysteresis
    /// configure(
    ///     &mut gpio_ad_b0_13,
    ///     Config::modify()
    ///         .set_slew_rate(SlewRate::Slow)
    ///         .set_hysteresis(Hysteresis::Enabled)
    /// );
    /// // DriveStrength::R0_7 as u32 | Hysteresis::Enabled as u32
    /// //
    /// // Notice that the DriveStrength field is preserved.
    /// ```
    pub const fn modify() -> Self {
        Config {
            value: 0u32,
            mask: 0u32,
        }
    }

    /// Returns `true` if this `Config` was created using [`zero()`](struct.Config.html#method.zero), meaning that it will
    /// zero any unspecified fields. If `false`, this config was created using [`modify()`](struct.Config.html#method.modify),
    /// which will not touch unspecified fields.
    ///
    /// ```
    /// use imxrt_iomuxc::Config;
    ///
    /// assert!(Config::zero().is_zero());
    /// assert!(!Config::modify().is_zero());
    /// ```
    pub const fn is_zero(&self) -> bool {
        self.mask & Self::ZERO_BIT != 0
    }

    /// Set the hysteresis bit
    pub const fn set_hysteresis(mut self, hys: Hysteresis) -> Self {
        self.value = (self.value & !HYSTERESIS_MASK) | (hys as u32);
        self.mask |= HYSTERESIS_MASK;
        self
    }

    /// Set the pull up / pull down / keeper configuration.
    ///
    /// A `None` value disables the pull / keeper function.
    pub const fn set_pull_keeper(mut self, pk: Option<PullKeeper>) -> Self {
        let pk = match pk {
            None => 0u32,
            Some(pk) => pk as u32,
        };
        self.value = (self.value & !PULL_KEEPER_MASK) | (pk as u32);
        self.mask |= PULL_KEEPER_MASK;
        self
    }

    /// Set the pull-up / pull-down value
    #[deprecated(since = "0.2.0", note = "Use PullKeeper and Config::set_pull_keeper")]
    #[allow(deprecated)]
    pub const fn set_pullupdown(mut self, pud: PullUpDown) -> Self {
        self.value = (self.value & !PULLUPDOWN_MASK) | (pud as u32);
        self.mask |= PULLUPDOWN_MASK;
        self
    }

    /// Set the the pull-up / pull-down or keeper selection bit
    #[deprecated(since = "0.2.0", note = "Use PullKeeper and Config::set_pull_keeper")]
    #[allow(deprecated)]
    pub const fn set_pull_keep_select(mut self, pke: PullKeepSelect) -> Self {
        self.value = (self.value & !PULL_KEEP_SELECT_MASK) | (pke as u32);
        self.mask |= PULL_KEEP_SELECT_MASK;
        self
    }

    /// Set the flag that enables the keeper or pull-up / pull-down configuration
    #[deprecated(since = "0.2.0", note = "Use PullKeeper and Config::set_pull_keeper")]
    #[allow(deprecated)]
    pub const fn set_pull_keep(mut self, pk: PullKeep) -> Self {
        self.value = (self.value & !PULLKEEP_MASK) | (pk as u32);
        self.mask |= PULLKEEP_MASK;
        self
    }

    /// Set the open drain value
    pub const fn set_open_drain(mut self, od: OpenDrain) -> Self {
        self.value = (self.value & !OPENDRAIN_MASK) | (od as u32);
        self.mask |= OPENDRAIN_MASK;
        self
    }

    /// Set the pin speed
    pub const fn set_speed(mut self, speed: Speed) -> Self {
        self.value = (self.value & !SPEED_MASK) | (speed as u32);
        self.mask |= SPEED_MASK;
        self
    }

    /// Set the drive strength
    pub const fn set_drive_strength(mut self, dse: DriveStrength) -> Self {
        self.value = (self.value & !DRIVE_STRENGTH_MASK) | (dse as u32);
        self.mask |= DRIVE_STRENGTH_MASK;
        self
    }

    /// Set the slew rate
    pub const fn set_slew_rate(mut self, sre: SlewRate) -> Self {
        self.value = (self.value & !SLEW_RATE_MASK) | (sre as u32);
        self.mask |= SLEW_RATE_MASK;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Pad(u32);

    const PAD_ALL_HIGH: Pad = Pad(0x0001_FFFFu32);

    /// The high bits represent the valid fields in pad registers.
    const PAD_BITMASK: u32 = 0b1_1111_1000_1111_1001u32;

    impl crate::private::Sealed for Pad {}

    unsafe impl Iomuxc for Pad {
        fn mux(&mut self) -> *mut u32 {
            panic!("Nothing calls mux() in these tests");
        }
        fn pad(&mut self) -> *mut u32 {
            &mut self.0 as *mut _
        }
    }

    #[test]
    fn zero() {
        let mut pad = PAD_ALL_HIGH;
        configure(&mut pad, Config::zero());
        assert_eq!(pad.0, 0);
    }

    #[test]
    fn zero_set_all() {
        let mut pad = PAD_ALL_HIGH;
        const CONFIG: Config = Config::zero()
            .set_hysteresis(Hysteresis::Enabled)
            .set_pull_keeper(Some(PullKeeper::Pullup22k))
            .set_open_drain(OpenDrain::Enabled)
            .set_speed(Speed::Max)
            .set_drive_strength(DriveStrength::R0_7)
            .set_slew_rate(SlewRate::Fast);

        configure(&mut pad, CONFIG);

        assert_eq!(pad.0, PAD_BITMASK);
    }

    #[test]
    fn modify_clear_all() {
        let mut pad = Pad(PAD_BITMASK);
        const CONFIG: Config = Config::modify()
            .set_hysteresis(Hysteresis::Disabled)
            .set_pull_keeper(None)
            .set_open_drain(OpenDrain::Disabled)
            .set_speed(Speed::Low)
            .set_drive_strength(DriveStrength::Disabled)
            .set_slew_rate(SlewRate::Slow);

        configure(&mut pad, CONFIG);

        assert_eq!(pad.0, 0);
    }

    #[test]
    fn pull_keeper_none() {
        let mut pad = Pad(0);
        configure(&mut pad, Config::zero().set_pull_keeper(None));
        assert_eq!(pad.0, 0);
    }

    #[test]
    fn pull_keeper_keeper() {
        let mut pad = Pad(0);
        configure(
            &mut pad,
            Config::zero().set_pull_keeper(Some(PullKeeper::Keeper)),
        );
        assert_eq!(pad.0, 1 << 12);
    }

    #[test]
    fn pull_keeper_pullupdown() {
        struct ConfigToField {
            config: PullKeeper,
            value: u32,
        }

        const TESTS: [ConfigToField; 4] = [
            ConfigToField {
                config: PullKeeper::Pulldown100k,
                value: 0 << 14,
            },
            ConfigToField {
                config: PullKeeper::Pullup100k,
                value: 2 << 14,
            },
            ConfigToField {
                config: PullKeeper::Pullup22k,
                value: 3 << 14,
            },
            ConfigToField {
                config: PullKeeper::Pullup47k,
                value: 1 << 14,
            },
        ];

        for test in TESTS {
            let mut pad = Pad(0);
            configure(&mut pad, Config::zero().set_pull_keeper(Some(test.config)));
            assert_eq!(pad.0, 1 << 12 | 1 << 13 | test.value);
        }
    }
}

/// ```rust
/// use imxrt_iomuxc::{Config, PullKeeper};
///
/// const CONFIG: Config = Config::zero().set_pull_keeper(Some(PullKeeper::Keeper));
#[cfg(doctest)]
struct PullKeeperConstant;
