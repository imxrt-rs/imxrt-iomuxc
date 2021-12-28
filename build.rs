#![allow(unused)]
use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "imxrt1010")]
    imxrt1010(fs::File::create(out_dir.join("imxrt1010.rs"))?)?;

    #[cfg(feature = "imxrt1060")]
    imxrt1060(fs::File::create(out_dir.join("imxrt1060.rs"))?)?;

    Ok(())
}

#[cfg(feature = "imxrt1010")]
fn imxrt1010<W: io::Write>(mut pads_rs: W) -> io::Result<()> {
    use imxrt_iomuxc_build as build;

    let gpio_ad = build::PadRange::new("GPIO_AD", 0..15);
    let gpio_sd = build::PadRange::new("GPIO_SD", 0..15);
    let gpio = build::PadRange::new("GPIO", 0..14);

    build::write_pads(&mut pads_rs, vec![&gpio_ad, &gpio_sd, &gpio])?;
    build::write_impl_gpio_pins(
        &mut pads_rs,
        vec![
            // GPIO1
            build::ImplGpioPin::from_range(&gpio_ad, build::GpioRange::no_offset(1, 5)),
            // GPIO2
            build::ImplGpioPin::from_range(&gpio_sd, build::GpioRange::no_offset(2, 5)),
            // GPIO3
            build::ImplGpioPin::from_range(&gpio, build::GpioRange::no_offset(3, 5)),
        ],
    )?;
    Ok(())
}

#[cfg(feature = "imxrt1060")]
fn imxrt1060<W: io::Write>(mut pads_rs: W) -> io::Result<()> {
    use imxrt_iomuxc_build as build;

    let gpio_emc = build::PadRange::new("GPIO_EMC", 0..42);
    let gpio_ad_b0 = build::PadRange::new("GPIO_AD_B0", 0..16);
    let gpio_ad_b1 = build::PadRange::new("GPIO_AD_B1", 0..16);
    let gpio_b0 = build::PadRange::new("GPIO_B0", 0..16);
    let gpio_b1 = build::PadRange::new("GPIO_B1", 0..16);
    let gpio_sd_b0 = build::PadRange::new("GPIO_SD_B0", 0..6);
    let gpio_sd_b1 = build::PadRange::new("GPIO_SD_B1", 0..12);

    build::write_pads(
        &mut pads_rs,
        vec![
            &gpio_emc,
            &gpio_ad_b0,
            &gpio_ad_b1,
            &gpio_b0,
            &gpio_b1,
            &gpio_sd_b0,
            &gpio_sd_b1,
        ],
    )?;
    build::write_impl_gpio_pins(
        &mut pads_rs,
        vec![
            // GPIO1
            build::ImplGpioPin::from_range(&gpio_ad_b0, build::GpioRange::no_offset(1, 5)),
            build::ImplGpioPin::from_range(
                &gpio_ad_b1,
                build::GpioRange {
                    module: 1,
                    offset: 16,
                    alt: 5,
                },
            ),
            // GPIO2
            build::ImplGpioPin::from_range(&gpio_b0, build::GpioRange::no_offset(2, 5)),
            build::ImplGpioPin::from_range(
                &gpio_b1,
                build::GpioRange {
                    module: 2,
                    offset: 16,
                    alt: 5,
                },
            ),
            // GPIO3
            build::ImplGpioPin::from_range(&gpio_sd_b1, build::GpioRange::no_offset(3, 5)),
            build::ImplGpioPin::from_range(
                &gpio_sd_b0,
                build::GpioRange {
                    module: 3,
                    offset: 12,
                    alt: 5,
                },
            ),
            build::ImplGpioPin::from_range(
                &gpio_emc.skip(32),
                build::GpioRange {
                    module: 3,
                    offset: 18,
                    alt: 5,
                },
            ),
            // GPIO4
            build::ImplGpioPin::from_range(&gpio_emc.take(32), build::GpioRange::no_offset(4, 5)),
        ],
    )?;
    Ok(())
}
