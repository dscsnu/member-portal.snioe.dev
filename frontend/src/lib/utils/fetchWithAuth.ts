import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { JWT_TOKEN_NAME, JwtTokenStore } from "$lib/stores/JwtTokenStore";
import type { Cookies } from "@sveltejs/kit";
import { get } from "svelte/store";

interface FetchOptions extends RequestInit {
    headers?: Record<string, string>;
};

// remove all trailing /
const CLEANED_API_URL = PUBLIC_BACKEND_URL.replace(/\/+$/, "");

export const fetchWithAuthBrowser = async (route: string, options: FetchOptions = {}): Promise<Response> => {
    const jwtToken = get(JwtTokenStore);
    // remove all / from start
    const cleanedRoute = route.replace(/^\/+/, "");

    return fetch(`${CLEANED_API_URL}/${cleanedRoute}`, {
        ...options,
        headers: {
            ...options.headers,
            'Authorization': `Bearer ${jwtToken}`,
        },
    });
}

export const fetchWithAuthServer = async (cookies: Cookies, route: string, options: FetchOptions = {}): Promise<Response> => {
    const jwtToken = cookies.get(JWT_TOKEN_NAME);
    const cleanedUrl = PUBLIC_BACKEND_URL.replace(/\/+$/, "");
    const cleanedRoute = route.replace(/^\/+/, "");

    return fetch(`${cleanedUrl}/${cleanedRoute}`, {
        ...options,
        headers: {
            ...options.headers,
            'Authorization': `Bearer ${jwtToken}`,
        },
    });
}