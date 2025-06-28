import { redirect, type Handle } from "@sveltejs/kit";

const authGuard: Handle = async ({ event, resolve }) => {
    const currentPath = event.url.pathname;

    if (currentPath.startsWith('/api')) {
        return resolve(event);
    }

    // if (!event.locals.user && !currentPath.startsWith('/auth')) {
    //     return redirect(303, '/auth');
    // }

    return resolve(event);
}

export default authGuard;