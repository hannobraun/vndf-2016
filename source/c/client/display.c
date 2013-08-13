#include "display.h"

#include <assert.h>
#include <math.h>


GLFWwindow *createWindow(int width, int height);


GLFWwindow *display_init(int screenWidth, int screenHeight)
{
	GLFWwindow * window = createWindow(screenWidth, screenHeight);
	glEnable(GL_TEXTURE_2D);

	glEnable(GL_BLEND);
	glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

	glLoadIdentity();

	// I'm not a 100% sure what this does, but it has to do with using textures
	// that are not power of two. Before I added this call, glTexture2D wouldn't
	// work correctly on an 11x11 texture, causing memory access errors and not
	// displaying it correctly.
	glPixelStorei(GL_UNPACK_ALIGNMENT, 1);

	double pi = atan(1) * 4;
	GLfloat zNear = 0.1f;
	GLfloat fovAngleY = 45.0f;
	GLfloat halfHeight = (float)tan( fovAngleY / 360.0f * pi ) * zNear;
	GLfloat halfWidth = halfHeight * screenWidth / screenHeight;
	glFrustum(
		-halfWidth, halfWidth,
		-halfHeight, halfHeight,
		zNear, 1000.0f);

	return window;
}

GLFWwindow *createWindow(int width, int height)
{
	int status = glfwInit();
	assert(status);

	GLFWwindow *window = glfwCreateWindow(
		width, height,
		"Von Neumann Defense Force",
		NULL, NULL);
	assert(window);

	glfwMakeContextCurrent(window);

	return window;
}

void display_render(
	GLFWwindow *window,
	camera     cam,
	posMap     positions,
	Texture    texture)
{
	glClear(GL_COLOR_BUFFER_BIT);

	glPushMatrix();

	glTranslatef(0.0f, 0.0f, -500.0f);
	glRotatef(cam.v, 1.0f, 0.0f, 0.0f);
	glRotatef(cam.h, 0.0f, 1.0f, 0.0f);

	glBindTexture(
		GL_TEXTURE_2D,
		texture.name);

	glColor4f(1.0f, 1.0f, 1.0f, 1.0f);

	idmap_each(positions, i,
		glPushMatrix();

		glTranslatef(
			idmap_get(positions, i).x - texture.width/2,
			idmap_get(positions, i).y - texture.height/2,
			0.0f);

		glBegin(GL_TRIANGLE_STRIP);
			glTexCoord2f(1.0f, 0.0f);
			glVertex3f(texture.width, texture.height, 0.0f);

			glTexCoord2f(1.0f, 1.0f);
			glVertex3f(texture.width, 0.0f, 0.0f);

			glTexCoord2f(0.0f, 0.0f);
			glVertex3f(0.0f, texture.height, 0.0f);

			glTexCoord2f(0.0f, 1.0f);
			glVertex3f(0.0f, 0.0f, 0.0f);
		glEnd();

		glPopMatrix();
	)

	glPopMatrix();

	glfwSwapBuffers(window);
	glfwPollEvents();
}
