<script lang="ts">
    import 'chartjs-adapter-moment';
    import TickerConfigForm from '$lib/ticker-config-form.svelte';
    import type { TickerConfiguration, WatchListConfiguration } from '$lib/ticker-configurations';
    import { getWatchListConfigByName, getWatchListNames, deleteTicker, configureTicker, createWatchListConfiguration, deleteWatchListConfiguration  } from '$lib/ticker-configurations';
    import WatchListConfigurationForm from '$lib/watch-list-configuration-form.svelte';
    

    let watchListName: string = 'default'
    let watchList: WatchListConfiguration = {}
    let watchListNameFormInput: string = ''
    let watchListNames: Promise<string[]> 

    function _getWatchListNames(){
        watchListNames = getWatchListNames()
    }
    _getWatchListNames()

    async function getWatchList() {
        await getWatchListConfigByName(watchListName).then(contents=>{
            watchList = contents
        })
    }
    getWatchList()

    let configuredTickersArray: TickerConfiguration[] 
    $: configuredTickersArray = Object.values(watchList)

    async function _configureTicker(config: TickerConfiguration){
        watchList = await configureTicker(watchListName, config)
    }

    async function _deleteTicker(ticker: string){
        watchList = await deleteTicker(watchListName, ticker)
    }

    async function createWatchListFormHandler(e: SubmitEvent) {
        e.preventDefault()
        createWatchListConfiguration(watchListNameFormInput)
        watchListName = watchListNameFormInput
        watchListNameFormInput = ''
        watchList = {}
        _getWatchListNames()
    }

</script>

<div style="margin-bottom: 20px;" class="container">
    <a href="/">comparision</a>
</div>

<div class="container">
    <form class="config-form" on:submit={createWatchListFormHandler}>
        <div class="config-form__input-container">
            <label for="ticker">Watch List Name:</label>
            <input required bind:value={watchListNameFormInput} name="ticker"/>
        </div>
    </form>
    <button on:click={async ()=>{
        if(confirm(`Are you sure you want to delete ${watchListName}`)){
            await deleteWatchListConfiguration(watchListName)
            _getWatchListNames()
        }
    }}>
        delete
    </button>
    {#await watchListNames then names}
        <select
            bind:value={watchListName}
            on:change={() => {
                getWatchList()
            }}
        >
            {#each names as name}
                <option value={name}>
                    {name}
                </option>
            {/each}
        </select>
    {/await}
</div>

<div class="container">
    <WatchListConfigurationForm watchListName={watchListName} watchList={watchList} />
</div>

