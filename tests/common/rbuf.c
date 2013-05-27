#include <bdd/bdd.h>

#include <common/rbuf.h>


spec
{
	describe("rbuf")
	{
		it("should act as a FIFO queue")
		{
			rbuf(int) buf;
			rbuf_init(buf, 2);

			int x, y, z;

			rbuf_put(buf, 1);
			rbuf_put(buf, 2);

			rbuf_get(buf, &x);

			rbuf_put(buf, 3);

			rbuf_get(buf, &y);
			rbuf_get(buf, &z);

			expectEqual(1, x);
			expectEqual(2, y);
			expectEqual(3, z);
		}

		it("should return the number of values in the buffer")
		{
			rbuf(int) buf;
			rbuf_init(buf, 2);

			expectEqual(0, (int)rbuf_size(buf));

			rbuf_put(buf, 1);
			rbuf_put(buf, 2);

			expectEqual(2, (int)rbuf_size(buf));
		}

		it("should return the number of available cells")
		{
			rbuf(int) buf;
			rbuf_init(buf, 2);

			expectEqual(2, (int)rbuf_available(buf));

			rbuf_put(buf, 1);
			rbuf_put(buf, 2);

			expectEqual(0, (int)rbuf_available(buf));
		}

		// The following requirements are not tested here (cause I don't know
		// how), but are part of the implementation (at the time of writing):
		// * Assert that put cannot overwrite the beginning of the buffer.
		// * Assert that get cannot read empty cells.
	}
}


int main(int argc, char const *argv[])
{
	specFunction();
	return 0;
}
