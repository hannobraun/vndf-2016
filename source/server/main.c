#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <sys/epoll.h>
#include <unistd.h>


int initServer(void);
int initPoller(void);
int acceptClient(int server_fd);
void sendPosition(int client_fd, int xPos, int yPos);


#define MAX_CLIENTS 1


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	int server_fd = initServer();
	int pollerFD  = initPoller();

	struct epoll_event event;
	event.events = EPOLLIN;
	int status = epoll_ctl(pollerFD, EPOLL_CTL_ADD, server_fd, &event);
	if (status != 0)
	{
		perror("Error registering server socket with epoll");
		exit(1);
	}

	#define MAX_EVENTS 1024
	struct epoll_event events[MAX_EVENTS];
	int numberOfEvents = epoll_wait(pollerFD, events, MAX_EVENTS, -1);
	if (numberOfEvents == -1)
	{
		perror("Error waiting for socket events");
		exit(1);
	}

	int client_fd = 0;
	for (int i = 0; i < numberOfEvents; i += 1)
	{
		client_fd = acceptClient(server_fd);
	}

	while (true)
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

	int yes=1;
	if (setsockopt(socket_fd,SOL_SOCKET,SO_REUSEADDR,&yes,sizeof(int)) == -1)
	{
		perror("Error setting socket option");
		exit(1);
	}

	status = bind(socket_fd, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		perror("Error binding socket");
		exit(1);
	}

	status = listen(socket_fd, 20);
	if (status != 0)
	{
		perror("Error listening on socket");
		exit(1);
	}

	freeaddrinfo(servinfo);

	return socket_fd;
}

int initPoller()
{
	int pollerFD = epoll_create(MAX_CLIENTS);
	if (pollerFD < 0)
	{
		perror("Error initiating epoll");
		exit(1);
	}

	return pollerFD;
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
		perror("Error sending message");
		exit(1);
	}
	if ((size_t)bytes_sent != message_length)
	{
		printf("Only sent %ld of %lu bytes.\n", bytes_sent, message_length);
		exit(1);
	}
}
