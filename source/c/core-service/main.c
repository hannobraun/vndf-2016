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

int net_number_of_events(net *net, int frameTimeInMs);
void handle_connects(int numberOfEvents, int serverFD, events *events);
void schedule_update(events *events);
void handle_events(events *events, clientMap *clientMap, int frameTimeInMs);


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

		int numberOfEvents = net_number_of_events(&net, frameTimeInMs);
		handle_connects(numberOfEvents, net.serverFD, &events);
		schedule_update(&events);
		handle_events(&events, &clientMap, frameTimeInMs);
	}
}
