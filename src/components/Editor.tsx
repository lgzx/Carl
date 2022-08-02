import { default as MonoEditor } from "@monaco-editor/react";
import { FloatIcon } from "./welcome/FloatIcon";

export function Editor() {
    return (
        <div className="h-full">
            <MonoEditor language="sql" />
            <FloatIcon icons={[{ text: "Produce", callback: () => { alert("run") } }]} className="bg-blue-500 rounded-md text-white" />
        </div>
    )
}
