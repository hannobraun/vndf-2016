#include "vec.h"


#include <math.h>


vec2 vec_normalize(vec2 v)
{
	double m = vec_magnitude(v);
	return vec_scale(v, 1/m);
}
