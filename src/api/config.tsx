import { invoke } from "@tauri-apps/api";
import { Request, Response } from "../proto/abi";
import bus from "../utils/EventBus";

export const DEFAULT_TIMEOUT = 10000;

export interface ConfigApi {
    configAdd(broker: string, name?: string): Promise<any>;
    configList(): Promise<any>;
    checkBroker(broker: string): Promise<Response>;
    topicList(broker: string): Promise<string[]>;
}


const config: ConfigApi = {
    configAdd: (broker, name) => {
        let addConfigPb: Request = { requestCmd: { oneofKind: "addconfig", addconfig: { cfg: { broker: broker, topic: "" } } } };
        return runWithTimeout(addConfigPb, DEFAULT_TIMEOUT);
    },

    configList: () => {
        let listConfigPb: Request = { requestCmd: { oneofKind: "listconfig", listconfig: {} } };
        return runWithTimeout(listConfigPb, DEFAULT_TIMEOUT);
    },

    checkBroker: function(broker: string): Promise<Response> {
        let checkBrokerPb: Request = { requestCmd: { oneofKind: "checkbroker", checkbroker: { broker: broker } } };
        return runWithTimeout(checkBrokerPb, DEFAULT_TIMEOUT);
    },
    topicList: function(broker: string): Promise<string[]> {
        let topicListPb: Request = { requestCmd: { oneofKind: "listtopics", listtopics: { broker: broker } } }
        return runWithTimeout(topicListPb, DEFAULT_TIMEOUT);
    }
}

function run(req: Request): Promise<any> {
    return invoke("run_command", { request: Request.toBinary(req).toString() }).then((rsp: any) => {
        return new Promise((resolve, reject) => {
            if (rsp?.status === "ok") {
                try {
                    rsp.data = JSON.parse(rsp?.data)
                } catch (e) {
                }
                resolve(rsp.data);
            } else {
                reject(new Error(`request failed, status : ${rsp?.status}`))
            }
        })
    })
}

export function runWithTimeout(req: Request, timeout: number): Promise<any> {
    return wrapPromiseWithSpiner(Promise.race([run(req), new Promise((_res, rej) => setTimeout(() => { rej("timeout") }, timeout))]), req);
}

function wrapPromiseWithSpiner(promise: Promise<any>, req: Request) {
    return new Promise((res, rej) => {
        bus.notice("loading", true);
        promise.then(d => {
            console.log(d, req)
            res(d)
        }).catch(e => {
            console.log("req", e, req)
            rej(e)
        }).finally(() => {
            bus.notice("loading", false);
        })
    })
}

export default config;
