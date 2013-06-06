typedef struct {
	int type;
	union ev {
		struct {} updateEvent;
	};
} event;
