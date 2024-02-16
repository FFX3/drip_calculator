<script lang="ts">
    type TickerConfiguaration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

    export let fetchData: (conf: TickerConfiguaration[])=>void

    export let tickerConfigurations: TickerConfiguaration[]

    let tickersToBeFetched: { [key: string]: TickerConfiguaration } = {}

    function handleSubmit(event: SubmitEvent){
        event.preventDefault()
        fetchData(Object.values(tickersToBeFetched))
    }

</script>


<form class="config-form" on:submit={handleSubmit}>
    <div style="display: flex; gap: 20px; padding-bottom: 20px;">
        {#each tickerConfigurations as config}
            <div>
                <label for={config.ticker}>{config.ticker.toUpperCase()}</label>
                <input type="checkbox" name={config.ticker} on:change={(event)=>{
                    //@ts-ignore
                    if(event.target.checked){
                        tickersToBeFetched[config.ticker] = config
                    } else {
                        delete tickersToBeFetched[config.ticker]
                    }
                }}>
            </div>
        {/each}
    </div>
    <input type="submit" value="Get Data">
</form>

