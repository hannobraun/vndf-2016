int net_connect(const char *hostname, char *port);
ssize_t net_receive(int socketFD, char *buffer, size_t bufferSize);
