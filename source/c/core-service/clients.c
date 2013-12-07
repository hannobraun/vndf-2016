#include "clients.h"


bool clients_canAdd(clientMap *c)
{
	return c->idPool.size > 0;
}

void clients_add(clientMap *c, int socketFD, vec2 pos, vec2 vel)
{
	size_t clientId;
	stack_pop(c->idPool, &clientId);

	body body = {pos, vel};

	client client = {socketFD, clientId, body};
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
