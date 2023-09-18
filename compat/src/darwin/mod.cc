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

#include <IOKit/IOKitLib.h>
#include <IOKit/IOKitKeys.h>
#include <Availability.h>

#include <vector>

#include "../api.hpp"

// Apple annoyingly decided to rename kIOMasterPortDefault (the default mach port) in macOS 12.
#if __MAC_OS_X_VERSION_MAX_ALLOWED >= MAC_OS_VERSION_12_0
#define DARWIN_IO_PORT kIOMainPortDefault
#else
#define DARWIN_IO_PORT kIOMasterPortDefault
#endif

extern "C" int get_pci_list(pci_device_stack_t* out) {
    PCIDeviceStack stack;

    io_iterator_t iterator = IO_OBJECT_NULL;
    kern_return_t services = IORegistryEntryCreateIterator(DARWIN_IO_PORT, kIODeviceTreePlane, kIORegistryIterateRecursively, &iterator);

    if(!services) {
        return -1;
    }

    while(true) {
        io_object_t object = IOIteratorNext(iterator);
        if(object == 0) {
            break;
        }

    }

    IOObjectRelease(iterator);

    *out = stack.raw();

    return 0;
}

extern "C" pci_device_t get_pci_by_id(uint16_t vendor, uint16_t device) {
    pci_device_t result;



    return result;
}
