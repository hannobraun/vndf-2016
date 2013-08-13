#include "textures.h"


typedef struct {
	unsigned char *data;

	int width;
	int height;
} image;


Texture images_load(void);
