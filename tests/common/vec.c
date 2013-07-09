#include <bdd/bdd.h>

#include <common/vec.h>


void expectEqualVec(vec2 expected, vec2 actual);
void expectEqualVec(vec2 expected, vec2 actual)
{
	expectEqualDouble(expected.x, actual.x);
	expectEqualDouble(expected.y, actual.y);
}


spec
{
	describe("vec")
	{
		it("should add two vectors")
		{
			vec2 a = {2, 3};
			vec2 b = {1, 2};

			vec2 c = {3, 5};

			expectEqualVec(c, vec_add(a, b));
		}
	}
}


int main(int argc, char const *argv[])
{
	specFunction();
	return 0;
}
