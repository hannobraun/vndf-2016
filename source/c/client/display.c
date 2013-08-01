#include <assert.h>
#include <math.h>

#include <GL/glfw.h>
#include <stb/image.h>

#include "display.h"


const int screenWidth  = 800;
const int screenHeight = 600;


GLuint display_init()
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

	int xSize, ySize, numberOfComponents;

	unsigned char *imageData = stbi_load(
		"images/spaceship.png",
		&xSize, &ySize,
		&numberOfComponents,
		0);
	assert(imageData != NULL);
	assert(numberOfComponents == 4);

	glEnable(GL_TEXTURE_2D);

	// Generate texture names.
	GLuint textureName;
	glGenTextures(1, &textureName);

	glBindTexture(
		GL_TEXTURE_2D,
		textureName);

	// Configure texture.
	glTexParameteri(
		GL_TEXTURE_2D,
		GL_TEXTURE_MIN_FILTER,
		GL_NEAREST);

	// Bind image data to texture name.
	glTexImage2D(
		GL_TEXTURE_2D,
		0,
		GL_RGBA8,
		xSize,
		ySize,
		0,
		GL_RGBA,
		GL_UNSIGNED_BYTE,
		imageData);

	return textureName;
}

void display_render(camera cam, posMap positions, GLuint textureName)
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

	glfwSwapBuffers();
}
