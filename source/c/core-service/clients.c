#include "clients.h"


void clients_remove(clientMap *c, size_t id)
{
	if (idmap_contains(c->clients, id))
	{
		idmap_remove(c->clients, id);
		stack_push(c->idPool, id);
	}
}
