#define idmap_entry(type) struct { int isOccupied; type value; }

#define idmap(type) \
struct { \
	size_t cap; \
	idmap_entry(type) *elems; \
}

#define idmap_put(map, k, v) \
map[k].isOccupied = 1; \
map[k].value = v;
