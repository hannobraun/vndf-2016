#include "input.h"

#include <GL/glfw.h>


const float cameraSpeed = 1.0f;


void input_apply(float *h, float *v)
{
	if (glfwGetKey(GLFW_KEY_RIGHT) == GLFW_PRESS)
	{
		*h -= cameraSpeed;
	}
	if (glfwGetKey(GLFW_KEY_LEFT) == GLFW_PRESS)
	{
		*h += cameraSpeed;
	}
	if (glfwGetKey(GLFW_KEY_UP) == GLFW_PRESS)
	{
		*v += cameraSpeed;
	}
	if (glfwGetKey(GLFW_KEY_DOWN) == GLFW_PRESS)
	{
		*v -= cameraSpeed;
	}

	if (*v >= 90.0f)
	{
		*v = 90.0f;
	}
	if (*v <= -90.0f)
	{
		*v = -90.0f;
	}
}
