#include <stdio.h>

#include <GL/glfw.h>


const int screenWidth  = 800;
const int screenHeight = 600;


void render(float xPos, float yPos);


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

	float xPos = -300.0f;

	while (
		glfwGetWindowParam(GLFW_OPENED) &&
		glfwGetKey(GLFW_KEY_ESC) == GLFW_RELEASE)
    {
    	xPos += 0.5f;
		render(xPos, 0.0f);
    }

	return 0;
}

void render(float xPos, float yPos) {
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
