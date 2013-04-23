#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <sys/epoll.h>
#include <unistd.h>


int initSocket(void);
int initPoller(void);
void registerAccept(int pollerFD, int serverFD);
int acceptClient(int serverFD);
void sendPosition(int clientFD, int xPos, int yPos);


#define CLIENT_ACCEPT_BACKLOG 1024
#define MAX_CLIENTS 1024


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	int serverFD = initSocket();
	int pollerFD = initPoller();

	registerAccept(pollerFD, serverFD);

	int clients[MAX_CLIENTS];
	int nextClientIndex = 0;

	while (true)
	{
		#define MAX_EVENTS 1024
		struct epoll_event events[MAX_EVENTS];
		int numberOfEvents = epoll_wait(pollerFD, events, MAX_EVENTS, 500);
		if (numberOfEvents == -1)
		{
			perror("Error waiting for socket events");
			exit(1);
		}

		for (int i = 0; i < numberOfEvents; i += 1)
		{
			int clientFD = acceptClient(serverFD);

			if (nextClientIndex == MAX_CLIENTS)
			{
				int status = close(clientFD);
				if (status != 0)
				{
					perror("Error rejecting client connection.");
					exit(1);
				}
			}
			else
			{
				clients[nextClientIndex] = clientFD;
				nextClientIndex += 1;
			}
		}

		int xPos = rand() % 600 - 300;
		int yPos = rand() % 400 - 200;

		for (int i = 0; i < nextClientIndex; i += 1)
		{
			int clientFD = clients[i];
			sendPosition(clientFD, xPos, yPos);
		}
	}
}

int initSocket()
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

	int socketFD = socket(
		servinfo->ai_family,
		servinfo->ai_socktype,
		servinfo->ai_protocol);

	if (socketFD == -1)
	{
		perror("Error creating socket");
		exit(1);
	}

	int yes=1;
	if (setsockopt(socketFD,SOL_SOCKET,SO_REUSEADDR,&yes,sizeof(int)) == -1)
	{
		perror("Error setting socket option");
		exit(1);
	}

	status = bind(socketFD, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		perror("Error binding socket");
		exit(1);
	}

	status = listen(socketFD, CLIENT_ACCEPT_BACKLOG);
	if (status != 0)
	{
		perror("Error listening on socket");
		exit(1);
	}

	freeaddrinfo(servinfo);

	return socketFD;
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

void registerAccept(int pollerFD, int serverFD)
{
	struct epoll_event event;
	event.events = EPOLLIN;
	int status = epoll_ctl(pollerFD, EPOLL_CTL_ADD, serverFD, &event);
	if (status != 0)
	{
		perror("Error registering server socket with epoll");
		exit(1);
	}
}

int acceptClient(int serverFD)
{
	struct sockaddr_storage remote_address;
	socklen_t address_size = sizeof remote_address;

	int clientFD = accept(
		serverFD,
		(struct sockaddr *)&remote_address,
		&address_size);

	return clientFD;
}

void sendPosition(int clientFD, int xPos, int yPos)
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
	ssize_t bytes_sent = send(clientFD, message, message_length, MSG_NOSIGNAL);
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
