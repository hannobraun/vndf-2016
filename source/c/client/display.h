#include "camera.h"
#include "pos.h"


GLFWwindow *display_init(void);
void display_render(
	GLFWwindow *window,
	camera cam,
	posMap positions,
	GLuint textureName);
