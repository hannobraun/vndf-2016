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
#include <common/rbuf.h>
#include <common/stack.h>
#include "clients.h"
#include "events.h"
#include "net.h"


void onConnect(int clientFD, clientMap *clientMap);
void onDisconnect(size_t clientId, clientMap *clientMap);
void onUpdate(clientMap *clientMap, events *events);
int sendUpdate(int clientFD, size_t id, int xPos, int yPos);


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	net net = net_init("34481");

	events events;
	rbuf_init(events, 16);

	clientMap clientMap;
	clients_initClientMap(&clientMap, 4);

	while (true)
	{
		const int maxEvents = 1024;
		struct epoll_event pollEvents[maxEvents];

		int numberOfEvents = epoll_wait(
			net.pollerFD,
			pollEvents,
			maxEvents,
			500);
		assert(numberOfEvents != -1);

		for (int i = 0; i < numberOfEvents; i += 1)
		{
			int clientFD = net_acceptClient(net.serverFD);

			event connectEvent;
			connectEvent.type = ON_CONNECT;
			connectEvent.ev.onConnect.clientFD = clientFD;

			rbuf_put(events, connectEvent);
		}

		event updateEvent;
		updateEvent.type = ON_UPDATE;

		rbuf_put(events, updateEvent);

		while (rbuf_size(events) > 0)
		{
			event event;
			rbuf_get(events, &event);

			switch (event.type)
			{
				case ON_CONNECT:
					onConnect(event.ev.onConnect.clientFD, &clientMap);
					break;

				case ON_DISCONNECT:
					onDisconnect(event.ev.onDisconnect.clientId, &clientMap);
					break;

				case ON_UPDATE:
					onUpdate(&clientMap, &events);
					break;

				default:
					assert(false);
			}
		}
	}
}

void onConnect(int clientFD, clientMap *clientMap)
{
	if (clients_canAdd(clientMap))
	{
		int xPos = rand() % 600 - 300;
		int yPos = rand() % 400 - 200;

		clients_add(clientMap, clientFD, xPos, yPos);
	}
	else
	{
		int status = close(clientFD);
		assert(status == 0);
	}
}

void onDisconnect(size_t clientId, clientMap *clientMap)
{
	clients_remove(clientMap, clientId);
}

void onUpdate(clientMap *clientMap, events *events)
{
	idmap_each(clientMap->clients, i,
		idmap_get(clientMap->clients, i).xPos += 5;
		idmap_get(clientMap->clients, i).yPos += 0;
	)

	idmap_each(clientMap->clients, i,
		idmap_each(clientMap->clients, j,
			int status = sendUpdate(
				idmap_get(clientMap->clients, i).socketFD,
				idmap_get(clientMap->clients, j).id,
				idmap_get(clientMap->clients, j).xPos,
				idmap_get(clientMap->clients, j).yPos);

			if (status < 0)
			{
				event disconnectEvent;
				disconnectEvent.type = ON_DISCONNECT;
				disconnectEvent.ev.onDisconnect.clientId = i;

				rbuf_put((*events), disconnectEvent);
			}
		)
	)
}

int sendUpdate(int clientFD, size_t id, int xPos, int yPos)
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

	return net_send(clientFD, message, messageLength);
}
