#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include <sys/socket.h>

#include <GL/glfw.h>

#include "net.h"


const int screenWidth  = 800;
const int screenHeight = 600;


void receivePosition(int socketFD, float *xPos, float *yPos);

void initRendering(void);
void render(float xPos, float yPos);


int main(int argc, char const *argv[])
{
	if (argc != 2)
	{
		fprintf(stderr, "Usage: %s serverHostname\n", argv[0]);
		exit(1);
	}

	int socketFD = net_connect(argv[1], "34481");
	initRendering();

	while (
		glfwGetWindowParam(GLFW_OPENED) &&
		glfwGetKey(GLFW_KEY_ESC) == GLFW_RELEASE)
	{
		float xPos;
		float yPos;

		receivePosition(socketFD, &xPos, &yPos);

		render(xPos, yPos);
	}

	return 0;
}

void receivePosition(int socketFD, float *xPos, float *yPos)
{
	#define MESSAGE_LENGTH 256
	char message[MESSAGE_LENGTH];
	ssize_t bytesReceived = net_receive(socketFD, message, MESSAGE_LENGTH);
	if (bytesReceived > 0)
	{
		int id;
		int status = sscanf(message + 1,
			"id: %d, pos: (%f, %f)\n",
			&id, xPos, yPos);
		if (status != 3)
		{
			printf(
				"Error reading from socket. Only %d item(s) matched.\n",
				status);
			exit(1);
		}
	}
}

void initRendering()
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

	glfwSetWindowTitle("Von Neumann Defense Force");
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
