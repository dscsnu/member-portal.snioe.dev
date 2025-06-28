<script lang="ts">
    import { PUBLIC_BACKEND_URL } from "$env/static/public";
    import { fetchWithAuthBrowser } from "$lib/utils/fetchWithAuth";

    const ENDPOINT = "/test/ping";

    const handleClick = async () => {
        fetchWithAuthBrowser(ENDPOINT, {
            method: "GET",
        });
    };

    const handleClick2 = async () => {
        fetch(`${PUBLIC_BACKEND_URL}${ENDPOINT}`, {
            method: "GET",
        });
    };

    const handleClick3 = async () => {
        fetch(`${PUBLIC_BACKEND_URL}${ENDPOINT}`, {
            method: "GET",
            headers: { Authorization: "Bearer sex" },
        });
    };
</script>

<div class={`flex flex-col justify-center items-center h-dvh w-dvw`}>
    <button
        onclick={() => handleClick()}
        class={`border-2 rounded-md px-4 py-3`}
    >
        Send Request
    </button>

    <button
        onclick={() => handleClick2()}
        class={`border-2 rounded-md px-4 py-3`}
    >
        Send Request Without Header
    </button>

    <button
        onclick={() => handleClick3()}
        class={`border-2 rounded-md px-4 py-3`}
    >
        Send Request With Random Header
    </button>
</div>
