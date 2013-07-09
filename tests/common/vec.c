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

		it("should subtract a vector from another")
		{
			vec2 a = {2, 3};
			vec2 b = {1, 2};

			vec2 c = {1, 1};

			expectEqualVec(c, vec_sub(a, b));
		}

		it("should scale a vector with a scalar")
		{
			vec2 a   = {2, 3};
			double s = 2;

			vec2 b = {4, 6};

			expectEqualVec(b, vec_scale(a, s));
		}
	}
}


int main(int argc, char const *argv[])
{
	specFunction();
	return 0;
}
