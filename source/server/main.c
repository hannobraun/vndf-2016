#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <sys/epoll.h>
#include <unistd.h>

#include "net.h"


void sendPosition(int clientFD, int xPos, int yPos);


#define MAX_CLIENTS 1024


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	int serverFD = net_initSocket();
	int pollerFD = net_initPoller();

	net_registerAccept(pollerFD, serverFD);

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
			int clientFD = net_acceptClient(serverFD);

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
