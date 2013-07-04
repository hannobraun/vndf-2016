#include <stdint.h>


typedef int64_t fix;


#define INT_BITS 52
#define FRAC_BITS sizeof(int64_t) * 8 - INT_BITS


fix math_fromInt(int i);

fix math_add(fix a, fix b);
fix math_sub(fix a, fix b);
fix math_mod(fix a, fix b);
