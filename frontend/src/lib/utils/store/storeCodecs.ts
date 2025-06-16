export const jsonCodec = {
    encode: JSON.stringify,
    decode: JSON.parse,
};

export const stringCodec = {
    encode: (s: string) => s,
    decode: (s: string) => s,
};