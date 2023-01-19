export default class EventBridge {
	entity: any;
	eventHandlers: any[];

	constructor(entity: any, dispatch: any, events: any[] = []) {
		this.entity = entity;

		this.eventHandlers = [];
		if (events) {
			const eventMap: any = {};
			events.forEach((event) => {
				if (!(event in eventMap)) {
					const handler = function (e: any) {
						dispatch(event, e);
					};
					this.eventHandlers.push({
						event: event,
						handler: handler
					});
					entity.on(event, handler);
					eventMap[event] = handler;
				}
			});
		}
	}

	unregister() {
		this.eventHandlers.forEach((entry) => {
			this.entity.off(entry.event, entry.handler);
		});
	}
}
