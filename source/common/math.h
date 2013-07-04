#include <stdint.h>

#ifndef COMMON_MATH_H
#define COMMON_MATH_H

typedef int64_t fix;


#define INT_BITS 52
#define FRAC_BITS sizeof(int64_t) * 8 - INT_BITS

#define PI 0x3243


fix math_fromInt(int i);
long math_toLong(fix f);

fix math_add(fix a, fix b);
fix math_sub(fix a, fix b);
fix math_mod(fix a, fix b);

#endif
