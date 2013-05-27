#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include <common/idmap.h>
#include <common/stack.h>


typedef struct {
	int    socketFD;
	size_t id;
	int    xPos;
	int    yPos;
} client;

typedef struct {
	idmap(client) clients;
	stack(size_t) idPool;
} clientMap;


void clients_initClientMap(clientMap *c, size_t cap);
bool clients_canAdd(clientMap *c);
