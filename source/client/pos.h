#ifndef POS_H
#define POS_H


#include <common/idmap.h>

typedef struct {
	float x;
	float y;
} pos;

typedef idmap(pos) posMap;


#endif
