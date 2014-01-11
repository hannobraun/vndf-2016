typedef struct {
	int pollerFD;
	int serverFD;
} net;

net net_init(char *port);
