#include <assert.h>
#include <limits.h>
#include <math.h>
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
#include "clients.h"
#include "events.h"
#include "net.h"


void logOutput(char *s);

void schedule_update(events *events);
void handle_events(events *events, clientMap *clientMap, int frameTimeInMs);
void onConnect(int clientFD, clientMap *clientMap);
void onDisconnect(size_t clientId, clientMap *clientMap, events *events);
void onUpdate(clientMap *clientMap, events *events, double dTimeInS);


int main(int argc, char const *argv[])
{
	logOutput("Core Service started.\n");

	srand((unsigned int)time(NULL));

	net net = net_init("34481");

	events events;
	rbuf_init(events, 16);

	clientMap clientMap;
	clients_initClientMap(&clientMap, 4);

	while (true)
	{
		const int frameTimeInMs = 50;

		#define MAX_EVENTS 1024
		struct epoll_event pollEvents[MAX_EVENTS];

		int numberOfEvents = epoll_wait(
			net.pollerFD,
			pollEvents,
			MAX_EVENTS,
			frameTimeInMs);
		assert(numberOfEvents != -1);

		for (int i = 0; i < numberOfEvents; i += 1)
		{
			int clientFD = net_acceptClient(net.serverFD);

			event connectEvent;
			connectEvent.type = ON_CONNECT;
			connectEvent.onConnect.clientFD = clientFD;

			rbuf_put(events, connectEvent);
		}

		schedule_update(&events);
		handle_events(&events, &clientMap, frameTimeInMs);
	}
}
