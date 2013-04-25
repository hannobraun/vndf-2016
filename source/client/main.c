#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <netdb.h>

#include <GL/glfw.h>


const int screenWidth  = 800;
const int screenHeight = 600;


int connectToServer(const char *hostname);
bool receivePosition(int socket_fd, float *xPos, float *yPos);

void initRendering(void);
void render(float xPos, float yPos);


int main(int argc, char const *argv[])
{
	if (argc != 2)
	{
		fprintf(stderr, "Usage: %s serverHostname\n", argv[0]);
		exit(1);
	}

	int socket_fd = connectToServer(argv[1]);
	initRendering();

	while (
		glfwGetWindowParam(GLFW_OPENED) &&
		glfwGetKey(GLFW_KEY_ESC) == GLFW_RELEASE)
	{
		float xPos;
		float yPos;

		receivePosition(socket_fd, &xPos, &yPos);

		render(xPos, yPos);
	}

	return 0;
}

int connectToServer(const char *hostname)
{
	int status;

	struct addrinfo hints;
	memset(&hints, 0, sizeof hints);
	hints.ai_family   = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;

	struct addrinfo *servinfo;

	status = getaddrinfo(hostname, "34481", &hints, &servinfo);
	if (status != 0)
	{
		perror("Error getting address info");
		exit(1);
	}

	int socket_fd = socket(
		servinfo->ai_family,
		servinfo->ai_socktype,
		servinfo->ai_protocol);
	if (socket_fd == -1)
	{
		perror("Error creating socket");
		exit(1);
	}

	status = connect(socket_fd, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		perror("Error connecting to server");
		exit(1);
	}

	freeaddrinfo(servinfo);

	return socket_fd;
}

bool receivePosition(int socket_fd, float *xPos, float *yPos)
{
	char message[256];
	ssize_t bytes_received = recv(
		socket_fd,
		message,
		sizeof(message),
		MSG_DONTWAIT);
	if (bytes_received == -1 && (errno == EAGAIN || errno == EWOULDBLOCK))
	{
		return false;
	}
	if (bytes_received < 0)
	{
		perror("Error receiving message");
		exit(1);
	}
	if (bytes_received == 0)
	{
		printf("Connection closed while receiving.\n");
		exit(1);
	}

	int status = sscanf(message, "%f %f\n", xPos, yPos);
	if (status != 2)
	{
		printf("Error reading from socket. Only %d item(s) matched.\n", status);
		exit(1);
	}

	return true;
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
