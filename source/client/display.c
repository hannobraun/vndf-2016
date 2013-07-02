#include <assert.h>
#include <math.h>

#include <GL/glfw.h>

#include "display.h"


const int screenWidth  = 800;
const int screenHeight = 600;


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

void display_render(GLfloat h, GLfloat v, posMap positions)
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
	glRotatef(h, 0.0f, 1.0f, 0.0f);
	glRotatef(v, 1.0f, 0.0f, 0.0f);

	idmap_each(positions, i,
		glPushMatrix();

		glTranslatef(
			idmap_get(positions, i).x,
			idmap_get(positions, i).y,
			0.0f);

		glColor3f(0.0f, 0.0f, 1.0f);
		glBegin(GL_TRIANGLE_STRIP);
			glVertex3f(  0.0f, 20.0f, 0.0f);
			glVertex3f(-20.0f,-10.0f, 0.0f);
			glVertex3f( 20.0f,-10.0f, 0.0f);
		glEnd();

		glPopMatrix();
	)

	glfwSwapBuffers();
}
