#include <stdio.h>
#include <assert.h>
#include "mathlib.h"

int main(void) {
    assert(add_ints(2,3) == 5);
    assert(sub_ints(5,3) == 2);
    assert(mul_ints(7,6) == 42);

    int64_t q;
    assert(div_ints(10,2,&q) && q == 5);
    assert(!div_ints(1,0,&q));  // division by zero should fail

    puts("All mathlib tests passed.");
    return 0;
}
