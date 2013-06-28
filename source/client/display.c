#include <assert.h>

#include <GL/glfw.h>

#include "display.h"


static const int screenWidth  = 800;
static const int screenHeight = 600;


void display_init()
{
	int status = glfwInit();
	assert(status);

	status = glfwOpenWindow(
		screenWidth, screenHeight,
		8, 8, 8, 8,
		0, 0,
		GLFW_WINDOW);
	assert(status);

	glfwSetWindowTitle("Von Neumann Defense Force");
}
