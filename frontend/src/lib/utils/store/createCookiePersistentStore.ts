import { browser } from "$app/environment";
import { writable, type Writable } from "svelte/store";
import getCookie from "$lib/utils/getCookie";

interface CookiePersistentStoreConfig<T> {
    tokenName: string;
    initialValue?: T | null;
    maxAgeSec?: number;
    encode: (data: T) => string;
    decode: (raw: string) => T;
    secure?: boolean;
}

const createCookiePersistentStore = <T>({
    tokenName,
    initialValue = null,
    maxAgeSec = 60 * 60 * 24 * 365,
    encode,
    decode,
    secure = false,
}: CookiePersistentStoreConfig<T>): {
    store: Writable<T | null>;
    set: (data: T | null) => void;
} => {
    const getInitialData = (): T | null => {
        if (!browser) return initialValue;

        const cookieData = getCookie(document.cookie, tokenName);
        if (cookieData) {
            try {
                return decode(cookieData);
            } catch (e) {
                console.error(`Error decoding data for ${tokenName}: `, e);
            }
        }

        return initialValue;
    }

    const store = writable<T | null>(getInitialData());

    const setData = (data: T | null): void => {
        if (!browser) return;

        const secureAttributes = secure ? "SameSite=Strict;" : "";

        if (data !== null) {
            try {
                const encoded = encode(data);
                document.cookie = `${tokenName}=${encoded};path=/;max-age=${maxAgeSec};${secureAttributes}`;
            } catch (e) {
                console.error(`Error encoding data for ${tokenName}: `, e);
            }
        } else {
            document.cookie = `${tokenName}=null;path=/;expires=Thu, 01 Jan 1970 00:00:01 GMT;${secureAttributes}`
        }

        store.set(data);
    }

    return {
        store,
        set: setData,
    };
}

export default createCookiePersistentStore;