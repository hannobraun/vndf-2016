#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include <common/dynamics.h>
#include <common/idmap.h>
#include <common/stack.h>


typedef struct {
	int    socketFD;
	size_t id;
	vec2   pos;
} client;

typedef struct {
	idmap(client) clients;
	stack(size_t) idPool;
} clientMap;


void clients_initClientMap(clientMap *c, size_t cap);
bool clients_canAdd(clientMap *c);
void clients_add(clientMap *c, int socketFD, double xPos, double yPos);
void clients_remove(clientMap *c, size_t id);
