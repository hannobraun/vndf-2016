#include <stdio.h>
#include <stdlib.h>

#include <GL/glfw.h>


const int screenWidth  = 800;
const int screenHeight = 600;


void read_initial_position(float *xPos, float *yPos);

void init_rendering(void);
void render(float xPos, float yPos);


int main(int argc, char const *argv[])
{
	float xPos;
	float yPos;

	read_initial_position(&xPos, &yPos);

	init_rendering();

	while (
		glfwGetWindowParam(GLFW_OPENED) &&
		glfwGetKey(GLFW_KEY_ESC) == GLFW_RELEASE)
	{
		xPos += 0.5f;
		render(xPos, yPos);
	}

	return 0;
}

void read_initial_position(float *xPos, float *yPos)
{
	*xPos = -300.0f;
	*yPos = 0.0f;
}

void init_rendering()
{
	if (!glfwInit())
	{
		printf("Error initializing GLFW.\n");
		exit(1);
	}

	if (
		!glfwOpenWindow(
			screenWidth, screenHeight,
			8, 8, 8, 8,
			0, 0,
			GLFW_WINDOW))
	{
		printf("Error opening GLFW window.\n");
		exit(1);
	}
}

void render(float xPos, float yPos)
{
	glClear(GL_COLOR_BUFFER_BIT);
	glLoadIdentity();
	glOrtho(
		-screenWidth/2, screenWidth/2,
		-screenHeight/2, screenHeight/2,
		-1, 1);

	glTranslatef(xPos, yPos, 0.0f);

	glColor3f(0.0f, 0.0f, 1.0f);
	glBegin(GL_TRIANGLE_STRIP);
		glVertex3f(  0.0f, 20.0f, 0.0f);
		glVertex3f(-20.0f,-10.0f, 0.0f);
		glVertex3f( 20.0f,-10.0f, 0.0f);
	glEnd();

	glfwSwapBuffers();
}
