#include <assert.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <sys/socket.h>

#include <GLFW/glfw3.h>

#include "textures.h"
#include <common/idmap.h>


typedef struct {
	float x;
	float y;
} pos;

typedef idmap(pos) posMap;


typedef struct {
	float h;
	float v;
} camera;


GLFWwindow *display_init(int screenWidth, int screenHeight);
void display_render(
	GLFWwindow *window,
	camera     cam,
	posMap     positions,
	Texture    texture);



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
