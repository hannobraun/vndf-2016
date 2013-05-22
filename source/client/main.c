#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <sys/socket.h>

#include <GL/glfw.h>

#include "net.h"
#include <common/idmap.h>


const int screenWidth  = 800;
const int screenHeight = 600;


#define BUFFER_SIZE 256
typedef struct {
	int  socketFD;
	char buffer[BUFFER_SIZE];
	int  bufferPos;
} conn;

typedef struct {
	float x;
	float y;
} pos;

typedef idmap_entry(pos) posEntry;

typedef struct {
	size_t cap;
	posEntry *elems;
} posMap;


void receivePosition(conn *c, posMap positions);

void initRendering(void);
void render(posMap positions);


int main(int argc, char const *argv[])
{
	if (argc != 2)
	{
		fprintf(stderr, "Usage: %s serverHostname\n", argv[0]);
		exit(1);
	}

	int socketFD = net_connect(argv[1], "34481");
	initRendering();

	conn c;
	c.socketFD  = socketFD;
	c.bufferPos = 0;

	const size_t maxPositions = 2;
	const size_t positionsMem = maxPositions * sizeof(posEntry);

	posMap positions;
	positions.cap = maxPositions;
	positions.elems = malloc(positionsMem);
	memset(positions.elems, 0, positionsMem);

	while (
		glfwGetWindowParam(GLFW_OPENED) &&
		glfwGetKey(GLFW_KEY_ESC) == GLFW_RELEASE)
	{
		receivePosition(&c, positions);

		render(positions);
	}

	return 0;
}

void receivePosition(conn *c, posMap positions)
{
	ssize_t bytesReceived = net_receive(
		c->socketFD,
		c->buffer + c->bufferPos,
		(size_t)(BUFFER_SIZE - c->bufferPos));

	c->bufferPos += bytesReceived;

	while (c->bufferPos > 0 && c->buffer[0] <= c->bufferPos)
	{
		if (c->buffer[0] < 0)
		{
			printf("Invalid message length: %d", c->buffer[0]);
			exit(1);
		}

		size_t id;
		pos position;
		int status = sscanf(c->buffer + 1,
			"UPDATE id: %lu, pos: (%f, %f)\n",
			&id, &position.x, &position.y);
		if (status != 3)
		{
			printf(
				"Error reading from socket. Only %d item(s) matched.\n",
				status);
			exit(1);
		}

		if (id >= positions.cap)
		{
			printf("Received id (%lu) too high. Limit: %lu\n",
				id, positions.cap);
			exit(1);
		}

		idmap_put(positions.elems, id, position);

		size_t messageSize = (size_t)c->buffer[0];

		memcpy(c->buffer, c->buffer + messageSize, messageSize);
		c->bufferPos -= messageSize;
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

void render(posMap positions)
{
	glClear(GL_COLOR_BUFFER_BIT);
	glLoadIdentity();
	glOrtho(
		-screenWidth/2, screenWidth/2,
		-screenHeight/2, screenHeight/2,
		-1, 1);

	for (size_t i = 0; i < positions.cap; i+= 1)
	{
		if (positions.elems[i].isOccupied)
		{
			glTranslatef(
				positions.elems[i].value.x,
				positions.elems[i].value.y, 0.0f);

			glColor3f(0.0f, 0.0f, 1.0f);
			glBegin(GL_TRIANGLE_STRIP);
				glVertex3f(  0.0f, 20.0f, 0.0f);
				glVertex3f(-20.0f,-10.0f, 0.0f);
				glVertex3f( 20.0f,-10.0f, 0.0f);
			glEnd();
		}
	}

	glfwSwapBuffers();
}
