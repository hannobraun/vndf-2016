#include <stdio.h>
#include <string.h>

#include <client/net.h>
#include <server/net.h>


char *currentSpec;
char *currentExample;

#define describe(name, body) \
void spec_##name(void); \
void spec_##name() { \
	currentSpec = #name; \
	body \
}

#define it(description, body) \
currentExample = description; \
body

void fail(char *message);
void fail(char *message)
{
	printf("%s %s: %s\n",
		currentSpec, currentExample, message);
}

#define MAX_MESSAGES 3

describe(net_receiveMessages,
	it("should return all received messages",
		int serverServerFD = net_initSocket("34489");
		int clientServerFD = net_connect("localhost", "34489");
		int serverClientFD = net_acceptClient(serverServerFD);

		char *msg1 = "Message 1\n";
		char *msg2 = "Message 2\n";
		net_send(serverClientFD, msg1, strlen(msg1));
		net_send(serverClientFD, msg2, strlen(msg2));

		char messages[MAX_MESSAGES][MESSAGE_LENGTH];

		int numberOfMessages = net_receiveMessages(
			clientServerFD,
			messages,
			MAX_MESSAGES);

		if (numberOfMessages != 2)
		{
			fail("Expected to receive 2 messages.");
		}
		if (strcmp(messages[0], msg1) != 0)
		{
			fail("First message not identical.");
		}
		if (strcmp(messages[1], msg2) != 0)
		{
			fail("Second message not identical.");
		}
	)

	// Nothing to receive.
	// Receive partial message
	// More messages available then can be received
	// Received message exceeds maximum length
)

int main(int argc, char const *argv[])
{
	spec_net_receiveMessages();
	return 0;
}
