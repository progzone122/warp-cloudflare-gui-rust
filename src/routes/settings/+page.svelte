<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import { getVersion } from '@tauri-apps/api/app';
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { cubicInOut } from "svelte/easing"

    import { Arrow_left } from "svelte-google-materialdesign-icons";
    import { Person_add } from "svelte-google-materialdesign-icons"
    import { Person_remove } from "svelte-google-materialdesign-icons"
    import AlertComponent from "../components/AlertComponent.svelte";

    let version: string | null = null;

    let alertRef: AlertComponent | null = null;

    onMount(async () => {
        version = await getVersion();
    });

    const registerHandle = async () => {
        try {
            let res = await invoke("register_account_api");
            alertRef.showAlert(res);
        } catch (e) {
            alertRef.showAlert(e);
        }
    }
    const deleteHandle = async () => {
        try {
            let res = await invoke("delete_account_api");
            alertRef.showAlert(res);
        } catch (e) {
            alertRef.showAlert(e);
        }
    }
</script>
<div class="w-full fixed top-0 z-10">
    <AlertComponent bind:this={alertRef} />
</div>
<div class="w-full h-screen flex flex-col gap-4 p-5" transition:fly={{ duration: 300, x: -500, easing: cubicInOut }}>
    <button type="button"
            class="w-10 h-10 flex justify-center items-center text-white bg-accent-500 hover:bg-accent-600 active:bg-accent-700 font-medium rounded-full text-sm p-2"
            on:click={() => goto("/")}>
        <Arrow_left size="24" />
    </button>
    <div class="w-full flex flex-col gap-2">
        <h1 class="text-2xl font-bold">Version: {version ?? "x.x.x"}</h1>
        <hr class="w-full h-1 mx-auto border-0 rounded dark:bg-gray-700">
        <h1 class="text-2xl font-bold">Account: </h1>
        <div class="flex gap-4">
            <button type="button"
                    class="h-10 flex justify-center items-center text-white bg-accent-500 hover:bg-accent-600 active:bg-accent-700 font-medium rounded-lg text-sm p-2"
                    on:click={() => registerHandle()}>
                 <div class="flex items-center gap-2 px-2">
                     <Person_add size="24" />
                     <h4 class="font-bold">Register</h4>
                 </div>
            </button>
            <button type="button"
                    class="h-10 flex justify-center items-center text-white bg-accent-500 hover:bg-accent-600 active:bg-accent-700 font-medium rounded-lg text-sm p-2"
                    on:click={() => deleteHandle() }>
                <div class="flex items-center gap-2 px-2">
                    <Person_remove size="24" />
                    <h4 class="font-bold">Delete</h4>
                </div>
            </button>
        </div>
    </div>

</div>