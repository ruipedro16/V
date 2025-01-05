#include <assert.h>
#include <limits.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

// The XOR operation (^) with 1 flips the least significant bit (LSB) of a number.
// If the LSB is 0 (even number), XOR with 1 will change it to 1, effectively adding 1 to the number.
// If the LSB is 1 (odd number), XOR with 1 will change it to 0, effectively subtracting 1 from the number.
//
// Therefore, for any integer x:
// - If x is even, x ^ 1 results in x + 1.
// - If x is odd,  x ^ 1 results in x - 1.
// 
static bool is_odd (uint32_t num) { return num % 2 != 0; }
static bool is_even(uint32_t num) { return num % 2 == 0; }

int main() {
    uint32_t upper_bound = (uint32_t)pow(2, 20);  // height h of the tree
    for (uint32_t i = 0; i < upper_bound /* UINT32_MAX */; i++) {
        printf("%u/%u\n", i + 1, upper_bound);
        if (is_even(i)) { assert(i ^ 1 == i + 1); }
        if (is_odd(i))  { assert(i ^ 1 == i - 1); }
    }
    return EXIT_SUCCESS;
}