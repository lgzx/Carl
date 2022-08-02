interface FloatProps {
    className?: string;
    children?: React.ReactNode;
}
export function FloatItem(prop: FloatProps) {
    let { className, children } = prop;
    return (
        <div className={`sticky h-0 ${className}`} >
            {children}
        </div >
    )
}
