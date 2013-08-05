#include <GL/gl.h>


typedef struct {
	unsigned char *data;

	int xSize;
	int ySize;
} image;


GLuint images_load(void);
