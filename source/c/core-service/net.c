#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <netdb.h>
#include <sys/epoll.h>

#include "net.h"


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
