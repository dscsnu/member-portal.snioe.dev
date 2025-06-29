import { z } from "zod";

export const TenureSchema = z.object({
    id: z.string(),
    year: z.number(),
    is_active: z.boolean()
});
export type Tenure = z.infer<typeof TenureSchema>