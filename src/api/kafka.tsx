import { Request } from "../proto/abi";
import { DEFAULT_TIMEOUT, runWithTimeout } from "./config";


export interface KafkaApi {
    pullMessage(broker: string, topic: string): Promise<any>;
    unsubscribe(channel: string): Promise<any>;
}

export const kafka: KafkaApi = {
    pullMessage: function(broker: string, topic: string): Promise<any> {
        let pullMessagePb: Request = { requestCmd: { oneofKind: "pullmessage", pullmessage: { cfg: { broker, topic } } } };
        return runWithTimeout(pullMessagePb, DEFAULT_TIMEOUT);
    },
    unsubscribe: function(channel: string): Promise<any> {
        let unsubscribe: Request = { requestCmd: { oneofKind: "closechannel", closechannel: { channel } } }

        return runWithTimeout(unsubscribe, DEFAULT_TIMEOUT);
    }
}
