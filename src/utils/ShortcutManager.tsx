let keyStack: Set<string> = new Set();

let keyCallbacks: Map<Set<string>, any> = new Map();

const getHumenCode = (e: KeyboardEvent) => {
    const map: { [key: number]: string } = {
        91: "cmd",
        16: "shift",
        17: "ctrl",
    }

    if (map[e.keyCode]) {
        return map[e.keyCode]
    }

    return e.key;

}

function setsAreEqual<T>(a: Set<T>, b: Set<T>): boolean {
    if (a.size !== b.size) {
        return false;
    }

    return Array.from(a).every(element => {
        return b.has(element);
    });
}

window.onkeydown = e => {
    let k = getHumenCode(e);
    keyStack.add(k)
    keyCallbacks.forEach((v, k) => {
        if (setsAreEqual(k, keyStack)) {
            v();
        }
    })
}

window.onkeyup = e => {
    let k = getHumenCode(e);
    keyStack.delete(k)
}

function mapSetSet<K extends Set<V>, V>(map: Map<K, V>, k: K, v: V) {
    let found = false;
    map.forEach((v1, k1) => {
        if (setsAreEqual(k1, k)) {
            found = true;
        }
    })
    if (!found) {
        map.set(k, v);
    }
}


export const shortcuts = (key: string, callback: () => void): () => void => {
    let keyCombine = new Set(key.split("+"));
    mapSetSet(keyCallbacks, keyCombine, callback);

    return () => {
        keyCallbacks.delete(keyCombine)
    }
}
