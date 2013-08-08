#include "input.h"


const static float cameraSpeed = 1.0f;


void input_apply(GLFWwindow *window, camera *cam)
{
	if (glfwGetKey(window, GLFW_KEY_RIGHT) == GLFW_PRESS)
	{
		cam->h -= cameraSpeed;
	}
	if (glfwGetKey(window, GLFW_KEY_LEFT) == GLFW_PRESS)
	{
		cam->h += cameraSpeed;
	}
	if (glfwGetKey(window, GLFW_KEY_UP) == GLFW_PRESS)
	{
		cam->v += cameraSpeed;
	}
	if (glfwGetKey(window, GLFW_KEY_DOWN) == GLFW_PRESS)
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
