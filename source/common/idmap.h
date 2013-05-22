#define idmap_entry(type) struct { int isOccupied; type value; }

#define idmap_put(map, k, v) \
map[k].isOccupied = 1; \
map[k].value = v;
