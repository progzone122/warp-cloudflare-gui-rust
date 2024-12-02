<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SwitchComponent from "./components/SwitchComponent.svelte";

  let switch1Ref: boolean = false;

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }

  const switch1Handle = (state: CustomEvent) => {
    if (state.detail) {
      
    }
  }
</script>

<div class="w-full h-screen">
  <div class="w-full flex flex-col gap-4 items-center py-5">
    <h1 class="text-6xl font-extrabold bg-gradient-to-r from-orange-600 via-orange-700 to-orange-400 bg-clip-text text-transparent">
      WARP
    </h1>

    <h1>Состояние переключателя: {switch1Ref ? 'Включен' : 'Выключен'}</h1>
    <SwitchComponent
            bind:isChecked={switch1Ref}
            on:change={switch1Handle}/>
  </div>
</div>