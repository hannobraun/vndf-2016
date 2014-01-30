#include <assert.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <sys/socket.h>

#include <GLFW/glfw3.h>

#include "camera.h"
#include "display.h"
#include "pos.h"
#include <common/idmap.h>


#define BUFFER_SIZE 256
typedef struct {
	int  socketFD;
	char buffer[BUFFER_SIZE];
	int  bufferPos;
} conn;

typedef struct {
	unsigned char *data;

	int width;
	int height;
} image;


Texture images_load(void);

void input_apply(GLFWwindow *window, camera *cam);

int net_connect(const char *hostname, char *port);
ssize_t net_receive(int socketFD, char *buffer, size_t bufferSize);
void receivePositions(conn *c, posMap positions);


const static int screenWidth  = 800;
const static int screenHeight = 600;


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

	GLFWwindow *window = display_init(screenWidth, screenHeight);
	Texture texture = images_load();
	int socketFD = net_connect(serverAddress, "34481");

	conn c;
	c.socketFD  = socketFD;
	c.bufferPos = 0;

	posMap positions;
	idmap_init(positions, 4);

	camera cam = {0.0f, 0.0f};

	while (
		!glfwWindowShouldClose(window) &&
		glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_RELEASE)
	{
		receivePositions(&c, positions);
		input_apply(window, &cam);
		display_render(window, cam, positions, texture);
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
		int messageSize = c->buffer[0];
		assert(messageSize >= 0);

		#define MSG_TYPE_LEN 32
		char msgType[MSG_TYPE_LEN];
		int readLen = 0;
		int status = sscanf(c->buffer + 1, "%s%n", msgType, &readLen);
		assert(status == 1);
		assert(readLen < MSG_TYPE_LEN);

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

		memmove(
			c->buffer,
			c->buffer + messageSize,
			(size_t)(BUFFER_SIZE - messageSize));
		c->bufferPos -= messageSize;
	}
}
