#include "mathlib.h"

int64_t add_ints(int64_t a, int64_t b) {
    return a + b;
}

int64_t sub_ints(int64_t a, int64_t b) {
    return a - b;
}

int64_t mul_ints(int64_t a, int64_t b) {
    return a * b;
}

bool div_ints(int64_t a, int64_t b, int64_t *result) {
    if (b == 0) {
        return false;
    }
    *result = a / b;
    return true;
}
