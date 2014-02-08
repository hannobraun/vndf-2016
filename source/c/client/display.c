#include "display.h"

#include <assert.h>
#include <math.h>


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
