#include <assert.h>
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <sys/epoll.h>
#include <unistd.h>

#include <common/idmap.h>
#include <common/stack.h>
#include "clients.h"
#include "net.h"


int sendPosition(int clientFD, size_t id, int xPos, int yPos);


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	net net = net_init("34481");

	clientMap clientMap;
	clients_initClientMap(&clientMap, 4);

	while (true)
	{
		#define MAX_EVENTS 1024
		struct epoll_event events[MAX_EVENTS];
		int numberOfEvents = epoll_wait(net.pollerFD, events, MAX_EVENTS, 500);
		assert(numberOfEvents != -1);

		for (int i = 0; i < numberOfEvents; i += 1)
		{
			int clientFD = net_acceptClient(net.serverFD);

			if (clients_canAdd(&clientMap))
			{
				int status = close(clientFD);
				assert(status == 0);
			}
			else
			{
				int xPos = rand() % 600 - 300;
				int yPos = rand() % 400 - 200;

				size_t clientId;
				stack_pop(clientMap.idPool, &clientId);

				client client = {clientFD, clientId, xPos, yPos};
				idmap_put(clientMap.clients, clientId, client);
			}
		}

		idmap_each(clientMap.clients, i,
			idmap_get(clientMap.clients, i).xPos += 5;
			idmap_get(clientMap.clients, i).yPos += 0;
		)

		idmap_each(clientMap.clients, i,
			idmap_each(clientMap.clients, j,
				int status = sendPosition(
					idmap_get(clientMap.clients, i).socketFD,
					idmap_get(clientMap.clients, j).id,
					idmap_get(clientMap.clients, j).xPos,
					idmap_get(clientMap.clients, j).yPos);

				if (status < 0)
				{
					idmap_remove(clientMap.clients, i);
					stack_push(clientMap.idPool, i);
				}
			)
		)
	}
}

int sendPosition(int clientFD, size_t id, int xPos, int yPos)
{
	char message[256];
	int status = snprintf(
		message + 1, sizeof message - 1,
		"UPDATE id: %lu, pos: (%d, %d)",
		id, xPos, yPos);
	assert(status >= 0);
	assert((size_t)status <= sizeof message);

	size_t messageLength = strlen(message + 1) + 1;
	assert(messageLength <= CHAR_MAX);
	message[0] = (char)messageLength;

	return net_send(clientFD, message, strlen(message));
}
