#!/usr/bin/env python3

"""
Generate Daisy constants from an i.MX RT SVD file

Example usage: to create SPI-related Daisy constants,

    ./daisy.py path/to/imxrt.svd | grep LPSPI

Copy and paste the constants into a Rust module.
"""

import xml.etree.ElementTree as ET


def daisy_constant(iomuxc):
    base_address = int(iomuxc.find("./baseAddress").text, 16)

    for register in iomuxc.findall("./registers/register"):
        name = register.find("./name").text
        if "_SELECT_INPUT" in name:
            name = name.replace("_SELECT_INPUT", "")
            offset = int(register.find("./addressOffset").text, 16)
            address = base_address + offset
            for field in register.findall("./fields/field"):
                for values in field.findall("./enumeratedValues/enumeratedValue"):
                    pad = values.find("./name").text
                    if "_ALT" in pad:
                        pad = pad[:-5]  # _ALTx
                    daisy = int(values.find("./value").text, 16)
                    constant = f"pub const DAISY_{name}_{pad}: Daisy = Daisy::new({address:#010x} as *mut u32, {daisy});"
                    print(constant)


def search_iomuxces(path):
    tree = ET.parse(path)
    root = tree.getroot()
    iomuxc = root.find("./peripherals/peripheral[name='IOMUXC']")
    iomuxc_aon = root.find("./peripherals/peripheral[name='IOMUXC_AON']")
    iomuxc_lpsr = root.find("./peripherals/peripheral[name='IOMUXC_LPSR']")

    if iomuxc is not None:
        daisy_constant(iomuxc)
    if iomuxc_aon is not None:
        daisy_constant(iomuxc_aon)
    if iomuxc_lpsr is not None:
        daisy_constant(iomuxc_lpsr)

if __name__ == "__main__":
    import sys

    search_iomuxces(sys.argv[1])
