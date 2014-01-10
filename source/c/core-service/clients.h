#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include <common/idmap.h>
#include <common/stack.h>


typedef struct {
	double x;
	double y;
} vec2;

typedef struct {
	vec2 pos;
	vec2 vel;
} body;

typedef struct {
	int    socketFD;
	size_t id;
	body   ship;
} client;

typedef struct {
	idmap(client) clients;
	stack(size_t) idPool;
} clientMap;


void clients_initClientMap(clientMap *c, size_t cap);
