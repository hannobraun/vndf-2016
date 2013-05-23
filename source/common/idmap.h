#define idmap_entry(type) struct { int isOccupied; type value; }

#define idmap(type) \
struct { \
	size_t cap; \
	idmap_entry(type) *elems; \
}

#define idmap_put(map, k, v) \
if (k >= map.cap) \
{ \
	printf("Can't add id (%lu). Value too high. Largest possible id: %lu\n", \
		id, map.cap - 1); \
	exit(1); \
} \
map.elems[k].isOccupied = 1; \
map.elems[k].value = v;
