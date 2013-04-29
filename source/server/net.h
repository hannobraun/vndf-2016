typedef struct {
	int pollerFD;
	int serverFD;
} net;

net net_init(char *port);
int net_acceptClient(int serverFD);
