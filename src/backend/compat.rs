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

use crate::pci::{PciDevice, PciEnumerationError};

use std::ffi::CStr;

use compat::{
    get_pci_stack,
    create_pci_device_stack,
    pci_device_stack_pop,
    free_pci_device_stack,
    pci_device_stack_t,
    pci_device_t,
    pci_error,
};

fn conv_c_pci_device(device: pci_device_t) -> PciDevice {
    PciDevice {
        domain: device.domain,
        bus: device.bus,
        device: device.device,
        function: device.function,
        label: unsafe { CStr::from_ptr(device.label) }.to_str().unwrap().to_string(),
        vendor_id: device.vendor_id,
        device_id: device.device_id,
        subsys_device_id: device.subsys_device_id,
        subsys_vendor_id: device.subsys_vendor_id,
        class: device.dev_class,
        subclass: device.subclass,
        programming_interface: device.programming_interface,
        revision_id: device.revision_id,
    }
}

fn pci_stack_to_list(stack: *mut pci_device_stack_t) -> Vec<PciDevice> {
    let mut list: Vec<PciDevice> = Vec::new();

    let stack_len = unsafe { (*stack).len };

    for i in 0..stack_len - 1 {
        let device = unsafe { pci_device_stack_pop(stack) };
        list.push(conv_c_pci_device(device));
    }

    unsafe {
        free_pci_device_stack(stack);
    }

    return list;
}

pub fn _get_pci_list() -> Result<Vec<PciDevice>, PciEnumerationError> {
    let mut stack: pci_device_stack_t;
    let mut res: pci_error;

    unsafe {
        stack = create_pci_device_stack();
        res = get_pci_stack(&mut stack) pci_error;
    };

    if res != 0 {
        return Err(PciEnumerationError::OsError) // TODO: create error enum on compat layer.
    }

    return Ok(pci_stack_to_list(&mut stack));
}

pub fn _get_pci_by_id(vendor: u16, device: u16) -> Result<PciDevice, PciEnumerationError> {
    todo!()
}