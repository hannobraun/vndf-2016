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

	glOrtho(
		-screenWidth/2, screenWidth/2,
		-screenHeight/2, screenHeight/2,
		-1, 1);

	while (glfwGetWindowParam(GLFW_OPENED))
    {
		glClear(GL_COLOR_BUFFER_BIT);

		glColor3f(0.0f, 0.0f, 1.0f);
		glBegin(GL_TRIANGLE_STRIP);
			glVertex3f(  0.0f, 20.0f, 0.0f);
			glVertex3f(-20.0f,-10.0f, 0.0f);
			glVertex3f( 20.0f,-10.0f, 0.0f);
		glEnd();

		glfwSwapBuffers();
    }

	return 0;
}
