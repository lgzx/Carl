import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

export function useInvoke(method: string, payload: any) {
    const [data, setData] = useState()
    const [error, setError] = useState(new Error(`failed invoke ${method}`))
    invoke(method, payload).then((e: any) => { setData(origin => e) }).catch((e: Error | any) => { setError(e) })

    return [data, error]
}
