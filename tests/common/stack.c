#include <bdd/bdd.h>

#include <common/stack.h>


spec
{
	describe("stack")
	{
		it("should pop the last value that was pushed")
		{
			stack(int) s;
			stack_init(s, 3);

			stack_push(s, 1);
			stack_push(s, 2);

			int x, y;

			stack_pop(s, &x);
			stack_pop(s, &y);

			expectEqualInt(2, x);
			expectEqualInt(1, y);
		}

		// The following behavior has been implemented, but is not specified
		// here (because I don't know how):
		// * Assert that elements can't be pushed onto a full stack.
		// * Assert that elements can't be popped from ana empty stack.
	}
}


int main(int argc, char const *argv[])
{
	specFunction();
	return 0;
}
