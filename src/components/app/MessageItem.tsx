import { useState } from "react";
import ReactJson from "react-json-view";

export interface MessageProps {
    msg: string,
    msgType?: MessageType,
}

export enum MessageType {
    Plain,
    Json,
}

export function Message(props: MessageProps) {
    const { msg, msgType } = props;

    let [innerType, setInnerType] = useState<MessageType>(msgType as MessageType);
    if (!innerType) {
        innerType = messageType(msg);
    }



    const messageRender = innerType === MessageType.Json
        ? (<ReactJson src={JSON.parse(msg)}
            displayDataTypes={false}
        />)
        : (<div className="font-medium">{msg}</div>)

    return (
        <div className="z-[-1]">
            {messageRender}
        </div>
    )
}

function messageType(message: string): MessageType {
    let messageType = MessageType.Plain;

    message = message.replaceAll(" ", "");

    const jsonPrefix = `{"`

    if (message.indexOf(jsonPrefix) === 0) {
        messageType = MessageType.Json;
    }

    return messageType;
}
