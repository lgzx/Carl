import IconCard from "../components/IconCard";

export default function Welcome() {
    return (
        <div className="bg-zinc-100 min-h-screen flex flex-col justify-center min-w-full space-y-16">
            <div className="flex flex-col items-center space-y-4">
                <p className="flex font-mono text-4xl text-[#353535]  basis-full">Welcome back</p>
                <p className="flex font-mono text-xs text-[#7C7B7B]  basis-full">Carl :: Data manager tool</p>
            </div>

            <div className="flex flex-row justify-center items-center space-x-10">
                <IconCard text="Kafka" cardColor="#FF6060" />
                <IconCard text="Mysql" cardColor="#4EC8FF" />
                <IconCard text="Redis" cardColor="#FF69F9" />
            </div>

            <div className="flex rounded-md w-60 h-32 drop-shadow-md text-center">
            </div>
        </div>
    )
}
