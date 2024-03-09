<script lang="ts">
    import { type WatchListConfiguration } from "$lib/ticker-configurations"
    type TickerConfiguration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

    export let fetchData: (conf: TickerConfiguration[], start: Date, end: Date, initialInvestment: number)=>void

    export let watchList: WatchListConfiguration = {}
    export let loading: boolean;
    let tickerConfigurations: TickerConfiguration[] 
    $: tickerConfigurations = Object.values(watchList);


    let end = (new Date()).toISOString().split('T')[0]
    let start = (()=>{
        const defaultStart = new Date()
        defaultStart.setFullYear(defaultStart.getFullYear() - 1)
        return  defaultStart
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
    <div style="display: flex; flex-direction: row; align-items: center;">
        <label for="start">Start: </label>
        <input bind:value={start} max={end} type="date" name="start">
        <label for="end">End: </label>
        <input bind:value={end} min={start} type="date" name="start">
        <input disabled={loading} type="submit" value="Get Data">
    </div>
    <div>
        <button disabled={loading}  on:click={()=>{
            end = (new Date()).toISOString().split('T')[0]
            start = (()=>{
                const defaultStart = new Date()
                defaultStart.setMonth(defaultStart.getMonth() - 1)
                return  defaultStart
            })().toISOString().split('T')[0]
        }}>1 M</button>
        <button disabled={loading}  on:click={()=>{
            end = (new Date()).toISOString().split('T')[0]
            start = (()=>{
                const defaultStart = new Date()
                defaultStart.setMonth(defaultStart.getMonth() - 3)
                return  defaultStart
            })().toISOString().split('T')[0]
        }}>3 M</button>
        <button disabled={loading}  on:click={()=>{
            end = (new Date()).toISOString().split('T')[0]
            start = (()=>{
                const defaultStart = new Date()
                defaultStart.setMonth(defaultStart.getMonth() - 5)
                return  defaultStart
            })().toISOString().split('T')[0]
        }}>5 M</button>
        <button disabled={loading}  on:click={()=>{
            end = (new Date()).toISOString().split('T')[0]
            start = (()=>{
                const defaultStart = new Date()
                defaultStart.setMonth(0)
                defaultStart.setDate(0)
                return  defaultStart
            })().toISOString().split('T')[0]
        }}>YTD</button>
        <button disabled={loading}  on:click={()=>{
            end = (new Date()).toISOString().split('T')[0]
            start = (()=>{
                const defaultStart = new Date()
                defaultStart.setFullYear(defaultStart.getFullYear() - 1)
                return  defaultStart
            })().toISOString().split('T')[0]
        }}>1 Y</button>
        <button disabled={loading}  on:click={()=>{
            end = (new Date()).toISOString().split('T')[0]
            start = (()=>{
                const defaultStart = new Date()
                defaultStart.setFullYear(defaultStart.getFullYear() - 5)
                return  defaultStart
            })().toISOString().split('T')[0]
        }}>5 Y</button>
    </div>
    <div style="display: flex; gap: 20px; padding-bottom: 20px;">
        <div>
            <label for="all_tickers">All</label>
            <input checked on:change={(e)=>{
                Object.keys(tickersToBeFetched).map((ticker)=>{
                    tickersToBeFetched[ticker] = e.target.checked
                })
                console.log(tickersToBeFetched)
            }} type="checkbox" name="all_tickers">
        </div>
        {#each tickerConfigurations as config}
            <div>
                <label for={config.ticker}>{config.ticker.toUpperCase()}</label>
                <input bind:checked={tickersToBeFetched[config.ticker]} type="checkbox" name={config.ticker}>
            </div>
        {/each}
    </div>
</form>

