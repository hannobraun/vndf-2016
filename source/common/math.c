#include "math.h"


fix math_fromInt(int i)
{
	return i << 12;
}

fix math_add(fix a, fix b)
{
	return a + b;
}

fix math_sub(fix a, fix b)
{
	return a - b;
}

fix math_mod(fix a, fix b)
{
	return a % b;
}
