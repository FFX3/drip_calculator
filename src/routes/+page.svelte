<script lang="ts">
    import loadingIndicatorGif from "$lib/assets/loading-indicator.gif"
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
    let chartState: 'no_drip' | 'drip' | 'drip_at_nav' = 'no_drip'

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

            carry.dripAtNav.push(dripAtNav ?? drip)

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

<div style="margin-bottom: 20px;">
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

{#if loading }
    <div class="container" style="display: flex; justify-content: center; align-items: center;">
        <img src={loadingIndicatorGif} />
    </div>
{:else}
    {#if state == 'chart'}
        <div class="container">
            <div style="display: flex; flex-direction: row;">
                <div style={`padding: 10px; border: 1px white solid; ${chartState == 'no_drip' ? 'background-color: white; color: black;' : ''}`} on:click={()=>chartState = 'no_drip'}>no DRIP</div>
                <div style={`padding: 10px; border: 1px white solid; ${chartState == 'drip' ? 'background-color: white; color: black;' : ''}`} on:click={()=>chartState = 'drip'}>DRIP</div>
                <div style={`padding: 10px; border: 1px white solid; ${chartState == 'drip_at_nav' ? 'background-color: white; color: black;' : ''}`} on:click={()=>chartState = 'drip_at_nav'}>DRIP at NAV</div>
            </div>
            <div style="width: 100%;">
                {#if 'no_drip' == chartState}
                    <Chart 
                        title='Total Returns with no DRIP'
                        chartId='no_drip'
                        datasets={noDripDatasets}
                    />
                {/if}
                {#if 'drip' == chartState}
                    <Chart 
                        title='Total Returns with DRIP at market price'
                        chartId='drip'
                        bind:datasets={dripDatasets}
                    />
                {/if}
                {#if 'drip_at_nav' == chartState}
                    <Chart 
                        title='Total Returns with DRIP at NAV price'
                        chartId='drip_at_nav'
                        datasets={dripAtNavDatasets}
                    />
                {/if}
            </div>
        </div>
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
