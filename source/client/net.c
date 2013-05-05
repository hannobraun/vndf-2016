#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <netdb.h>

#include "net.h"

int net_connect(const char *hostname, char *port)
{
	int status;

	struct addrinfo hints;
	memset(&hints, 0, sizeof hints);
	hints.ai_family   = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;

	struct addrinfo *servinfo;

	status = getaddrinfo(hostname, port, &hints, &servinfo);
	if (status != 0)
	{
		perror("Error getting address info");
		exit(1);
	}

	int socketFD = socket(
		servinfo->ai_family,
		servinfo->ai_socktype,
		servinfo->ai_protocol);
	if (socketFD == -1)
	{
		perror("Error creating socket");
		exit(1);
	}

	status = connect(socketFD, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		perror("Error connecting to server");
		exit(1);
	}

	freeaddrinfo(servinfo);

	return socketFD;
}

int net_receiveMessages(
	int socketFD,
	char messages[][MESSAGE_LENGTH],
	size_t maxMessage)
{
	char message[MESSAGE_LENGTH];

	ssize_t bytesReceived = recv(
		socketFD,
		message,
		MESSAGE_LENGTH,
		MSG_DONTWAIT);

	// TODO: Handle no bytes being available.
	// if (bytesReceived == -1 && (errno == EAGAIN || errno == EWOULDBLOCK))
	// {
	// 	return 0;
	// }

	if (bytesReceived == -1)
	{
		perror("Error receiving message");
		exit(1);
	}
	if (bytesReceived == 0)
	{
		printf("Connection closed while receiving.\n");
		exit(1);
	}

	int currentMessage = 0;
	size_t start = 0;
	for (size_t i = 0; (ssize_t)i < bytesReceived; i += 1)
	{
		if (message[i] == '\n')
		{
			size_t bytes = i - start;
			memcpy(messages[currentMessage], message + start, bytes);
			strcpy(messages[currentMessage] + bytes, "\n\0");
			currentMessage += 1;
			start = i + 1;
		}
	}

	return currentMessage;
}
