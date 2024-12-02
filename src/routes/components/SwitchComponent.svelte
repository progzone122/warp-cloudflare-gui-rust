<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let isChecked: boolean;
    export let loading: boolean = false;

    const dispatch = createEventDispatcher();

    // Обновляем состояние переключателя и отправляем событие
    const setState = (state: boolean): void => {
        dispatch("change", state);
    };
</script>

<div class="relative inline-block">
    <input
            id="switch-component-1"
            type="checkbox"
            bind:checked={isChecked}
            disabled={loading}
            on:change={() => setState(isChecked)}
    class="peer appearance-none w-32 h-16 bg-slate-100 rounded-full checked:bg-orange-500 cursor-pointer transition-colors duration-300"
    />
    <label
            for="switch-component-1"
            class="absolute top-0 left-0 w-16 h-16 bg-white rounded-full border border-slate-300 shadow-sm transition-transform duration-300 peer-checked:translate-x-16 cursor-pointer">
    </label>
</div>