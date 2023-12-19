//! SAI / I2S pin implementation

use super::{gpio::*, gpio_sd::*};
use crate::{consts::*, sai::*, Daisy};

/// SAI1 multiplexed TX / RX pin
///
/// Implements both `RxDataSignal` and `TxDataSignal`.
pub enum TxData1RxData1 {}

impl Signal for TxData1RxData1 {}
impl TxDataSignal for TxData1RxData1 {
    type Index = U1;
}
impl RxDataSignal for TxData1RxData1 {
    type Index = U1;
}
impl private::Sealed for TxData1RxData1 {}

//
// SAI1
//

sai!(module: U1, alt: 0, pad: GPIO_08, signal: Mclk, daisy: None);
sai!(module: U1, alt: 0, pad: GPIO_01, signal: RxBclk, daisy: None);
sai!(module: U1, alt: 0, pad: GPIO_03, signal: RxData, daisy: None);
sai!(module: U1, alt: 0, pad: GPIO_02, signal: RxSync, daisy: None);
sai!(module: U1, alt: 0, pad: GPIO_06, signal: TxBclk, daisy: None);
sai!(module: U1, alt: 0, pad: GPIO_04, signal: TxData, daisy: None);
sai!(module: U1, alt: 0, pad: GPIO_05, signal: TxData1RxData1, daisy: None);
sai!(module: U1, alt: 0, pad: GPIO_07, signal: TxSync, daisy: None);

//
// SAI3
//

sai!(module: U3, alt: 1, pad: GPIO_00,   signal: Mclk, daisy: None);
sai!(module: U3, alt: 1, pad: GPIO_SD_13, signal: RxBclk, daisy: None);
sai!(module: U3, alt: 1, pad: GPIO_SD_03, signal: RxData, daisy: None);
sai!(module: U3, alt: 1, pad: GPIO_SD_04, signal: RxSync, daisy: None);
sai!(module: U3, alt: 1, pad: GPIO_SD_01, signal: TxBclk, daisy: None);
sai!(module: U3, alt: 1, pad: GPIO_SD_02, signal: TxData, daisy: None);
sai!(module: U3, alt: 1, pad: GPIO_SD_00, signal: TxSync, daisy: None);
