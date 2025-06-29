<script lang="ts">
    import { Edit, RouteOff, SquarePen, Trash2 } from "@lucide/svelte";
    import type { PageProps } from "./$types.js";
    import { Dialog, Tooltip } from "bits-ui";
    import EditTenureButton from "$lib/components/EditTenureButton.svelte";

    let { data }: PageProps = $props();
    let { tenures } = $derived(data);

    const handleEdit = async (tenureId: string) => {};

    const handleDelete = async (tenureId: string) => {};

    const handleAdd = async (tenureId: string) => {};
</script>

<main class={`min-h-dvh w-full flex flex-col`}>
    <h2
        class={`h-[75px] w-full flex justify-start items-center px-4 text-4xl border-b-2 border-neutral-600 text-neutral-200`}
    >
        Tenure Management
    </h2>

    <div class={`flex-grow grid place-items-center`}>
        <div class={`w-1/2 rounded-md border-2 border-neutral-900`}>
            <table class={`w-full`}>
                <thead class={`bg-neutral-900`}>
                    <tr class={`text-neutral-100 font-light`}>
                        <th class={`py-1 font-semibold w-[400px]`}>ID</th>
                        <th class={`py-1 font-semibold`}>Year</th>
                        <th class={`py-1 font-semibold`}>Status</th>
                        <th class={`py-1 font-semibold`}>Actions</th>
                    </tr>
                </thead>

                <tbody class={``}>
                    {#each tenures as t (t.id)}
                        <tr>
                            <td class={`py-2 text-center`}>{t.id} </td>

                            <td class={`py-2 text-center`}>{t.year}</td>

                            <td class={`py-2 grid place-items-center`}>
                                {#if t.is_active}
                                    <div
                                        class={`flex items-center gap-2 rounded-md`}
                                    >
                                        <div
                                            class={`h-[8px] aspect-square rounded-full bg-green-500`}
                                        ></div>
                                        <p>Active</p>
                                    </div>
                                {:else}
                                    <div
                                        class={`flex items-center gap-2 rounded-md`}
                                    >
                                        <div
                                            class={`h-[8px] aspect-square rounded-full bg-neutral-500`}
                                        ></div>
                                        <p>Inactive</p>
                                    </div>
                                {/if}
                            </td>

                            <td class={``}>
                                <div class={`flex`}>
                                    <EditTenureButton tenure={t} />
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>
</main>

<svelte:head>
    <title>Member Portal | Tenure Management</title>
</svelte:head>
