#include "pos.h"


typedef struct {
	float h;
	float v;
} camera;


GLuint display_init(void);
void display_render(GLfloat h, GLfloat v, posMap positions, GLuint textureName);
