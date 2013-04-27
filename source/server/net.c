#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <netdb.h>
#include <sys/epoll.h>

#include "net.h"


#define CLIENT_ACCEPT_BACKLOG 1024


int net_initSocket()
{
	int status;

	struct addrinfo hints;
	memset(&hints, 0, sizeof hints);
	hints.ai_family   = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;
	hints.ai_flags    = AI_PASSIVE;

	struct addrinfo *servinfo;

	status = getaddrinfo(NULL, "34481", &hints, &servinfo);

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

	int yes=1;
	if (setsockopt(socketFD,SOL_SOCKET,SO_REUSEADDR,&yes,sizeof(int)) == -1)
	{
		perror("Error setting socket option");
		exit(1);
	}

	status = bind(socketFD, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		perror("Error binding socket");
		exit(1);
	}

	status = listen(socketFD, CLIENT_ACCEPT_BACKLOG);
	if (status != 0)
	{
		perror("Error listening on socket");
		exit(1);
	}

	freeaddrinfo(servinfo);

	return socketFD;
}

int net_initPoller()
{
	int pollerFD = epoll_create(1);
	if (pollerFD < 0)
	{
		perror("Error initiating epoll");
		exit(1);
	}

	return pollerFD;
}

void net_registerAccept(int pollerFD, int serverFD)
{
	struct epoll_event event;
	event.events = EPOLLIN;
	int status = epoll_ctl(pollerFD, EPOLL_CTL_ADD, serverFD, &event);
	if (status != 0)
	{
		perror("Error registering server socket with epoll");
		exit(1);
	}
}

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
