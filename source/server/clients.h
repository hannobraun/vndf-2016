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
