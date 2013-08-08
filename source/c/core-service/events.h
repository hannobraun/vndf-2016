#define ON_CONNECT    0
#define ON_DISCONNECT 1
#define ON_UPDATE     2

typedef struct {
	int dummy; // Empty structs are not supported by standard C.
} events_update;
typedef struct {
	int clientFD;
} events_connect;
typedef struct {
	size_t clientId;
} events_disconnect;

typedef struct {
	int type;
	union ev {
		events_connect    onConnect;
		events_disconnect onDisconnect;
		events_update     onUpdate;
	} ev;
} event;

typedef rbuf(event) events;
