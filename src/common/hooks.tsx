import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { AddConfig, ListConfig, PullMessage, Request } from "../proto/abi";


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
