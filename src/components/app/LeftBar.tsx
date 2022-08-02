import { faSearch, faSearchMinus, faTable } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useEffect, useState } from "react";
import { setEnvironmentData } from "worker_threads";
import config from "../../api/config";
import { kafka } from "../../api/kafka";
import bus, { EVENT_TOPIC_SUBSCRIBE, EVENT_TOPIC_SUBSCRIBED, EVENT_TOPIC_UNSUBSCRIBE } from "../../utils/EventBus";

export interface TopicList {
    broker: string
}

export default function LeftBar(props: TopicList) {

    let { broker } = props;

    /* left-top-bar  */

    const menuBar = (
        <div className="flex gap-1 items-center justify-around min-w-full text-xs font-bold">
            <div>
                topics
            </div>
            <div>
                queries
            </div>
        </div>
    )


    /* search bar  */

    const [search, setSearch] = useState("");

    const searchBar = (
        <div className="flex gap-2 items-center bg-white justify-center  mx-4 pl-2 text-gray-600 rounded-md w-full">
            <FontAwesomeIcon icon={faSearch} className="text-xs" />
            <input className="outline-none bg-transparent w-full" value={search} onChange={(state) => setSearch(_ => state.target.value)} />
        </div>
    )


    /* listitem  */
    const [topics, setTopics] = useState<string[]>([]);
    useEffect(() => {
        config.topicList(broker).then(rsp => setTopics(_ => rsp))
        bus.register(EVENT_TOPIC_SUBSCRIBED, (payload: any) => {
            setChannel(_ => payload?.channel);
        });
    }, []);

    const [selectNumber, setSelectNumber] = useState(-1);
    const [currentChannel, setChannel] = useState("");

    const clickTopic = (index: number, topic: string) => {
        setSelectNumber(old => {
            bus.notice(EVENT_TOPIC_SUBSCRIBE, { topic: topic, broker: broker });
            if (old !== -1) {
                kafka.unsubscribe(currentChannel)
            }
            return index;
        });
    }

    const filtered = topics.filter(x => search ? x.includes(search.toLowerCase()) : true)

    const listItem = filtered.map((topic, index) => {
        const selected = selectNumber == index;

        return (
            <div className={`pl-4 flex items-center overflow-y-scroll p-1 ${selected ? "bg-blue-500 text-white" : ""}`} onClick={() => clickTopic(index, topic)
            }>
                <FontAwesomeIcon icon={faTable} className={`mr-3  ${selected ? "text-white" : "text-blue-500"}`} />
                <div className="text-sm  whitespace-nowrap">{topic}</div>
            </div >
        )
    });






    return (
        <div className="flex gap-1 flex-col max-h-screen min-h-full overflow-hidden bg-gray-100">
            <div className="flex cate h-8  items-center ">
                {menuBar}
            </div>

            <div className="flex search h-8 items-center">
                {searchBar}
            </div>

            <div className="list pt-2  overflow-x-scroll no-scrollbar">
                {listItem}
            </div>
        </div>
    )
}
