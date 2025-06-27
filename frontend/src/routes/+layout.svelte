<script lang="ts">
	import '$lib/styles/app.css';
    import { invalidate } from '$app/navigation';
    import { setJwtToken } from '$lib/stores/JwtTokenStore.js';
    import { SupaStore, UserStore } from '$lib/stores/SupaStore.js';
    import { onMount } from 'svelte';
    import Navbar from '$lib/components/Navbar.svelte';

	let { data, children } = $props();
	let { supabase, session, user } = $derived(data);

	onMount(() => {
		const {
			data: { subscription }
		} = supabase.auth.onAuthStateChange((event, newSession) => {
			if (
				newSession?.expires_at !== session?.expires_at ||
				event === 'SIGNED_OUT'
			) {
				setJwtToken(null);
				invalidate("supabase:auth")
			}

			if (
				["SIGNED_IN", "TOKEN_REFRESHED"].includes(event) &&
				session?.access_token
			) {
				setJwtToken(session.access_token);
			}
		});

		SupaStore.set(supabase);
		UserStore.set(user);

		return () => subscription.unsubscribe();
	});
</script>

<div class={`h-dvh w-dvw flex font-roboto bg-neutral-200 text-neutral-950`}>
	<Navbar />
	{@render children()}
</div>
