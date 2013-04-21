#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <netdb.h>

#include <GL/glfw.h>


const int screenWidth  = 800;
const int screenHeight = 600;


void read_initial_position(float *xPos, float *yPos);

void init_rendering(void);
void render(float xPos, float yPos);


int main(int argc, char const *argv[])
{
	float xPos;
	float yPos;

	read_initial_position(&xPos, &yPos);

	init_rendering();

	while (
		glfwGetWindowParam(GLFW_OPENED) &&
		glfwGetKey(GLFW_KEY_ESC) == GLFW_RELEASE)
	{
		xPos += 0.5f;
		render(xPos, yPos);
	}

	return 0;
}

void read_initial_position(float *xPos, float *yPos)
{
	int status;

	struct addrinfo hints;
	memset(&hints, 0, sizeof hints);
	hints.ai_family   = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;

	struct addrinfo *servinfo;

	status = getaddrinfo("localhost", "34481", &hints, &servinfo);
	if (status != 0)
	{
		printf("Error getting address info: %s\n", strerror(errno));
		exit(1);
	}

	int socket_fd = socket(
		servinfo->ai_family,
		servinfo->ai_socktype,
		servinfo->ai_protocol);
	if (socket_fd == -1)
	{
		printf("Error creating socket: %s\n", strerror(errno));
		exit(1);
	}

	status = connect(socket_fd, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		printf("Error connecting to server: %s\n", strerror(errno));
		exit(1);
	}

	char message[256];
	ssize_t bytes_received = recv(socket_fd, message, sizeof(message), 0);
	if (bytes_received < 0)
	{
		printf("Error receiving message: %s\n", strerror(errno));
		exit(1);
	}
	if (bytes_received == 0)
	{
		printf("Connection closed while receiving.\n");
		exit(1);
	}

	status = sscanf(message, "%f %f\n", xPos, yPos);
	if (status != 2)
	{
		printf("Error reading from socket. Only %d item(s) matched.\n", status);
		exit(1);
	}

	freeaddrinfo(servinfo);
}

void init_rendering()
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
