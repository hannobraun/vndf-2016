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
