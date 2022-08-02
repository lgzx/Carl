import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { kafka } from "../../api/kafka";
import bus, { EVENT_TOPIC_SUBSCRIBE, EVENT_TOPIC_SUBSCRIBED } from "../../utils/EventBus";
import { FloatItem } from "../FloatItem";
import { Paginator } from "../Paginator";
import { Message } from "./MessageItem";

export function MessageList() {

    const [channel, setChannel] = useState("");

    useEffect(() => {
        bus.register(EVENT_TOPIC_SUBSCRIBE, (conf: any) => {
            setMsg(_ => []);
            kafka.pullMessage(conf?.broker, conf?.topic).then(e => {
                bus.notice(EVENT_TOPIC_SUBSCRIBED, { channel: e })
                let unlis = listen(e, evt => {
                    setMsg(list => [evt.payload as string, ...list]);
                });
            })
        })
    }, [])

    /* header */
    const header = (
        <div className="flex items-center gap-2 text-sm h-8  py-1 bg-gray-200 bg-opacity-80 font-medium" style={{ zIndex: 1 }}>
            <div className="px-6 ">
                id
            </div>
            <div className="px-6 ">
                time
            </div>
            <div className="px-6  w-full text-center">
                message
            </div>
        </div>
    )

    const [messageList, setMsg] = useState<string[]>([]);
    const [page, setPage] = useState(0);
    const perPage = 10;
    const start = page * perPage;
    const end = (page + 1) * perPage;
    const listItems = messageList.slice(start, end).map((x, index) => {
        return (

            <div className="flex items-center gap-2 text-sm  py-1">
                <div className="px-6 ">
                    {index + 1}
                </div>
                <div className="px-6">
                    {new Date().toLocaleTimeString()}
                </div>
                <div className="px-6  w-full">
                    <Message msg={x} />
                </div>
            </div>
        )
    });

    return (
        <div className="h-0">
            <div className="fixed top-0 w-full">
                {header}
            </div>
            <div className="mt-8 relative">
                {listItems}
                <FloatItem className="bottom-20  m-auto">
                    <Paginator total={messageList.length} onSelected={e => setPage(_ => e)} />
                </FloatItem>
            </div>
        </div>
    )
}
