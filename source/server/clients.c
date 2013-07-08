#include "clients.h"


void clients_initClientMap(clientMap *c, size_t cap)
{
	idmap_init(c->clients, cap);
	stack_init(c->idPool, cap);

	for (size_t i = cap; i > 0; i -= 1)
	{
		stack_push(c->idPool, i - 1);
	}
}

bool clients_canAdd(clientMap *c)
{
	return c->idPool.size > 0;
}

void clients_add(clientMap *c, int socketFD, fix xPos, fix yPos)
{
	size_t clientId;
	stack_pop(c->idPool, &clientId);

	client client = {socketFD, clientId, fix_toLong(xPos), fix_toLong(yPos)};
	idmap_put(c->clients, clientId, client);
}

void clients_remove(clientMap *c, size_t id)
{
	if (idmap_contains(c->clients, id))
	{
		idmap_remove(c->clients, id);
		stack_push(c->idPool, id);
	}
}
