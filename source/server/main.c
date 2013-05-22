#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <sys/epoll.h>
#include <unistd.h>

#include "net.h"


#define MAX_CLIENTS 4


typedef struct {
	int socketFD;
	int id;
	int xPos;
	int yPos;
} client;


void sendPosition(int clientFD, int id, int xPos, int yPos);


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	net net = net_init("34481");

	client clients[MAX_CLIENTS];
	int nextClientId = 0;

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

			if (nextClientId == MAX_CLIENTS)
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

				clients[nextClientId] =
					(client){clientFD, nextClientId, xPos, yPos};
				nextClientId += 1;
			}
		}

		for (int i = 0; i < nextClientId; i += 1)
		{
			clients[i].xPos += 5;
			clients[i].yPos += 0;
		}

		for (int i = 0; i < nextClientId; i += 1)
		{
			for (int j = 0; j < nextClientId; j += 1)
			{
				sendPosition(
					clients[i].socketFD,
					clients[j].id,
					clients[j].xPos,
					clients[j].yPos);
			}

		}
	}
}

void sendPosition(int clientFD, int id, int xPos, int yPos)
{
	char message[256];
	int status = snprintf(
		message + 1, sizeof message - 1,
		"UPDATE id: %d, pos: (%d, %d)",
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

	size_t messageLength = strlen(message + 1) + 1;
	if (messageLength <= CHAR_MAX)
	{
		message[0] = (char)messageLength;
	}
	else
	{
		printf(
			"Message size cannot be encoded. Message: \"%s\", size: %lu\n",
			message + 1, messageLength);
		exit(1);
	}

	net_send(clientFD, message, strlen(message));
}
