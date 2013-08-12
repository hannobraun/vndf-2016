#include "camera.h"
#include "pos.h"


GLFWwindow *display_init(int screenWidth, int screenHeight);
void display_render(
	GLFWwindow *window,
	camera cam,
	posMap positions,
	GLuint textureName);
