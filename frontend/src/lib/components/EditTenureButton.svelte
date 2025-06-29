<script lang="ts">
    import {
        validateInput,
        ValidationOptions,
    } from "$lib/directives/validateInput.svelte";
    import type { Tenure } from "$lib/types/tenure.type";
    import { Button, Dialog, Label, Tooltip, Switch } from "bits-ui";

    let { tenure }: { tenure: Tenure } = $props();

    // Form state
    let year: number = $state(tenure.year);
    let isActive: boolean = $state(tenure.is_active);
    let isOpen: boolean = $state(false);

    const handleSubmit = async () => {
        // isOpen = false;
    };
</script>

<Dialog.Root bind:open={isOpen}>
    <Dialog.Trigger>
        <Tooltip.Provider>
            <Tooltip.Root delayDuration={200}>
                <Tooltip.Trigger>
                    <button
                        class="p-2 rounded-lg hover:bg-neutral-800/50 transition-colors duration-200 group"
                        aria-label="Edit tenure"
                    >
                        {@render editSvg()}
                    </button>
                </Tooltip.Trigger>

                <Tooltip.Content side="right" sideOffset={8}>
                    <div
                        class="bg-neutral-900 border border-neutral-700 px-3 py-2 rounded-lg shadow-lg"
                    >
                        <p class="text-neutral-200 text-sm font-medium">
                            Edit Tenure
                        </p>
                    </div>
                </Tooltip.Content>
            </Tooltip.Root>
        </Tooltip.Provider>
    </Dialog.Trigger>

    <Dialog.Portal>
        <Dialog.Overlay
            class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/60 backdrop-blur-sm"
        />

        <Dialog.Content
            class="z-50 w-[480px] max-w-[95vw] fixed left-[50%] top-[50%] translate-x-[-50%] translate-y-[-50%] bg-neutral-900 border border-neutral-700 rounded-xl shadow-2xl text-neutral-100 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95"
        >
            <!-- Header -->
            <div class="px-6 py-5 border-b border-neutral-700">
                <Dialog.Title class="text-2xl font-semibold text-neutral-100">
                    Edit Tenure
                </Dialog.Title>
                <p class="text-sm text-neutral-400 mt-1">
                    Update the year and status for this tenure record
                </p>
            </div>

            <!-- Form Content -->
            <div class="px-6 py-6 space-y-6">
                <!-- Year Input -->
                <div class="space-y-2">
                    <Label.Root
                        for="tenure-year"
                        class="text-sm font-medium text-neutral-200"
                    >
                        Year
                    </Label.Root>

                    <input
                        id="tenure-year"
                        bind:value={year}
                        use:validateInput={{
                            allowed: [ValidationOptions.NUMERIC],
                            maxLength: 4,
                        }}
                        class="w-full h-11 px-4 bg-neutral-800 border border-neutral-600 rounded-lg text-neutral-100 placeholder-neutral-400 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 transition-colors"
                        placeholder="Enter year"
                        min="1900"
                        max="2100"
                    />
                </div>

                <!-- Status Switch -->
                <div class="space-y-2">
                    <Label.Root
                        for="tenure-status"
                        class="text-sm font-medium text-neutral-200"
                    >
                        Status
                    </Label.Root>

                    <div
                        class="flex items-center justify-between p-4 bg-neutral-800/50 border border-neutral-700 rounded-lg"
                    >
                        <div class="flex flex-col">
                            <span class="text-sm font-medium text-neutral-200">
                                {isActive ? "Active" : "Inactive"}
                            </span>

                            <span class="text-xs text-neutral-400">
                                {isActive
                                    ? "This tenure is currently active"
                                    : "This tenure is inactive"}
                            </span>
                        </div>

                        <Switch.Root
                            bind:checked={isActive}
                            class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:ring-offset-2 focus:ring-offset-neutral-900 {isActive
                                ? 'bg-blue-600'
                                : 'bg-neutral-600'}"
                        >
                            <Switch.Thumb
                                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {isActive
                                    ? 'translate-x-6'
                                    : 'translate-x-1'}"
                            />
                        </Switch.Root>
                    </div>
                </div>

                <!-- Tenure ID Display (Read-only) -->
                <div class="space-y-2">
                    <Label.Root class="text-sm font-medium text-neutral-200">
                        Tenure ID
                    </Label.Root>

                    <div
                        class="w-full h-11 px-4 bg-neutral-800/30 border border-neutral-700 rounded-lg flex items-center"
                    >
                        <code class="text-sm text-neutral-400 font-mono">
                            {tenure.id}
                        </code>
                    </div>
                </div>
            </div>

            <!-- Footer -->
            <div
                class="px-6 py-4 border-t border-neutral-700 flex gap-3 justify-end"
            >
                <Dialog.Close
                    class="px-4 py-2 text-sm font-medium text-neutral-300 hover:text-neutral-100 hover:bg-neutral-800 border border-neutral-600 hover:border-neutral-500 rounded-lg transition-colors"
                >
                    Cancel
                </Dialog.Close>

                <Button.Root
                    class="px-6 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:ring-2 focus:ring-blue-500/50 focus:ring-offset-2 focus:ring-offset-neutral-900 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                    onclick={() => handleSubmit()}
                >
                    Save Changes
                </Button.Root>
            </div>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>

{#snippet editSvg()}
    <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="h-5 w-5 stroke-2 stroke-neutral-400 group-hover:stroke-blue-400 fill-none transition-colors duration-200"
    >
        <path d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
        <path
            d="M18.375 2.625a1 1 0 0 1 3 3l-9.013 9.014a2 2 0 0 1-.853.505l-2.873.84a.5.5 0 0 1-.62-.62l.84-2.873a2 2 0 0 1-.506-.852z"
        />
    </svg>
{/snippet}
