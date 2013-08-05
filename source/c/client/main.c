#include <assert.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <sys/socket.h>

#include <GL/glfw.h>

#include "camera.h"
#include "display.h"
#include "images.h"
#include "input.h"
#include "net.h"
#include "pos.h"
#include <common/idmap.h>


#define BUFFER_SIZE 256
typedef struct {
	int  socketFD;
	char buffer[BUFFER_SIZE];
	int  bufferPos;
} conn;


void receivePositions(conn *c, posMap positions);


int main(int argc, char const *argv[])
{
	char serverAddress[100];
	if (argc == 2)
	{
		strcpy(serverAddress, argv[1]);
	}
	else if (argc > 2)
	{
		fprintf(
			stderr,
			"Usage: %s serverAddress\n",
			argv[0]);
	}
	else
	{
		FILE *serverFile = fopen("server", "r");
		assert(serverFile != NULL);

		char *ret = fgets(serverAddress, sizeof serverAddress, serverFile);
		assert(ret != NULL);

		if (serverAddress[strlen(serverAddress) - 1] == '\n')
		{
			serverAddress[strlen(serverAddress) - 1] = '\0';
		}
	}

	display_init();
	GLuint textureName = images_load();
	int socketFD = net_connect(serverAddress, "34481");

	conn c;
	c.socketFD  = socketFD;
	c.bufferPos = 0;

	posMap positions;
	idmap_init(positions, 4);

	camera cam = {0.0f, 0.0f};

	while (
		glfwGetWindowParam(GLFW_OPENED) &&
		glfwGetKey(GLFW_KEY_ESC) == GLFW_RELEASE)
	{
		receivePositions(&c, positions);
		input_apply(&cam);
		display_render(cam, positions, textureName);
	}

	return 0;
}

void receivePositions(conn *c, posMap positions)
{
	ssize_t bytesReceived = net_receive(
		c->socketFD,
		c->buffer + c->bufferPos,
		(size_t)(BUFFER_SIZE - c->bufferPos));

	c->bufferPos += bytesReceived;

	while (c->bufferPos > 0 && c->buffer[0] <= c->bufferPos)
	{
		size_t messageSize = (size_t)c->buffer[0];
		assert(messageSize >= 0);

		const int msgTypeLen = 32;
		char msgType[msgTypeLen];
		size_t readLen = 0;
		int status = sscanf(c->buffer + 1, "%s%n", msgType, &readLen);
		assert(status == 1);
		assert(readLen < msgTypeLen);

		if (strcmp(msgType, "UPDATE") == 0)
		{
			size_t id;
			pos position;
			status = sscanf(c->buffer + 1,
				"UPDATE id: %lu, pos: (%f, %f)%n\n",
				&id, &position.x, &position.y, &readLen);
			assert(status == 3);
			assert(readLen == messageSize - 1);

			idmap_put(positions, id, position);
		}
		else if (strcmp(msgType, "REMOVE") == 0)
		{
			size_t id;

			status = sscanf(c->buffer + 1,
				"REMOVE id: %lu%n",
				&id, &readLen);
			assert(status == 1);
			assert(readLen == messageSize - 1);

			idmap_remove(positions, id);
		}
		else
		{
			printf("Unknown message type: %s\n", msgType);
			assert(false);
		}

		memcpy(c->buffer, c->buffer + messageSize, BUFFER_SIZE - messageSize);
		c->bufferPos -= messageSize;
	}
}
