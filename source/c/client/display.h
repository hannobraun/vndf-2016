#include "camera.h"
#include "pos.h"
#include "textures.h"

#include <GLFW/glfw3.h>


GLFWwindow *display_init(int screenWidth, int screenHeight);
void display_render(
	GLFWwindow *window,
	camera     cam,
	posMap     positions,
	Texture    texture);
