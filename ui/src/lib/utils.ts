export function formatAddress(address: string) {
    const formatted = address.slice(0, 6) + '...' + address.slice(address.length - 4, address.length);
    return formatted;
}

export function areUint8ArraysEqual(array1: Uint8Array, array2: Uint8Array) {
    if (array1.length !== array2.length) {
        return false; // If the lengths are different, they can't be the same.
    }

    for (let i = 0; i < array1.length; i++) {
        if (array1[i] !== array2[i]) {
            return false; // If any element doesn't match, the arrays are not the same.
        }
    }

    return true; // All elements matched, arrays are the same.
}
