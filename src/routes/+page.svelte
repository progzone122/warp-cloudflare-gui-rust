<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SwitchComponent from "./components/SwitchComponent.svelte";
  import AlertComponent from "./components/AlertComponent.svelte";
  import Settings from "svelte-google-materialdesign-icons/Settings.svelte";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import {cubicInOut} from "svelte/easing";

  let is_connected: boolean = false;
  let switch1Loading: boolean = false;

  let connectionStatus: string | null = null;

  let alertRef: AlertComponent | null = null;


  const switch1Handle = async (state: CustomEvent) => {
    switch1Loading = true;
    try {
      await invoke(state.detail ? "connect_api" : "disconnect_api");
      is_connected = await invoke("is_connected_api");
      try {
        let st = await invoke("status_api");
        console.log(st);
      } catch (error) {
        alertRef.showAlert({
          code: error.code,
          details: error.details,
          message: error.message
        });
      }
      console.log(await invoke("status_api"))
      // connectionStatus = is_connected ? "Connected" : showError(await invoke("status_api"));
    } catch (e) {
      console.error(e);
    }
    switch1Loading = false;
  }
</script>

<div class="w-full fixed top-0 z-10">
  <AlertComponent bind:this={alertRef} />
</div>
<div class="w-full h-screen p-5 flex flex-col" transition:fly={{ duration: 300, x: 500, easing: cubicInOut }}>
  <div class="w-full flex flex-col flex-grow gap-4 items-center py-8">
    <h1 class="text-6xl font-extrabold bg-gradient-to-r from-orange-600 via-orange-700 to-orange-400 bg-clip-text text-transparent">
      WARP
    </h1>
    {is_connected}
    <SwitchComponent
            bind:isChecked={is_connected}
            bind:loading={switch1Loading}
            on:change={switch1Handle}/>
    {connectionStatus}
  </div>
  <div class="w-full flex flex-col justify-end items-end gap-6 mt-auto">
    <button type="button"
            class="w-10 h-10 flex justify-center items-center text-white bg-accent-500 hover:bg-accent-600 active:bg-accent-700 font-medium rounded-full text-sm"
            on:click={() => goto("settings")}>
      <Settings size="18" />
    </button>
  </div>
</div>