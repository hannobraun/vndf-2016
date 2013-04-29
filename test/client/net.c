#include <stdio.h>

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

describe(net_receiveMessages,
	it("should return all received messages",
		// initialize networking
		// send a few message
		// call receiveMessages
		// expect the messages to be there

		fail("Not implemented.");
	)
)

int main(int argc, char const *argv[])
{
	spec_net_receiveMessages();
	return 0;
}
