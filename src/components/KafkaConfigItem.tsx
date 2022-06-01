export interface ConfigItem {
    broker: string
}

export function KafkaConfigItem(props: { item: ConfigItem }) {

    const { item } = props;

    return (
        <div className="flex flex-row gap-6 items-center">
            <div className="imgc "><img className="w-8" src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1200px-Rust_programming_language_black_logo.svg.png" alt="" /></div>
            <div className="item col-span-4">
                <div className="">
                    {item.broker}
                </div>
                <div className="text-xs text-gray-500">
                    {item.broker}
                </div>
            </div>
        </div>

    )
}
