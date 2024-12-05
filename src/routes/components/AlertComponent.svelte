<script lang="ts">
    import { createEventDispatcher } from "svelte";

    import BugReport from "svelte-google-materialdesign-icons/Bug_report.svelte";
    import CheckBox from "svelte-google-materialdesign-icons/Check_circle.svelte";
    import Close from "svelte-google-materialdesign-icons/Close.svelte";

    export let visible: boolean = false;

    export interface Alert {
        message: string;
        details: string;
        code: string;
    }

    export interface AlertComponentMethods {
        showAlert: (alertData: Alert) => void;
        hideAlert: () => void;
    }

    export let data: Alert = {
        code: "UnknownError",
        details: "",
        message: ""
    }

    export const showAlert = (alertData: Alert): void => {
        data = alertData;
        visible = true;
    };

    export const hideAlert = (): void => {
        visible = false;
    };
</script>
{#if visible}
    <div class="w-full p-2">
        <div class="w-full flex gap-2 items-center p-2 rounded-lg border-2 { data.code !== 'Success' ? 'bg-red-700 border-red-900' :
                                                                             data.code === 'Success' ? 'bg-green-700 border-green-900' : ''}">


            <div>
                {#if data.code !== "Success"}
                    <BugReport size="28" />
                {:else}
                    <CheckBox size="28" />
                {/if}
            </div>

            <div class="flex flex-col w-full">
                <h4 class="font-bold text-sm">{ data.message ?? "Unknown alert title" }</h4>
                <p class="text-xs">{ data.details ?? "Unknown alert text" }</p>
            </div>
            <div class="flex flex-col">
                <button type="button"
                        class="text-white
                            { data.code === 'Success' ? 'bg-green-800 hover:bg-green-600 active:bg-green-900' : 'bg-red-800 hover:bg-red-600 active:bg-red-900' }
                            font-medium rounded-full text-sm p-2"
                        on:click={hideAlert}>
                    <Close size="16" />
                </button>
            </div>

        </div>
    </div>
{/if}