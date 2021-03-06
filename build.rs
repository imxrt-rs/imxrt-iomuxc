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

    let ad = build::PadRange::new("AD", 0..16);
    let sd = build::PadRange::new("SD", 0..16);
    let gpio = build::PadRange::new("GPIO", 0..16);

    build::write_pads(&mut pads_rs, vec![&ad, &sd, &gpio])?;
    build::write_impl_gpio_pins(
        &mut pads_rs,
        vec![
            // GPIO1
            build::ImplGpioPin::from_range(&ad, build::GpioRange::no_offset(1, 5)),
            // GPIO2
            build::ImplGpioPin::from_range(&sd, build::GpioRange::no_offset(2, 5)),
            // GPIO3
            build::ImplGpioPin::from_range(&gpio, build::GpioRange::no_offset(3, 5)),
        ],
    )?;
    Ok(())
}

#[cfg(feature = "imxrt1060")]
fn imxrt1060<W: io::Write>(mut pads_rs: W) -> io::Result<()> {
    use imxrt_iomuxc_build as build;

    let emc = build::PadRange::new("EMC", 0..42);
    let ad_b0 = build::PadRange::new("AD_B0", 0..16);
    let ad_b1 = build::PadRange::new("AD_B1", 0..16);
    let b0 = build::PadRange::new("B0", 0..16);
    let b1 = build::PadRange::new("B1", 0..16);
    let sd_b0 = build::PadRange::new("SD_B0", 0..6);
    let sd_b1 = build::PadRange::new("SD_B1", 0..12);

    build::write_pads(
        &mut pads_rs,
        vec![&emc, &ad_b0, &ad_b1, &b0, &b1, &sd_b0, &sd_b1],
    )?;
    build::write_impl_gpio_pins(
        &mut pads_rs,
        vec![
            // GPIO1
            build::ImplGpioPin::from_range(&ad_b0, build::GpioRange::no_offset(1, 5)),
            build::ImplGpioPin::from_range(
                &ad_b1,
                build::GpioRange {
                    module: 1,
                    offset: 16,
                    alt: 5,
                },
            ),
            // GPIO2
            build::ImplGpioPin::from_range(&b0, build::GpioRange::no_offset(2, 5)),
            build::ImplGpioPin::from_range(
                &b1,
                build::GpioRange {
                    module: 2,
                    offset: 16,
                    alt: 5,
                },
            ),
            // GPIO3
            build::ImplGpioPin::from_range(&sd_b1, build::GpioRange::no_offset(3, 5)),
            build::ImplGpioPin::from_range(
                &sd_b0,
                build::GpioRange {
                    module: 3,
                    offset: 12,
                    alt: 5,
                },
            ),
            build::ImplGpioPin::from_range(
                &emc.skip(32),
                build::GpioRange {
                    module: 3,
                    offset: 18,
                    alt: 5,
                },
            ),
            // GPIO4
            build::ImplGpioPin::from_range(&emc.take(32), build::GpioRange::no_offset(4, 5)),
        ],
    )?;
    Ok(())
}
