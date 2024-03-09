<script lang="ts">
    import 'chartjs-adapter-moment';
    import TickerConfigForm from '$lib/ticker-config-form.svelte';
    import type { TickerConfiguration, WatchListConfiguration } from '$lib/ticker-configurations';
    import { deleteTicker, configureTicker } from '$lib/ticker-configurations';

    export let watchListName: string
    export let watchList: WatchListConfiguration = {}

    let configuredTickersArray: TickerConfiguration[] 
    $: configuredTickersArray = Object.values(watchList)

    async function _configureTicker(config: TickerConfiguration){
        watchList = await configureTicker(watchListName, config)
    }

    async function _deleteTicker(ticker: string){
        watchList = await deleteTicker(watchListName, ticker)
    }

</script>

<div style="width: max-content; background-color: #535C91; display: flex; flex-direction: row; gap: 10px;">
    <TickerConfigForm 
        clearOnSubmit={true}
        submitButtonText='Add'
        addTicker={_configureTicker}
    />
</div>
{#each configuredTickersArray as config }
    <div style="width: max-content; background-color: #535C91; display: flex; flex-direction: row; gap: 10px;">
        <TickerConfigForm 
            {...config}
            addTicker={_configureTicker}
        />
        <button on:click={()=>{
            _deleteTicker(config.ticker)
        }}>Delete</button>
    </div>
{/each}
