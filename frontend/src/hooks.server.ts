import type { Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

import createSupabase from "$lib/server/middleware/createSupabase";
import authGuard from "$lib/server/middleware/authGuard";

export const handle: Handle = sequence(createSupabase, authGuard);