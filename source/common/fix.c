#include "fix.h"


fix fix_fromInt(int i)
{
	return i << FRAC_BITS;
}

long fix_toLong(fix f)
{
	return f >> FRAC_BITS;
}

fix fix_add(fix a, fix b)
{
	return a + b;
}

fix fix_sub(fix a, fix b)
{
	return a - b;
}

fix fix_mul(fix a, fix b)
{
	return a * b >> FRAC_BITS;
}

fix fix_div(fix a, fix b)
{
	return (a << FRAC_BITS) / b;
}

fix fix_mod(fix a, fix b)
{
	return a % b;
}
