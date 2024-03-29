export interface EventBus {
    registry: Map<String, ((payload: any) => void)[]>;
    register(evt: string, fn: (payload: any) => void): void;
    notice(evt: string, payload: any): void;
}

export const EVENT_TOPIC_SUBSCRIBE = "topic-subscribe";
export const EVENT_TOPIC_SUBSCRIBED = "topic-subscribed";
export const EVENT_TOPIC_UNSUBSCRIBE = "topic-unsubscribe";

const bus: EventBus = {
    registry: new Map(),
    register: function(evt: string, fn: (payload: any) => void): void {
        if (this.registry.has(evt)) {
            let list = this.registry.get(evt);
            if (list) {
                list.push(fn);
            }
        } else {
            this.registry.set(evt, [fn]);
        }
    },
    notice: function(evt: string, payload: any): void {
        let list = this.registry.get(evt);
        list?.forEach(x => {
            x(payload);
        })
    }
};

export default bus;
