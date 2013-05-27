#include <assert.h>
#include <stdint.h>

#define rbuf(type) \
struct \
{ \
	uint64_t first; \
	uint64_t last; \
	size_t   cap; \
	type     *buffer; \
}

#define rbuf_init(buf, capacity) \
buf.first = 0; \
buf.last = 0; \
buf.cap = capacity; \
buf.buffer = malloc(capacity * sizeof buf.buffer[0]);

#define rbuf_put(buf, v) \
assert(rbuf_available(buf) > 0); \
buf.buffer[buf.last % buf.cap] = v; \
buf.last += 1;

#define rbuf_get(buf, pv) \
assert(rbuf_size(buf) > 0); \
*pv = buf.buffer[buf.first % buf.cap]; \
buf.first += 1;

#define rbuf_size(buf) (buf.last - buf.first)
#define rbuf_available(buf) (buf.cap - rbuf_size(buf))
