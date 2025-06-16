<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { setJwtToken } from '$lib/stores/JwtTokenStore.js';
    import { SupaStore, UserStore } from '$lib/stores/SupaStore.js';
    import { onMount } from 'svelte';
	import '$lib/styles/app.css';

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
				invalidateAll();
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

{@render children()}
