import { useState } from "react";

interface PaginatorProps {
    onSelected?: (index: number) => void;
    total: number;
    per?: number;
}

const DefaultProps: PaginatorProps = {
    total: 0,
    per: 10,
}
export function Paginator(props: PaginatorProps) {

    let { total, per, onSelected } = { ...DefaultProps, ...props };


    console.log(total)

    const [n, setN] = useState(-1);

    let pageNum = Math.ceil(total / (per as number));

    if (total < (per as number)) {
        return (<div></div>)
    }

    let list = Array.from(new Array(pageNum).keys()).map(i => {
        let item = i !== n ? (
            <a href="#" className="py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">{i + 1}</a>
        ) : (
            <a href="#" aria-current="page" className="z-10 py-2 px-3 leading-tight text-blue-600 bg-blue-50 border border-blue-300 hover:bg-blue-100 hover:text-blue-700 dark:border-gray-700 dark:bg-gray-700 dark:text-white">{i + 1}</a>
        );
        return (
            <li onClick={_ => {
                onSelected?.(i as number)
                setN(_ => i)
            }}>
                {item}
            </li>
        )
    });


    return (
        <div aria-label="Page navigation example" className="w-5">
            <ul className="flex items-center -space-x-px">
                {list}
            </ul>
        </div>
    )
}
