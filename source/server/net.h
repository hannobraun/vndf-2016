int net_initSocket(void);
int net_initPoller(void);
void net_registerAccept(int pollerFD, int serverFD);
int net_acceptClient(int serverFD);
