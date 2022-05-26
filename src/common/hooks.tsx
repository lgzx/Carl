import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { AddConfig, Cmd, ListConfig, PullMessage, Request } from "../proto/abi";

export function useInvoke(method: string, payload: any) {
    const [data, setData] = useState()
    const [error, setError] = useState(new Error(`failed invoke ${method}`))
    invoke(method, payload).then((e: any) => { setData(origin => e) }).catch((e: Error | any) => { setError(e) })

    return [data, error]
}

export async function useEvent(channal: string, callback: Function) {
    const unlisten = await listen(channal, event => {
        callback(event);
    });

    return unlisten;
}

export async function emmit(channal: string, payload: any) {
    return emit(channal, payload);
}


export async function emitRequestAsync(r: Request) {
    console.error(Request.toBinary(r));
    return emit("event", Request.toBinary(r));
}
