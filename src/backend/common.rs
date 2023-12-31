// Copyright (c) 2023 NamedNeon. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use core::fmt;
use std::{num::ParseIntError, fs::{DirEntry, read_to_string}};
use std::io::ErrorKind;
use std::fmt::Display;

#[derive(Debug)]
pub enum PciEnumerationError {
    OsError,
    GenericIoError(std::io::Error),
    ReadDirectory,
    NotFound,
    PermissionDenied,
    ParseInt(ParseIntError),
}

// Convert IO errors to PCI enumeration errors.
impl From<std::io::Error> for PciEnumerationError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => PciEnumerationError::NotFound,
            ErrorKind::PermissionDenied => PciEnumerationError::PermissionDenied,
            _ => PciEnumerationError::GenericIoError(err),
        }
    }
}

// Convert integer parsing error into PCI enumeration error.
impl From<ParseIntError> for PciEnumerationError {
    fn from(err: ParseIntError) -> Self {
        PciEnumerationError::ParseInt(err)
    }
}

// Define a PCI device as its component fields
#[derive(Debug, Clone)]
pub struct PciDevice {
    pub domain: u32,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub label: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsys_device_id: u16,
    pub subsys_vendor_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub programming_interface: u8,
    pub revision_id: u8,
}



// ############################## Begin hex helper functions ##############################
pub(crate) fn ox_hex_string_to_u8(input_string: &str) -> Result<u8, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u8::from_str_radix(input_string, 16)
}

pub(crate) fn ox_hex_string_to_u16(input_string: &str) -> Result<u16, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u16::from_str_radix(input_string, 16)
}

pub(crate) fn ox_hex_string_to_u32(input_string: &str) -> Result<u32, ParseIntError> {
    let input_string = if input_string.starts_with("0x") {
        &input_string[2..]
    } else {
        input_string
    }.trim();
    u32::from_str_radix(input_string, 16)
}
// ############################## End hex helper functions ##############################



// ############################## Begin attribute hepler functions ##############################
pub(crate) fn get_pci_device_attribute_u8(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u8, PciEnumerationError> {
    let dir_usable = match dir {
        Ok(f) => f,
        Err(_) => {
            return Err(PciEnumerationError::ReadDirectory);
        }
    };

    let file_contents = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute))?;
    let decoded_number = ox_hex_string_to_u8(&file_contents)?;

    Ok(decoded_number)
}

pub(crate) fn get_pci_device_attribute_u16(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u16, PciEnumerationError> {
    let dir_usable = match dir {
        Ok(f) => f,
        Err(_) => {
            return Err(PciEnumerationError::ReadDirectory);
        }
    };

    let file_contents = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute))?;
    let decoded_number = ox_hex_string_to_u16(&file_contents)?;

    Ok(decoded_number)
}

pub(crate) fn get_pci_device_attribute_u32(dir: &Result<DirEntry, std::io::Error>, attribute: &str) -> Result<u32, PciEnumerationError> {
    let dir_usable = match dir {
        Ok(f) => f,
        Err(_) => {
            return Err(PciEnumerationError::ReadDirectory);
        }
    };

    let file_contents = read_to_string(format!("{}/{}", dir_usable.path().to_string_lossy(), attribute))?;
    let decoded_number = ox_hex_string_to_u32(&file_contents)?;

    Ok(decoded_number)
}
// ############################## End attribute helper functions ##############################



impl Display for PciDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}:{:02x}:{:02x}.{:x} VID={:04x} DID={:04x} SVID={:04x} SDID={:02x} Class={:x} Subclass={:x} PIF={:x} Rev={:x}", self.domain, self.bus, self.device, self.function, self.vendor_id, self.device_id, self.subsys_vendor_id, self.subsys_device_id, self.class, self.subclass, self.programming_interface, self.revision_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::common::{ox_hex_string_to_u16, ox_hex_string_to_u32, ox_hex_string_to_u8};

    #[test]
    fn test_hex_decoding() {
        // Test to make sure every bit is recognized using the highest possible integer!
        assert_eq!(ox_hex_string_to_u8("0xFF"), Ok(255));
        assert_eq!(ox_hex_string_to_u16("0xFFFF"), Ok(65535));
        assert_eq!(ox_hex_string_to_u32("0xFFFFFFFF"), Ok(4294967295));
    }
}