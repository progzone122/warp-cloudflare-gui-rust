<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SwitchComponent from "./components/SwitchComponent.svelte";

  let is_connected: boolean = false;
  let switch1Loading: boolean = false;

  const switch1Handle = async (state: CustomEvent) => {
    switch1Loading = true;
    try {
      await invoke(state.detail ? "connect_api" : "disconnect_api");
      is_connected = await invoke("is_connected_api");
    } catch (e) {
      console.error(e);
    }
    switch1Loading = false;
  }
</script>

<div class="w-full h-screen">
  <div class="w-full flex flex-col gap-4 items-center py-8">
    <h1 class="text-6xl font-extrabold bg-gradient-to-r from-orange-600 via-orange-700 to-orange-400 bg-clip-text text-transparent">
      WARP
    </h1>
    {is_connected}
    <SwitchComponent
            bind:isChecked={is_connected}
            bind:loading={switch1Loading}
            on:change={switch1Handle}/>
  </div>
</div>