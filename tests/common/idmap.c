#include <bdd/bdd.h>

#include <common/idmap.h>


spec
{
	// The idmap code has not been developed test-driven, so this spec only
	// covers a subset of functionality.

	describe("idmap")
	{
		it("should indicate whether an id is contained in the map.")
		{
			idmap(int) m;
			idmap_init(m, 2);

			idmap_put(m, 0, 1);

			expectEqualInt(1, idmap_contains(m, 0));
			expectEqualInt(0, idmap_contains(m, 1));
		}
	}
}


int main(int argc, char const *argv[])
{
	specFunction();
	return 0;
}
