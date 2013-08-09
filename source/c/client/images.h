#include <GL/gl.h>


typedef struct {
	unsigned char *data;

	int width;
	int height;
} image;


GLuint images_load(void);
