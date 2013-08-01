#include <assert.h>
#include <string.h>


#define idmap_entry(type) struct { int isOccupied; type value; }
#define idmap_memSize(map) (map.cap * sizeof map.elems[0])

#define idmap(type) \
struct { \
	size_t cap; \
	idmap_entry(type) *elems; \
}

#define idmap_init(map, capacity) \
map.cap = capacity; \
map.elems = malloc(idmap_memSize(map)); \
memset(map.elems, 0, idmap_memSize(map));

#define idmap_get(map, k) map.elems[k].value

#define idmap_put(map, k, v) \
assert(k < map.cap); \
map.elems[k].isOccupied = 1; \
map.elems[k].value = v;

#define idmap_contains(map, k) map.elems[k].isOccupied

#define idmap_remove(map, k) map.elems[k].isOccupied = 0;

#define idmap_each(map, i, body) \
for (size_t i = 0; i < map.cap; i += 1) \
{ \
	if (map.elems[i].isOccupied) \
	{ \
		body \
	} \
}
