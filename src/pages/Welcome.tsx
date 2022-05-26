import IconCard from "../components/IconCard";
import { useNavigate } from "react-router-dom"
import { emit, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { AddConfig, Cmd, ListConfig, Request } from "../proto/abi";
import { emitRequestAsync } from "../common/hooks";


export default function Welcome() {
    return (
        <div className="">
        </div>
    )
}
