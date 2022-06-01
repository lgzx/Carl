import { invoke } from "@tauri-apps/api";
import { Request, Response } from "../proto/abi";

const DEFAULT_TIMEOUT = 1000;

export interface ConfigApi {
    configAdd(broker: string, name?: string): Promise<any>;
    configList(): Promise<any>;
    checkBroker(broker: string): Promise<Response>;
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
        let checkBrokerPb: Request = { requestCmd: { oneofKind: "checkbroker", checkbroker: { broker: broker } } }
        return runWithTimeout(checkBrokerPb, DEFAULT_TIMEOUT);
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
                resolve(rsp);
            } else {
                reject(new Error(`request failed, status : ${rsp?.status}`))
            }
        })
    })
}

function runWithTimeout(req: Request, timeout: number): Promise<any> {
    return Promise.race([run(req), new Promise((_res, rej) => setTimeout(() => { rej("timeout") }, timeout))]);
}

export default config;
