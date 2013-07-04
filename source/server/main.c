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
#include <common/math.h>
#include <common/rbuf.h>
#include <common/stack.h>
#include "clients.h"
#include "events.h"
#include "net.h"


void log(char *s);

void onConnect(int clientFD, clientMap *clientMap);
void onDisconnect(size_t clientId, clientMap *clientMap, events *events);
void onUpdate(clientMap *clientMap, events *events);
int sendUpdate(int clientFD, size_t id, fix xPos, fix yPos);
int sendRemove(int clientFD, size_t id);


int main(int argc, char const *argv[])
{
	log("Server started.\n");

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
					onDisconnect(
						event.ev.onDisconnect.clientId,
						&clientMap,
						&events);
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

void log(char *s)
{
	time_t t = time(NULL);
	char *ts = ctime(&t);
	ts[strlen(ts) - 1] = '\0';

	printf("%s  %s", ts, s);
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

void onDisconnect(size_t clientId, clientMap *clientMap, events *events)
{
	clients_remove(clientMap, clientId);

	idmap_each(clientMap->clients, i,
		int status = sendRemove(
			idmap_get(clientMap->clients, i).socketFD,
			clientId);

		if (status < 0)
		{
			event disconnectEvent;
			disconnectEvent.type = ON_DISCONNECT;
			disconnectEvent.ev.onDisconnect.clientId = i;

			rbuf_put((*events), disconnectEvent);
		}
	)
}

void onUpdate(clientMap *clientMap, events *events)
{
	idmap_each(clientMap->clients, i,
		client *client = &idmap_get(clientMap->clients, i);
		client->xPos = math_add(client->xPos, math_fromInt(5));
		client->yPos = math_add(client->yPos, math_fromInt(0));
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

int sendUpdate(int clientFD, size_t id, fix xPos, fix yPos)
{
	char message[256];
	int status = snprintf(
		message + 1, sizeof message - 1,
		"UPDATE id: %lu, pos: (%ld, %ld)",
		id, math_toLong(xPos), math_toLong(yPos));
	assert(status >= 0);
	assert((size_t)status <= sizeof message);

	size_t messageLength = strlen(message + 1) + 1;
	assert(messageLength <= CHAR_MAX);
	message[0] = (char)messageLength;

	return net_send(clientFD, message, messageLength);
}

int sendRemove(int clientFD, size_t id)
{
	char message[256];
	int status = snprintf(
		message + 1, sizeof message - 1,
		"REMOVE id: %lu",
		id);
	assert(status >= 0);
	assert((size_t)status <= sizeof message);

	size_t messageLength = strlen(message + 1) + 1;
	assert(messageLength <= CHAR_MAX);
	message[0] = (char)messageLength;

	return net_send(clientFD, message, messageLength);
}
