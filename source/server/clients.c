#include "clients.h"

void clients_initClientMap(clientMap *c, size_t cap)
{
	idmap_init(c->clients, cap);
	stack_init(c->idPool, cap);

	for (size_t i = cap; i > 0; i -= 1)
	{
		stack_push(c->idPool, i - 1)
	}
}
