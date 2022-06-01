import { faSearch, faSearchMinus, faTable } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useState } from "react";
import { setEnvironmentData } from "worker_threads";

export default function LeftBar() {

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
    const lists: string[] = [
        "private-domain",
        "private-domain-testprivate-domain-testprivate-domain-testprivate-domain-test",
        "private-test",
        "private-domain-puzu",
    ];

    const [selectNumber, setSelectNumber] = useState(-1);

    const listItem = lists.filter(x => search ? x.includes(search.toLowerCase()) : true).map((x, index) => {
        const selected = selectNumber == index;

        return (
            <div className={`pl-4 flex items-center p-1 ${selected ? "bg-blue-500 text-white" : ""}`} onClick={() => setSelectNumber(x => index)
            }>
                <FontAwesomeIcon icon={faTable} className={`mr-3  ${selected ? "text-white" : "text-blue-500"}`} />
                <div className="text-sm  whitespace-nowrap">{x}</div>
            </div >
        )
    });






    return (
        <div className="flex gap-1 flex-col min-h-full overflow-hidden bg-gray-100">
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
