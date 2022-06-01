import { useEffect, useState } from "react";

export function MessageList() {

    /* header */
    const header = (
        <div className="flex items-center gap-2 text-sm  py-1 border-2">
            <div className="px-6 border-r-2">
                id
            </div>
            <div className="px-6 border-r-2">
                time
            </div>
            <div className="px-6  w-full text-center">
                message
            </div>
        </div>
    )

    /* list item  */
    const tmp: string[] = [
        "justify-center justify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-center",
        "justify-center justify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-center",
        "justify-center justify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-center",
        "justify-center justify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-centerjustify-center"
    ]
    const [messageList, setMsg] = useState(tmp);
    const listItems = messageList.map((x, index) => {
        return (

            <div className="flex items-center gap-2 text-sm  py-1 ">
                <div className="px-6 ">
                    {index + 1}
                </div>
                <div className="px-6">
                    {new Date().toLocaleTimeString()}
                </div>
                <div className="px-6  w-full">
                    {x}
                </div>
            </div>
        )
    });

    return (
        <div className="h-0">
            <div>
                {header}
            </div>
            <div>
                {listItems}
            </div>
        </div>
    )
}
