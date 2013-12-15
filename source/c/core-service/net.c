#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <netdb.h>
#include <sys/epoll.h>

#include "net.h"


int initPoller(void);
void registerAccept(int pollerFD, int serverFD);


int net_acceptClient(int serverFD)
{
	struct sockaddr_storage remote_address;
	socklen_t address_size = sizeof remote_address;

	int clientFD = accept(
		serverFD,
		(struct sockaddr *)&remote_address,
		&address_size);

	return clientFD;
}

int net_send(int clientFD, char *message, size_t messageLength)
{
	ssize_t bytesSent = send(clientFD, message, messageLength, MSG_NOSIGNAL);
	if (bytesSent < 0)
	{
		return -1;
	}
	if ((size_t)bytesSent != messageLength)
	{
		printf("Only sent %ld of %lu bytes.\n", bytesSent, messageLength);
		exit(1);
	}

	return 0;
}
