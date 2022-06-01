import { faAdd, faCheck, faGift, faHeartBroken } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { sendNotification } from "@tauri-apps/api/notification";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { WebviewWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import config from "../api/config";
import { ConfigItem, KafkaConfigItem } from "../components/KafkaConfigItem";
import { FloatIcon, FloatIconProp } from "../components/welcome/FloatIcon";

export default function Welcome() {
    const nav = useNavigate()

    const [configItemList, setConfigList] = useState<ConfigItem[]>([]);

    function refreshList() {
        config.configList().then((rsp: any) => {
            let brokers = rsp?.data.map((x: any) => {
                return {
                    broker: x?.broker
                }
            })
            setConfigList(x => brokers);
        });
    }

    useEffect(() => {
        refreshList()
    }, [])


    const icons: FloatIconProp[] = [
        { icon: faAdd, text: "+", callback: addConfig }
    ];


    const configListElement = configItemList.map(x => {
        return (
            <div className="transition-slow  pl-5 rounded-lg  p-3" onClick={_ => openKafka(x.broker)}>
                <KafkaConfigItem item={x} />
            </div>
        )
    })

    return (
        <div className="grid grid-cols-3  min-h-screen max-h-screen text-sm">
            {/* left  */}
            <div className="col-span-1 flex flex-row items-center justify-center bg-gray-100">
                <div className="w-full ">
                    <div className="grid grid-rows-8 gap-1 items-center text-center">
                        <div className="row-span-3 flex justify-center">
                            <img className="w-32 rounded-full" src="https://cdnb.artstation.com/p/assets/images/images/012/148/701/large/brendan-le-glaunec-bloggologo.jpg?1533277817" alt="" onClick={() => nav("/app")} />
                        </div>
                        <div className="row-span-1 text-gray-500">Welcome to Carl tools</div>
                        <div className="row-span-1 gap-2 flex items-center justify-center">
                            <FontAwesomeIcon icon={faGift} className="text-red-500" />
                            <FontAwesomeIcon icon={faHeartBroken} />
                            <FontAwesomeIcon icon={faCheck} />
                        </div>
                    </div>
                    <FloatIcon icons={icons} />
                </div>
            </div>



            {/* right  */}
            <div className=" col-span-2  max-h-screen overflow-auto pt-3">
                {configListElement}
            </div>

        </div>
    )
}

function addConfig() {
    console.log("123123")
}

function openKafka(broker: string) {
    checkKafka(broker).then(e => {
        if (e) {
            const webview = new WebviewWindow('theUniqueLabel', {
                url: `/app?broker=${broker}`,
                focus: true
            })
        }
    })
}

async function checkKafka(broker: string) {
    let rsp = await config.checkBroker(broker)
    return rsp?.data
}
