import { faAdd, faArrowDown, IconDefinition } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export interface FloatIconProp {
    icon?: IconDefinition,
    text: string,
    callback: () => void,
}

export function FloatIcon(data: { icons?: FloatIconProp[], className?: string }) {
    let { icons, className } = data;

    icons = icons ? icons : []

    const buttons = icons.map(x => {
        return (
            <div className="flex items-center mx-1" onClick={(e) => x.callback()}>
                {
                    x.icon ? <FontAwesomeIcon icon={x.icon} /> :
                        <div>{x.text}</div>
                }

            </div>
        )
    });
    return (
        <div className={`absolute right-10 bottom-8 ${className}`}>
            <div className="flex gap-1 items-center justify-center shadow-lg rounded-sm py-1 px-2">
                {buttons}
            </div>
        </div>
    )
}
