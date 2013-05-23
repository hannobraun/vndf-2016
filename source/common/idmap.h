#define idmap_entry(type) struct { int isOccupied; type value; }
#define idmap_memSize(type, cap) cap * sizeof(idmap_entry(type))

#define idmap(type) \
struct { \
	size_t cap; \
	idmap_entry(type) *elems; \
}

#define idmap_init(type, map, capacity) \
map.cap = capacity; \
map.elems = malloc(idmap_memSize(type, capacity)); \
memset(map.elems, 0, idmap_memSize(type, capacity));

#define idmap_get(map, k) map.elems[k].value

#define idmap_put(map, k, v) \
if (k >= map.cap) \
{ \
	printf("Can't add id (%lu). Value too high. Largest possible id: %lu\n", \
		k, map.cap - 1); \
	exit(1); \
} \
map.elems[k].isOccupied = 1; \
map.elems[k].value = v;

#define idmap_each(map, i, body) \
for (size_t i = 0; i < map.cap; i += 1) \
{ \
	if (map.elems[i].isOccupied) \
	{ \
		body \
	} \
}
