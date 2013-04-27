typedef struct {
	int pollerFD;
	int serverFD;
} net;

net net_init(void);
int net_acceptClient(int serverFD);
