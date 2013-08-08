#include <assert.h>
#include <math.h>

#include <GLFW/glfw3.h>

#include "display.h"


const static int screenWidth  = 800;
const static int screenHeight = 600;


GLFWwindow *createWindow(void);


GLFWwindow *display_init()
{
	GLFWwindow * window = createWindow();
	glEnable(GL_TEXTURE_2D);

	return window;
}

GLFWwindow *createWindow()
{
	int status = glfwInit();
	assert(status);

	GLFWwindow *window = glfwCreateWindow(
		screenWidth, screenHeight,
		"Von Neumann Defense Force",
		NULL, NULL);
	assert(window);

	glfwMakeContextCurrent(window);

	return window;
}

void display_render(
	GLFWwindow *window,
	camera cam,
	posMap positions,
	GLuint textureName)
{
	glClear(GL_COLOR_BUFFER_BIT);
	glLoadIdentity();

	double pi = atan(1) * 4;
	GLfloat zNear = 0.1f;
	GLfloat fovAngleY = 45.0f;
	GLfloat halfHeight = (float)tan( fovAngleY / 360.0f * pi ) * zNear;
	GLfloat halfWidth = halfHeight * screenWidth / screenHeight;
	glFrustum(
		-halfWidth, halfWidth,
		-halfHeight, halfHeight,
		zNear, 1000.0f);

	glTranslatef(0.0f, 0.0f, -500.0f);
	glRotatef(cam.v, 1.0f, 0.0f, 0.0f);
	glRotatef(cam.h, 0.0f, 1.0f, 0.0f);

	glBindTexture(
		GL_TEXTURE_2D,
		textureName);

	glColor4f(1.0f, 1.0f, 1.0f, 1.0f);

	idmap_each(positions, i,
		glPushMatrix();

		glTranslatef(
			idmap_get(positions, i).x,
			idmap_get(positions, i).y,
			0.0f);

		glBegin(GL_TRIANGLE_STRIP);
			glTexCoord2f(1.0f, 0.0f);
			glVertex3f(20.0f, 20.0f, 0.0f);

			glTexCoord2f(1.0f, 1.0f);
			glVertex3f(20.0f, -20.0f, 0.0f);

			glTexCoord2f(0.0f, 0.0f);
			glVertex3f(-20.0f, 20.0f, 0.0f);

			glTexCoord2f(0.0f, 1.0f);
			glVertex3f(-20.0f, -20.0f, 0.0f);
		glEnd();

		glPopMatrix();
	)

	glfwSwapBuffers(window);
	glfwPollEvents();
}
