#include <stdio.h>
#include <stdlib.h>

char *currentSpec;
char *currentExample;

#define spec \
void specFunction(void); \
void specFunction()

#define describe(name) currentSpec = name;
#define it(description) currentExample = description;

void fail(char *message);
void fail(char *message)
{
	printf("%s %s: %s\n", currentSpec, currentExample, message);
}

void expectEqual(int expected, int actual);
void expectEqual(int expected, int actual)
{
	if (expected != actual)
	{
		char buffer[1024];
		int status = snprintf(
			buffer, sizeof buffer,
			"Expected %d but was %d.",
			expected, actual);
		if (status < 0)
		{
			printf("Error writing error message.\n");
			exit(1);
		}

		fail(buffer);
	}
}
