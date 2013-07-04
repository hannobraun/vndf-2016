#include <bdd/bdd.h>

#include <common/math.h>


spec
{
	describe("math module")
	{
		it("should implement addition")
		{
			fix52_12 a = 0x1800; // 1.5
			fix52_12 b = 0x1000; // 1.0
			fix52_12 c = 0x2800; // 2.5

			expectEqual(c, math_add(a, b));
		}

		it("should implement subtraction")
		{
			fix52_12 a = 0x2800; // 2.5
			fix52_12 b = 0x1000; // 1.0
			fix52_12 c = 0x1800; // 1.5

			expectEqual(c, math_sub(a, b));
		}

		it("should implement modulo")
		{
			fix52_12 a = 0x1800; // 1.5
			fix52_12 b = 0x1000; // 1.0
			fix52_12 c = 0x0800; // 0.5

			expectEqual(c, math_mod(a, b));
		}
	}
}


int main(int argc, char const *argv[])
{
	specFunction();
	return 0;
}
