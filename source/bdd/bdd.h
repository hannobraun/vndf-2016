#include <stdint.h>
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

void expectEqual(int64_t expected, int64_t actual);
void expectEqual(int64_t expected, int64_t actual)
{
	if (expected != actual)
	{
		char buffer[1024];
		int status = snprintf(
			buffer, sizeof buffer,
			"Expected %ld but was %ld.",
			expected, actual);
		if (status < 0)
		{
			printf("Error writing error message.\n");
			exit(1);
		}

		fail(buffer);
	}
}
