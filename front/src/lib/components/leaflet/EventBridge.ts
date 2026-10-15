export type LeafletHandlers = Record<string, (e: any) => void>;

export default class EventBridge {
	entity: any;
	eventHandlers: { event: string; handler: (e: any) => void }[];

	constructor(entity: any, handlers: LeafletHandlers = {}) {
		this.entity = entity;

		this.eventHandlers = Object.entries(handlers).map(([event, handler]) => ({ event, handler }));
		this.eventHandlers.forEach(({ event, handler }) => {
			entity.on(event, handler);
		});
	}

	unregister() {
		this.eventHandlers.forEach((entry) => {
			this.entity.off(entry.event, entry.handler);
		});
	}
}
