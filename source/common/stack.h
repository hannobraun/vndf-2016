#include <assert.h>


#define stack(type) \
struct \
{ \
	size_t cap; \
	size_t size; \
	type   *elems; \
}

#define stack_init(s, capacity) \
s.cap = capacity; \
s.size = 0; \
s.elems = malloc(capacity * sizeof s.elems[0]);

#define stack_push(s, v) \
assert(s.size < s.cap); \
s.elems[s.size] = v; \
s.size += 1;

#define stack_pop(s, pv) \
assert(s.size > 0); \
s.size -= 1; \
*pv = s.elems[s.size];
