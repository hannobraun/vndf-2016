#define ON_CONNECT 0
#define ON_UPDATE 1

typedef struct {} events_update;
typedef struct {
	int clientFD;
} events_connect;

typedef struct {
	int type;
	union ev {
		events_update  onUpdate;
		events_connect onConnect;
	} ev;
} event;
