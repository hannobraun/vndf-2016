#include "math.h"


fix math_fromInt(int i)
{
	return i << FRAC_BITS;
}

long math_toLong(fix f)
{
	return f >> FRAC_BITS;
}

fix math_add(fix a, fix b)
{
	return a + b;
}

fix math_sub(fix a, fix b)
{
	return a - b;
}

fix math_mul(fix a, fix b)
{
	return a * b >> FRAC_BITS;
}

fix math_mod(fix a, fix b)
{
	return a % b;
}
