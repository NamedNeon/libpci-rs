#include "../api.hpp"

#include "test.h"

int main() {
    auto stack = new PCIDeviceStack();

    pci_device_t item1 = {};
    pci_device_t item2 = {};
    pci_device_t item3 = {};

    item1.dev_class = 12;
    item2.dev_class = 34;
    item3.dev_class = 250;

    stack->push(item1);
    stack->push(item2);
    stack->push(item3);

    pci_device_t item3_pop = stack->pop();
    pci_device_t item2_pop = stack->pop();
    pci_device_t item1_pop = stack->pop();

    TEST_ASSERT((item3.dev_class == item3_pop.dev_class), "popped item 3 test attribute did not equal original");
    TEST_ASSERT((item2.dev_class == item2_pop.dev_class), "popped item 2 test attribute did not equal original");
    TEST_ASSERT((item1.dev_class == item1_pop.dev_class), "popped item 1 test attribute did not equal original");

    delete stack;

    return 0;
}