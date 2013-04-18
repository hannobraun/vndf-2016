#include <stdio.h>

#include "GL/glfw.h"

const int screenWidth  = 800;
const int screenHeight = 600;

int main(int argc, char const *argv[])
{
	if (!glfwInit())
	{
		printf("Error initializing GLFW.\n");
		return 1;
	}

	if (
		!glfwOpenWindow(
			screenWidth, screenHeight,
			8, 8, 8, 8,
			0, 0,
			GLFW_WINDOW))
	{
		printf("Error opening GLFW window.\n");
		return 1;
	}

	while (glfwGetWindowParam(GLFW_OPENED))
    {
		glfwSwapBuffers();
    }

	return 0;
}
