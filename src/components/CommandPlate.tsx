import { faAngleRight, faArrowRight, faFaceAngry } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { register } from "@tauri-apps/api/globalShortcut"
import hotkeys from "hotkeys-js";
import { useCallback, useEffect, useRef, useState } from "react";
import { shortcuts } from "../utils/ShortcutManager";

export default function CommandPlate() {
    const [show, setShow] = useState(false);
    const [cmd, setCmd] = useState("");
    const inputRef = useRef<HTMLInputElement>(null);


    useEffect(() => {
        register("cmd+p", () => {
            setShow(s => !s)
            setCmd(s => "")
            inputRef?.current?.focus()
        })
    }, [])


    const getCommandList = (search: string) => {

        const data: { icon?: any, cmd: string, fn?: () => void }[] = [{ cmd: "test-cmd" }, { cmd: "consume-kafak", fn: () => { alert("123") } }];

        let element = [];


        for (let cmd of data.filter(x => { return search ? x.cmd.includes(search) : false; })) {
            element.push(
                (
                    <div className="flex items-center hover:bg-gray-100 transition py-1" onClick={() => { if (cmd.fn) { cmd.fn() } }} >
                        <FontAwesomeIcon icon={faFaceAngry} className="text-green-200 text-lg mr-3 ml-2" />
                        <div className="text-gray-600"> {cmd.cmd} </div>
                    </div>
                )
            )
        }

        return (
            <div className="min-h-min overflow-auto ">
                {element}
            </div>
        )
    }

    return (
        <div className={`left-0 right-0 top-16 absolute bg-white  m-auto  w-2/3 min-h-min h-10 shadow-xl rounded-lg ${show ? "" : "hidden"} `}>
            <div className="commandInput flex items-center py-3 p-5 ">
                <FontAwesomeIcon icon={faAngleRight} className="mr-4 text-gray-500" />
                <input ref={inputRef} className=" w-full h-full text-xl bg-transparent outline-none text-gray-600" value={cmd} onChange={(e) => setCmd(e.target.value)} />
            </div>

            <div className="commandList">
                {getCommandList(cmd)}
            </div>

        </div>
    )
}
