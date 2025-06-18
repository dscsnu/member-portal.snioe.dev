import createCookiePersistentStore from "$lib/utils/store/createCookiePersistentStore";
import { stringCodec } from "$lib/utils/store/storeCodecs";

export const JWT_TOKEN_NAME = 'gdsc-member-portal-jwt-token';
const {
    store: JwtTokenStore, set: setJwtToken
} = createCookiePersistentStore<string>({
    tokenName: JWT_TOKEN_NAME,
    ...stringCodec,
    secure: true,
});

export { JwtTokenStore, setJwtToken };