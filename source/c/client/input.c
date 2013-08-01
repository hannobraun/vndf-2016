#include "input.h"

#include <GL/glfw.h>


const float cameraSpeed = 1.0f;


void input_apply(camera *cam)
{
	if (glfwGetKey(GLFW_KEY_RIGHT) == GLFW_PRESS)
	{
		cam->h -= cameraSpeed;
	}
	if (glfwGetKey(GLFW_KEY_LEFT) == GLFW_PRESS)
	{
		cam->h += cameraSpeed;
	}
	if (glfwGetKey(GLFW_KEY_UP) == GLFW_PRESS)
	{
		cam->v += cameraSpeed;
	}
	if (glfwGetKey(GLFW_KEY_DOWN) == GLFW_PRESS)
	{
		cam->v -= cameraSpeed;
	}

	if (cam->v >= 90.0f)
	{
		cam->v = 90.0f;
	}
	if (cam->v <= -90.0f)
	{
		cam->v = -90.0f;
	}
}
