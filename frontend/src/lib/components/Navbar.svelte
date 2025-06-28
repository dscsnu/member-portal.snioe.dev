<script lang="ts">
    import { page } from "$app/state";
    import { Boxes, Calendar, House, IdCard, Users } from "@lucide/svelte";
    import Auth from "$lib/components/Auth.svelte";

    const internalAnchors = [
        { id: 0, display: "Home", href: "/", icon: House },
        { id: 1, display: "Tenure Management", href: "/tenure", icon: Calendar },
        { id: 2, display: "Team Management", href: "/team", icon: Boxes },
        { id: 3, display: "Team Member Management", href: "/team-member", icon: Users },
        { id: 4, display: "Edit Profile Card", href: "/profile-card", icon: IdCard },
    ]
</script>

<nav class={`h-dvh w-[300px] flex flex-col gap-4 p-4 bg-slate-950 text-neutral-400`}>
    <hgroup class={`flex items-center gap-2 font-montserrat font-semibold`}>
        <img alt="GDSC Logo" src="/assets/images/gdsc-logo.png" class={`h-[36px] aspect-video`}>
        <div class={`flex flex-col`}>
            <h1>GDSC - SNIoE</h1>
            <h2>Member Portal</h2>
        </div>
    </hgroup>

    <ul class={`flex flex-col items-center gap-2 mt-8`}>
        {#each internalAnchors as anchor (anchor.id)}
            {@const Icon = anchor.icon}
            {@const currentPath = page.url.pathname}
            <li class={`w-full`}>
                <a
                    data-currentpage={currentPath === anchor.href || currentPath.startsWith(`${anchor.href}/`)}
                    href={anchor.href}
                    class={`w-full flex justify-start items-center gap-2 px-2 py-2 data-[currentpage="true"]:bg-slate-900/80 bg-slate-900/20 hover:bg-slate-800/40 rounded-md border-2 data-[currentpage="true"]:border-green-900 border-slate-800 transition-colors duration-300`}
                >
                    <Icon />
                    <p>{anchor.display}</p>
                </a>
            </li>
        {/each}
    </ul>

    <div class={`flex flex-col flex-grow justify-end gap-4`}>
        <hr class={`border-slate-900 mt-10`}/>

        <Auth />
    </div>
</nav>