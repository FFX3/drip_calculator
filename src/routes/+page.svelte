<script lang="ts">
    import 'chartjs-adapter-moment';
    import Chart from '$lib/Chart.svelte';
    import Exporter from '$lib/Exporter.svelte';
    import Overview from '$lib/Overview.svelte';
    import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
    import { buildDataset } from '$lib/get-data'
    import FetchDataForm from '$lib/fetch-data-form.svelte';
    
    type TickerConfiguaration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

    let state: 'chart' | 'export' | 'ticker_overview' = 'chart'

    let configuredTickers: {
        [key: string]: TickerConfiguaration
    } = {} 

    readTextFile('tickers.conf', { dir: BaseDirectory.AppConfig }).then(contents=>{
        if("" == contents){
            configuredTickers = {}
        } else {
            configuredTickers = JSON.parse(contents)
        }
    })

    let configuredTickersArray: TickerConfiguaration[] 
    $: configuredTickersArray = Object.values(configuredTickers)

    let noDripDatasets: any[] = []
    let dripDatasets: any[] = []
    let dripAtNavDatasets: any[] = []
    let csvs: any[] = []
    let onlyWithDividends: any
    let loading = false

    async function fetchData(configs: TickerConfiguaration[], start: Date, end: Date, initialInvestment: number){
        loading = true

        noDripDatasets = []
        dripDatasets = []
        dripAtNavDatasets = []
        onlyWithDividends = {}

        const promises = configs.map((config)=>{
            return buildDataset(config.ticker, config.mic, start, end, initialInvestment, config.color, config.dripAtNav)
        })
        const results = await Promise.all(promises)
        //@ts-ignore
        const { drip, dripAtNav, noDrip, csvs: _csvs, onlyWithDividends: _onlyWithDividends } = results.reduce((carry, { drip, dripAtNav, noDrip, csv, ticker, onlyWithDividends })=>{

            console.log(ticker, csv)
            carry.drip.push(drip)
            carry.noDrip.push(noDrip)
            carry.csvs.push({
                ticker,
                csv
            })
            carry.onlyWithDividends.push({
                ticker,
                data: onlyWithDividends
            })

            if(dripAtNav){
                carry.dripAtNav.push(dripAtNav)
            }

            return carry
        },{
            drip: [],
            dripAtNav: [],
            noDrip: [],
            csvs: [],
            onlyWithDividends: [],
        })

        loading = false
        dripDatasets = drip
        dripAtNavDatasets = dripAtNav
        noDripDatasets = noDrip
        csvs = _csvs
        onlyWithDividends = _onlyWithDividends
    }
</script>

<div style="margin-bottom: 20px;" class="container">
    <a href="/configuration">configuration</a>
</div>

<div class="container">
    <FetchDataForm 
        loading={loading}
        fetchData={fetchData}
        tickerConfigurations={configuredTickersArray}
    />
</div>

<div style="display: flex; flex-direction: row; gap: 20px; margin-top: 40px;" class="container">
    <div>
        <label for="chart">Chart:</label>
        <input type="radio" name="chart" value="chart" bind:group={state}>
    </div>
    <div>
        <label for="export">Export:</label>
        <input type="radio" name="export" value="export" bind:group={state}>
    </div>
    <div>
        <label for="ticker_overview">Ticker Overview:</label>
        <input type="radio" name="ticker_overview" value="ticker_overview" bind:group={state}>
    </div>
</div>

{#if !loading }
    {#if state == 'chart'}
        {#if 0 != noDripDatasets.length}
            <div class="container">
                <h2>No drip</h2>
                <Chart 
                    chartId='no_drip'
                    datasets={noDripDatasets}
                />
            </div>
        {/if}

        {#if 0 != dripDatasets.length}
            <div class="container">
                <h2>Drip</h2>
                <Chart 
                    chartId='drip'
                    bind:datasets={dripDatasets}
                />
            </div>
        {/if}

        {#if 0 != dripAtNavDatasets.length}
            <div class="container">
                <h2>Drip at nav</h2>
                <Chart 
                    chartId='drip_at_nav'
                    datasets={dripAtNavDatasets}
                />
            </div>
        {/if}
    {/if}

    {#if state == 'export'}
        <Exporter csvs={csvs} />
    {/if}

    {#if state == 'ticker_overview'}
        <Overview 
            data={onlyWithDividends} 
        />
    {/if}
{/if}
