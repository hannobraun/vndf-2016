typedef struct {
	double x;
	double y;
} vec2;

vec2 vec_add(vec2 a, vec2 b);
vec2 vec_sub(vec2 a, vec2 b);
vec2 vec_scale(vec2 v, double s);
double vec_magnitude(vec2 v);
vec2 vec_normalize(vec2 v);
