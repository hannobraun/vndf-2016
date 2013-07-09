#include "vec.h"


#include <math.h>


vec2 vec_add(vec2 a, vec2 b)
{
	return (vec2){a.x + b.x, a.y + b.y};
}

vec2 vec_sub(vec2 a, vec2 b)
{
	return (vec2){a.x - b.x, a.y - b.y};
}

vec2 vec_scale(vec2 v, double s)
{
	return (vec2){v.x * s, v.y * s};
}

double vec_magnitude(vec2 v)
{
	return sqrt(v.x*v.x + v.y*v.y);
}
