typedef struct {
	int pollerFD;
	int serverFD;
} net;

net net_init(char *port);
int net_acceptClient(int serverFD);
int net_send(int clientFD, char *message, size_t messageLength);
