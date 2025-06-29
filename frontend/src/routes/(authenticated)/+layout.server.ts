import { fetchWithoutAuth } from "$lib/utils/fetchWithAuth";
import { z } from "zod";
import { TenureSchema, type Tenure } from "$lib/types/tenure.type";
import type { LayoutServerLoad } from "./$types";

const TenureArraySchema = z.array(TenureSchema)

const fetchTenures = async (): Promise<Tenure[]> => {
    const response = await fetchWithoutAuth('/tenure', {
        method: 'GET'
    });

    if (response.status !== 200) {
        throw new Error("Failed to fetch tenure data");
    }

    const json = await response.json();

    const parsed = TenureArraySchema.safeParse(json);

    if (!parsed.success) {
        throw new Error("Invalid tenure data format");
    }

    return parsed.data;
}

// @ts-ignore
export const load: LayoutServerLoad = async () => {

    return {
        tenures: await fetchTenures()
    }
}