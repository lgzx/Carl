interface Props {
    circleColor?: string
    cardColor?: string
    icon?: string
    text?: string
}

let Default: Props = {
    text: "kafka",
    circleColor: "#ffffff",
    cardColor: "#4ec8ff",
    icon: "",
}
export default function IconCard(props: Props) {
    props = { ...Default, ...props }

    return (
        <div className="flex flex-col items-center rounded-2xl space-y-1 w-20 py-3 drop-shadow-lg" style={{ backgroundColor: props.cardColor }} >
            <div className="rounded-full flex h-10 w-10 drop-shadow-xl" style={{ backgroundColor: props.circleColor }}></div>
            <div className="text-center flex text-sm mt-0 text-white">{props.text}</div>
        </div >
    )
}
