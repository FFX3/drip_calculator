<script lang="ts">
    import 'chartjs-adapter-moment';
    import Chart from '$lib/Chart.svelte';
    import TickerConfigForm from '$lib/ticker-config-form.svelte';
    import { buildDataset } from '$lib/get-data'
    import FetchDataForm from '$lib/fetch-data-form.svelte';


    type TickerConfiguaration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

    let configuredTickers: {
        [key: string]: TickerConfiguaration
    } = {
        qqq: {
            ticker: 'qqq',
            mic: 'xnas',
            color: 'magenta',
            dripAtNav: false,
        },

        clm: {
            ticker: 'clm',
            mic: 'xase',
            color: 'blue',
            dripAtNav: true,
        },

        crf: {
            ticker: 'crf',
            mic: 'xase',
            color: 'red',
            dripAtNav: true,
        },
    }

    let configuredTickersArray: TickerConfiguaration[] 
    $: configuredTickersArray = Object.values(configuredTickers)

    function configureTicker(config: TickerConfiguaration){
        configuredTickers[config.ticker] = config
    }

    const today = new Date()
    const onYearAgo = new Date(today)
    onYearAgo.setFullYear(today.getFullYear() - 1)

    let noDripDatasets: any[] = []
    let dripDatasets: any[] = []
    let dripAtNavDatasets: any[] = []
    let chartLoading = false

    async function fetchData(configs: TickerConfiguaration[]){
        chartLoading = true

        noDripDatasets = []
        dripDatasets = []
        dripAtNavDatasets = []

        const promises = configs.map((config)=>{
            return buildDataset(config.ticker, config.mic, onYearAgo, today, 1000, config.color, config.dripAtNav)
        })
        const results = await Promise.all(promises)
        //@ts-ignore
        const { drip, dripAtNav, noDrip } = results.reduce((carry, { drip, dripAtNav, noDrip })=>{
            carry.drip.push(drip)
            carry.noDrip.push(noDrip)

            if(dripAtNav){
                carry.dripAtNav.push(dripAtNav)
            }

            return carry
        },{
            drip: [],
            dripAtNav: [],
            noDrip: [],
        })

        chartLoading = false
        dripDatasets = drip
        dripAtNavDatasets = dripAtNav
        noDripDatasets = noDrip
    }
</script>

<div style="width: 700px; display: flex; flex-wrap: wrap;  gap: 40px; padding: 40px;">
    {#each configuredTickersArray as config }
        <TickerConfigForm 
            {...config}
            addTicker={configureTicker}
        />
    {/each}

    <TickerConfigForm 
        clearOnSubmit={true}
        submitButtonText='Add'
        addTicker={configureTicker}
    />
</div>

<FetchDataForm 
    fetchData={fetchData}
    tickerConfigurations={configuredTickersArray}
/>
{#if !chartLoading}
    {#if 0 != noDripDatasets.length}
        <Chart 
            chartId='no_drip'
            datasets={noDripDatasets}
        />
    {/if}

    {#if 0 != dripDatasets.length}
        <Chart 
            chartId='drip'
            bind:datasets={dripDatasets}
        />
    {/if}

    {#if 0 != dripAtNavDatasets.length}
        <Chart 
            chartId='drip_at_nav'
            datasets={dripAtNavDatasets}
        />
    {/if}
{/if}
