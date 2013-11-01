#include "vec.h"


#include <math.h>


double vec_magnitude(vec2 v)
{
	return sqrt(v.x*v.x + v.y*v.y);
}

vec2 vec_normalize(vec2 v)
{
	double m = vec_magnitude(v);
	return vec_scale(v, 1/m);
}
