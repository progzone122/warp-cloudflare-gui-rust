<script lang="ts">
    import { goto } from "$app/navigation";
    import { Arrow_left } from "svelte-google-materialdesign-icons";
    import { getVersion } from '@tauri-apps/api/app';
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { cubicInOut } from "svelte/easing"

    let version: string | null = null;

    onMount(async () => {
        version = await getVersion();
    });
</script>
<div class="w-full h-screen flex flex-col gap-4 p-5" transition:fly={{ duration: 300, x: -500, easing: cubicInOut }}>
    <button type="button"
            class="w-10 h-10 flex justify-center items-center text-white bg-accent-500 hover:bg-accent-600 active:bg-accent-700 font-medium rounded-full text-sm p-2"
            on:click={() => goto("/")}>
        <Arrow_left size="24" />
    </button>
    <h1 class="text-3xl">Version: {version ?? "x.x.x"}</h1>
</div>