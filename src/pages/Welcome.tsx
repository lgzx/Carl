import IconCard from "../components/IconCard";
import { useNavigate } from "react-router-dom"


export default function Welcome() {
    let nav = useNavigate()

    return (
        <div className="flex min-h-screen flex-col justify-center items-center bg-slate-50 rounded-lg py-10 ">
            <div className="flex flex-col items-center py-10 justify-center space-y-10 border-none drop-shadow-xl bg-white shadow-lg w-9/12 rounded-lg">
                <div className="flex flex-col items-center space-y-4">
                    <p className="flex font-mono text-4xl text-[#353535]  basis-full">Welcome back</p>
                    <p className="flex font-mono text-xs text-[#7C7B7B]  basis-full">Carl :: Data manager tool</p>
                </div>

                <div className="flex flex-row justify-center items-center space-x-10">
                    <IconCard text="Kafka" cardColor="#FF6060" />
                    <IconCard text="Mysql" cardColor="#4EC8FF" />
                    <IconCard text="Redis" cardColor="#FF69F9" />
                </div>

                <div className="grid grid-cols-8 gap-10  rounded-md h-32 drop-shadow-md  content-around w-9/12 mx-auto  place-items-center">
                    <div className="col-span-5 grid grid-flow-row">
                        <input type="" name="" />
                        <input type="" name="" />
                    </div>
                    <div className="grid col-span-2">
                        <span onClick={e => nav("app")}>Go for it</span>
                    </div>
                </div>
            </div>
        </div>
    )
}
