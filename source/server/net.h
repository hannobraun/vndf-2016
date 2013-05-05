typedef struct {
	int pollerFD;
	int serverFD;
} net;

net net_init(char *port);
int net_acceptClient(int serverFD);

// The following functions are usually not needed, but are exposed for test
// code.
int net_initSocket(char *port);
