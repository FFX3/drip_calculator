<script lang="ts">
    type TickerConfiguration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

    export let fetchData: (conf: TickerConfiguration[], start: Date, end: Date, initialInvestment: number)=>void

    export let tickerConfigurations: TickerConfiguration[]
    export let loading: boolean;


    let end = (new Date()).toISOString().split('T')[0]
    let start = (()=>{
        const defaultStart = new Date()
        defaultStart.setFullYear(defaultStart.getFullYear() - 1)
        return defaultStart
    })().toISOString().split('T')[0]

    let initialInvestment = 1000

    let tickersToBeFetched: { [key: string]: boolean } = {}
    $: defaultTickersToBeFetched = tickerConfigurations.reduce((carry, config)=>{
        carry[config.ticker] = true
        return carry
    }, {} as { [key: string]: boolean })
        console.log(tickersToBeFetched, tickerConfigurations)
    $: tickersToBeFetched = { ...defaultTickersToBeFetched, ...tickersToBeFetched }

    function handleSubmit(event: SubmitEvent){
        event.preventDefault()
        console.log(tickersToBeFetched, tickerConfigurations)
        const tickerConfigs: TickerConfiguration[] = Object.keys(tickersToBeFetched).reduce((carry, ticker)=>{
            const config = tickerConfigurations.find(config=>config.ticker === ticker)
            if(config && tickersToBeFetched[ticker]) {
                carry.push(config)
            }
            return carry
        },[] as TickerConfiguration[])
        fetchData(tickerConfigs, new Date(Date.parse(start)), new Date(Date.parse(end)), initialInvestment)
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
                <input bind:checked={tickersToBeFetched[config.ticker]} type="checkbox" name={config.ticker}>
            </div>
        {/each}
    </div>
    <input disabled={loading} type="submit" value="Get Data">
</form>

