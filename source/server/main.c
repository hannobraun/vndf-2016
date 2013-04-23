#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <unistd.h>


int initServer(void);
int acceptClient(int server_fd);
void sendPosition(int client_fd, int xPos, int yPos);


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	int server_fd = initServer();
	int client_fd = acceptClient(server_fd);

	for (;;)
	{
		int xPos = rand() % 600 - 300;
		int yPos = rand() % 400 - 200;

		sendPosition(client_fd, xPos, yPos);
		usleep(500000);
	}
}

int initServer()
{
	int status;

	struct addrinfo hints;
	memset(&hints, 0, sizeof hints);
	hints.ai_family   = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;
	hints.ai_flags    = AI_PASSIVE;

	struct addrinfo *servinfo;

	status = getaddrinfo(NULL, "34481", &hints, &servinfo);

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

	int yes=1;
	if (setsockopt(socket_fd,SOL_SOCKET,SO_REUSEADDR,&yes,sizeof(int)) == -1)
	{
		printf("Error setting socket option: %s", strerror(errno));
		exit(1);
	}

	status = bind(socket_fd, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		printf("Error binding socket: %s\n", strerror(errno));
		exit(1);
	}

	status = listen(socket_fd, 20);
	if (status != 0)
	{
		printf("Error listening on socket: %s\n", strerror(errno));
		exit(1);
	}

	freeaddrinfo(servinfo);

	return socket_fd;
}

int acceptClient(int server_fd)
{
	struct sockaddr_storage remote_address;
	socklen_t address_size = sizeof remote_address;

	int client_fd = accept(
		server_fd,
		(struct sockaddr *)&remote_address,
		&address_size);

	return client_fd;
}

void sendPosition(int client_fd, int xPos, int yPos)
{
	char message[256];
	int status = snprintf(message, sizeof message, "%d %d\n", xPos, yPos);
	if (status < 0)
	{
		printf("Error encoding message.\n");
		exit(1);
	}
	if ((size_t)status > sizeof message)
	{
		printf("Message did not fit into buffer.\n");
		exit(1);
	}

	size_t message_length = strlen(message);
	ssize_t bytes_sent = send(client_fd, message, message_length, 0);
	if (bytes_sent < 0)
	{
		printf("Error sending message: %s\n", strerror(errno));
		exit(1);
	}
	if ((size_t)bytes_sent != message_length)
	{
		printf("Only sent %ld of %lu bytes.\n", bytes_sent, message_length);
		exit(1);
	}
}
