typedef struct {
	int    socketFD;
	size_t id;
	int    xPos;
	int    yPos;
} client;

typedef idmap(client) clientMap;
