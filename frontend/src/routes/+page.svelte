<script lang="ts">
    import getCustomClaims from '$lib/utils/supabase/getCustomClaims.js';

    let { data } = $props();
    let { supabase, user } = $derived(data);

    const handleSignIn = async () => {
        await supabase.auth.signInWithOAuth({
            provider: 'google',
            options: {
                redirectTo: `${window.location.origin}/api/auth/callback`,
                queryParams: {
                    access_type: 'offline',
                    prompt: 'consent',
                },
            },
        });
    }

    const handleSignOut = async () => {
        await supabase.auth.signOut();
    }
</script>

<div class={`h-dvh w-dvw flex flex-col justify-center items-center`}>
    {#if user}
        <div class={`flex flex-col`}>
            {user.email}
            <pre>{JSON.stringify(getCustomClaims(user), null, 2)}</pre>
        </div>
        <button onclick={() => handleSignOut()} class={`border-2 px-4 py-2 rounded-sm`}>
            Sign Out
        </button>
    {:else}
        <button onclick={() => handleSignIn()} class={`border-2 px-4 py-2 rounded-sm`}>
            Sign In
        </button>
    {/if}
</div>