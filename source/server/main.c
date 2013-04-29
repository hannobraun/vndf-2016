#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <sys/epoll.h>
#include <unistd.h>

#include "net.h"


#define MAX_CLIENTS 1024


typedef struct {
	int socketFD;
	int xPos;
	int yPos;
} client;


void sendPosition(int clientFD, int xPos, int yPos);


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	net net = net_init("34481");

	client clients[MAX_CLIENTS];
	int nextClientIndex = 0;

	while (true)
	{
		#define MAX_EVENTS 1024
		struct epoll_event events[MAX_EVENTS];
		int numberOfEvents = epoll_wait(net.pollerFD, events, MAX_EVENTS, 500);
		if (numberOfEvents == -1)
		{
			perror("Error waiting for socket events");
			exit(1);
		}

		for (int i = 0; i < numberOfEvents; i += 1)
		{
			int clientFD = net_acceptClient(net.serverFD);

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
				int xPos = rand() % 600 - 300;
				int yPos = rand() % 400 - 200;

				clients[nextClientIndex] = (client){clientFD, xPos, yPos};
				nextClientIndex += 1;
			}
		}

		for (int i = 0; i < nextClientIndex; i += 1)
		{
			clients[i].xPos += 5;
			clients[i].yPos += 0;

			sendPosition(
				clients[i].socketFD,
				clients[i].xPos,
				clients[i].yPos);
		}
	}
}

void sendPosition(int clientFD, int xPos, int yPos)
{
	int id = 0;

	char message[256];
	int status = snprintf(
		message, sizeof message,
		"id: %d, pos: (%d, %d)\n",
		id, xPos, yPos);
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
