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

#include <CoreFoundation/CFDictionary.h>
#include <CoreFoundation/CFData.h>

#include <IOKit/IOKitLib.h>

#include <Availability.h>

#include "../api.h"

// Apple annoyingly decided to rename kIOMasterPortDefault (the default mach port) in macOS 12.
#if __MAC_OS_X_VERSION_MAX_ALLOWED >= MAC_OS_VERSION_12_0
#define DARWIN_IO_PORT kIOMainPortDefault
#else
#define DARWIN_IO_PORT kIOMasterPortDefault
#endif

#define CF_IS_DATA(x) CFGetTypeID(x) == CFDataGetTypeID()

#define kIOPCIDevice "IOPCIDevice"

int get_pci_stack(pci_device_stack_t* out) {
    pci_device_stack_t stack = create_pci_device_stack();

    io_iterator_t iterator = IO_OBJECT_NULL;
    // Get services matching "IOPCIDevice"
    kern_return_t services = IOServiceGetMatchingServices(DARWIN_IO_PORT, IOServiceNameMatching(kIOPCIDevice), &iterator);

    if(services != kIOReturnSuccess) {
        goto error;
    }

    io_object_t dev;

    while((dev = IOIteratorNext(iterator))) {
        CFMutableDictionaryRef service_dictionary;
        // Put properties of matched service into a dictionary.
        if(IORegistryEntryCreateCFProperties(dev, &service_dictionary, kCFAllocatorDefault, kNilOptions) != kIOReturnSuccess) {
            IOObjectRelease(dev);
            continue;
        }

        CFDataRef vendor_id = (CFDataRef) CFDictionaryGetValue(service_dictionary, "vendor-id");
        CFDataRef device_id = (CFDataRef) CFDictionaryGetValue(service_dictionary, "device-id");
        CFDataRef revision_id = (CFDataRef) CFDictionaryGetValue(service_dictionary, "revision-id");
        CFDataRef class_code = (CFDataRef) CFDictionaryGetValue(service_dictionary, "class-code");

        // TODO: for the love of god there must be a better way to do this.
        if(
            CF_IS_DATA(vendor_id) &&
            CF_IS_DATA(device_id) &&
            CF_IS_DATA(revision_id) &&
            CF_IS_DATA(class_code)) {

            pci_device_t pci_device;

            uint8_t vendor_id_bytes[4];
            CFDataGetBytes(vendor_id, CFRangeMake(0, 3), vendor_id_bytes);
            pci_device.vendor_id = vendor_id_bytes[3] << 8 | vendor_id_bytes[2];

            pci_device_stack_push(&stack, pci_device);
        } else {
            IOObjectRelease(dev);
            continue;
        }

        IOObjectRelease(dev);
        CFRelease(service_dictionary);
    }

    IOObjectRelease(iterator);

    *out = stack;

    return 0;

error: // NOTE: yes i'm using a goto. cry about it.
    IOObjectRelease(iterator);
    return -1; // todo: error code
}

int get_pci_by_id(pci_device_t* dev, uint16_t vendor, uint16_t device) {
    pci_device_t result;

    *dev = result;

    return 0;
}
