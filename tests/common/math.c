#include <bdd/bdd.h>

#include <common/math.h>


spec
{
	describe("math module")
	{
		it("should implement conversion from int")
		{
			int i = 5;
			fix f = 0x5000; // 5.0

			expectEqual(f, math_fromInt(i));
		}

		it("should implement addition")
		{
			fix a = 0x1800; // 1.5
			fix b = 0x1000; // 1.0
			fix c = 0x2800; // 2.5

			expectEqual(c, math_add(a, b));
		}

		it("should implement subtraction")
		{
			fix a = 0x2800; // 2.5
			fix b = 0x1000; // 1.0
			fix c = 0x1800; // 1.5

			expectEqual(c, math_sub(a, b));
		}

		it("should implement modulo")
		{
			fix a = 0x1800; // 1.5
			fix b = 0x1000; // 1.0
			fix c = 0x0800; // 0.5

			expectEqual(c, math_mod(a, b));
		}
	}
}


int main(int argc, char const *argv[])
{
	specFunction();
	return 0;
}
