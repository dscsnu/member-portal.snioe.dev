<script lang="ts">
    import { page } from "$app/state";
    import { Boxes, Calendar, House, IdCard, Users } from "@lucide/svelte";
    import { Button } from "bits-ui";
    import Auth from "$lib/components/Auth.svelte";

    const internalAnchors = [
        {
            id: 0,
            display: "Home",
            href: "/",
            icon: House,
        },
        {
            id: 1,
            display: "Tenure Management",
            href: "/tenure",
            icon: Calendar,
        },
        {
            id: 2,
            display: "Team Management",
            href: "/team",
            icon: Boxes,
        },
        {
            id: 3,
            display: "Team Member Management",
            href: "/team-member",
            icon: Users,
        },
        {
            id: 4,
            display: "Edit Member Card",
            href: "/member-card",
            icon: IdCard,
        },
    ];
</script>

<nav
    class={`h-dvh min-w-[325px] flex flex-col gap-4 text-neutral-400 border-r-2 border-neutral-600`}
>
    <hgroup
        class={`h-[75px] border-b-2 border-neutral-600 flex justify-center items-center gap-2 font-semibold`}
    >
        <img
            alt="GDSC Logo"
            src="/assets/images/gdsc-logo.png"
            class={`h-[24px] aspect-video`}
        />

        <h1 class={`w-max`}>GDSC - SNIoE Member Portal</h1>
    </hgroup>

    <ul class={`flex flex-col items-center gap-2 px-4`}>
        {#each internalAnchors as anchor (anchor.id)}
            {@const Icon = anchor.icon}
            {@const currentPath = page.url.pathname}
            <li class={`w-full`}>
                <Button.Root
                    data-currentpage={currentPath === anchor.href ||
                        currentPath.startsWith(`${anchor.href}/`)}
                    href={anchor.href}
                    class={`w-full flex justify-start items-center gap-2 px-2 py-2 data-[currentpage="true"]:bg-neutral-800/40 hover:bg-neutral-800/40 rounded-md transition-colors duration-300`}
                >
                    <Icon />
                    <p>{anchor.display}</p>
                </Button.Root>
            </li>
        {/each}
    </ul>

    <div class={`flex flex-col flex-grow justify-end px-4 pb-4`}>
        <hr class={`border-neutral-600 mb-3`} />

        <Auth />
    </div>
</nav>
