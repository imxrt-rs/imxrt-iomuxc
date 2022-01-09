//! uSDHC pin implementations

use super::sd_b0::*;
use crate::{config::PullKeeper::*, consts::*, usdhc::*, Daisy};

//
// uSDHC1
//
usdhc!(module: U1, alt: 0, pad: SD_B0_00, signal: Cmd, keeper: Some(Pullup100k), daisy: None);
usdhc!(module: U1, alt: 0, pad: SD_B0_01, signal: Clk, keeper: Some(Keeper), daisy: None);
usdhc!(module: U1, alt: 0, pad: SD_B0_02, signal: Data0, keeper: Some(Pullup100k), daisy: None);
usdhc!(module: U1, alt: 0, pad: SD_B0_03, signal: Data1, keeper: Some(Pullup100k), daisy: None);
usdhc!(module: U1, alt: 0, pad: SD_B0_04, signal: Data2, keeper: Some(Pullup100k), daisy: None);
usdhc!(module: U1, alt: 0, pad: SD_B0_05, signal: Data3, keeper: Some(Pulldown100k), daisy: None);

mod daisy {
    #![allow(unused)]
    use super::Daisy;
}
