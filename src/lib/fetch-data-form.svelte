<script lang="ts">
    type TickerConfiguaration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

    export let fetchData: (conf: TickerConfiguaration[], start: Date, end: Date, initialInvestment: number)=>void

    export let tickerConfigurations: TickerConfiguaration[]
    export let loading: boolean;


    let end = (new Date()).toISOString().split('T')[0]
    let start = (()=>{
        const defaultStart = new Date()
        defaultStart.setFullYear(defaultStart.getFullYear() - 1)
        return defaultStart
    })().toISOString().split('T')[0]

    let initialInvestment = 1000

    let tickersToBeFetched: { [key: string]: TickerConfiguaration } = {}

    function handleSubmit(event: SubmitEvent){
        event.preventDefault()
        fetchData(Object.values(tickersToBeFetched), new Date(Date.parse(start)), new Date(Date.parse(end)), initialInvestment)
    }

</script>


<form class="config-form" on:submit={handleSubmit}>
    <div>
        <label for="investment">Initial Investment: </label>
        <input bind:value={initialInvestment} type="number" name="investment">
    </div>
    <div>
        <label for="start">Start: </label>
        <input bind:value={start} max={end} type="date" name="start">
        <label for="end">End: </label>
        <input bind:value={end} min={start} type="date" name="start">
    </div>
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
    <input disabled={loading} type="submit" value="Get Data">
</form>

