<script lang="ts">
    import 'chartjs-adapter-moment';
    import type { TickerConfiguration, WatchListConfiguration } from '$lib/ticker-configurations';
    import { getWatchListConfigByName, getWatchListNames, createWatchListConfiguration, deleteWatchListConfiguration, } from '$lib/ticker-configurations';
    
    export let watchListName: string
    export let watchList: WatchListConfiguration = {}
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

    async function createWatchListFormHandler(e: SubmitEvent) {
        e.preventDefault()
        createWatchListConfiguration(watchListNameFormInput)
        watchListName = watchListNameFormInput
        watchListNameFormInput = ''
        watchList = {}
        _getWatchListNames()
    }

</script>

<form style="display: flex; flex-direction: row;" class="config-form" on:submit={createWatchListFormHandler}>
    <div class="config-form__input-container">
        <label for="ticker">New List Name:</label>
        <input required bind:value={watchListNameFormInput} name="ticker"/>
        <input value="create" type="submit">
    </div>
</form>

<button on:click={async ()=>{
    if(confirm(`Are you sure you want to delete ${watchListName}`)){
        await deleteWatchListConfiguration(watchListName)
        _getWatchListNames()
    }
}}>
    delete current watchlist
</button>

{#await watchListNames then names}
    <select
        style="text-align-last: center;"
        bind:value={watchListName}
        on:change={() => {
            getWatchList()
        }}
    >
        <option value="" selected disabled hidden>Choose watchlist</option>
        {#each names as name}
            <option value={name}>
                {name}
            </option>
        {/each}
    </select>
{/await}

