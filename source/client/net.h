#define MESSAGE_LENGTH 128

int net_connect(const char *hostname, char *port);
int net_receiveMessages(
	int socketFD,
	char messages[][MESSAGE_LENGTH],
	size_t maxMessage);
