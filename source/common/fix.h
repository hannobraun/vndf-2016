#include <stdint.h>

#ifndef COMMON_fix_H
#define COMMON_fix_H

typedef int64_t fix;


#define INT_BITS 52
#define FRAC_BITS sizeof(int64_t) * 8 - INT_BITS

#define PI 0x3243


fix fix_fromInt(int i);
long fix_toLong(fix f);

fix fix_add(fix a, fix b);
fix fix_sub(fix a, fix b);
fix fix_mul(fix a, fix b);
fix fix_div(fix a, fix b);
fix fix_mod(fix a, fix b);

#endif
