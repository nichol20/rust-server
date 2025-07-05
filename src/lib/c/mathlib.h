#ifndef MATHLIB_H
#define MATHLIB_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Adds two 64-bit integers.
 *
 * @param a  First addend.
 * @param b  Second addend.
 * @return   Sum of a and b.
 */
int64_t add_ints(int64_t a, int64_t b);

/**
 * @brief Subtracts two 64-bit integers.
 *
 * @param a  Minuend.
 * @param b  Subtrahend.
 * @return   a − b.
 */
int64_t sub_ints(int64_t a, int64_t b);

/**
 * @brief Multiplies two 64-bit integers.
 *
 * @param a  First factor.
 * @param b  Second factor.
 * @return   Product of a and b.
 */
int64_t mul_ints(int64_t a, int64_t b);

/**
 * @brief Safely divides two 64-bit integers.
 *
 * @param a       Dividend.
 * @param b       Divisor.
 * @param result  Pointer to store quotient if b≠0.
 * @return        true if division succeeded (b≠0), false otherwise.
 */
bool div_ints(int64_t a, int64_t b, int64_t *result);

#ifdef __cplusplus
}
#endif

#endif /* MATHLIB_H */
