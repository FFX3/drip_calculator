<script lang="ts">
    import loadingIndicatorGif from "$lib/assets/loading-indicator.gif"
    import 'chartjs-adapter-moment';
    import Chart from '$lib/Chart.svelte';
    import Exporter from '$lib/Exporter.svelte';
    import Overview from '$lib/Overview.svelte';
    import { buildDataset } from '$lib/get-data'
    import FetchDataForm from '$lib/fetch-data-form.svelte';
    import type { TickerConfiguration, WatchListConfiguration } from "$lib/ticker-configurations";
    import WatchListConfigurationForm from "$lib/watch-list-configuration-form.svelte";
    import CreateOrPickWatchList from "$lib/create-or-pick-watch-list.svelte";
    import { getContext } from "svelte";

    let state: 'chart' | 'export' | 'ticker_overview' = 'chart'
    let chartsOpen = true
    let overviewOpen = false
    let exportsOpen = false
    let chartState: 'no_drip' | 'drip' | 'drip_at_nav' = 'no_drip'

    let noDripDatasets: any[] = []
    let dripDatasets: any[] = []
    let dripAtNavDatasets: any[] = []

    let monthlyTotalReturnsNoDrip: { ticker: string, data: { total_return: number, month: number } }[] = []
    let monthlyTotalReturnsDrip: { ticker: string, data: { total_return: number, month: number } }[] = []
    let monthlyTotalReturnsDripAtNav: { ticker: string, data: { total_return: number, month: number } }[] = []
    let csvs: any[] = []
    let onlyWithDividends: any
    let loading = false
    let watchList: WatchListConfiguration = {}
    let watchListName: string

    async function fetchData(configs: TickerConfiguration[], start: Date, end: Date, initialInvestment: number){
        loading = true

        noDripDatasets = []
        dripDatasets = []
        dripAtNavDatasets = []
        onlyWithDividends = {}
        monthlyTotalReturnsNoDrip = []
        monthlyTotalReturnsDrip = []
        monthlyTotalReturnsDripAtNav = []
        
        try {

            const promises = configs.map((config)=>{
                return buildDataset(config.ticker, config.mic, start, end, initialInvestment, config.color, config.dripAtNav)
            })
            const results = await Promise.all(promises)
            //@ts-ignore
            const { drip, dripAtNav, noDrip, csvs: _csvs, onlyWithDividends: _onlyWithDividends, monthly_total_return } = results.reduce((carry, { drip, dripAtNav, noDrip, csv, ticker, onlyWithDividends, monthly_total_return })=>{

                const mtrND = [] 
                const mtrD = [] 
                const mtrDAN = [] 


                monthly_total_return.forEach((entry)=>{
                    mtrND.push({
                        month: entry.month,
                        total_return: entry.no_drip
                    })
                    mtrD.push({
                        month: entry.month,
                        total_return: entry.drip
                    })
                    mtrDAN.push({
                        month: entry.month,
                        total_return: entry.drip_at_nav ?? entry.drip
                    })
                })

                monthlyTotalReturnsNoDrip.push({
                    ticker,
                    data: mtrND
                })
                monthlyTotalReturnsDrip.push({
                    ticker,
                    data: mtrD
                })
                monthlyTotalReturnsDripAtNav.push({
                    ticker,
                    data: mtrDAN
                })
                
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

            dripDatasets = drip
            dripAtNavDatasets = dripAtNav
            noDripDatasets = noDrip
            csvs = _csvs
            onlyWithDividends = _onlyWithDividends
        } catch(e) {
            console.error("DATA FETCH FAILED", e)
            alert("DATA FETCH FAILED")
        } finally {
            loading = false
        }
    }
</script>

<div class="container" style="display: flex; justify-content: space-between; flex-direction: row; align-items: start;">
    <div style="display: flex; gap: 10px; flex-direction: column;">
        <CreateOrPickWatchList
            bind:watchListName={watchListName}
            bind:watchList={watchList}
        />
        <FetchDataForm 
            bind:watchList={watchList}
            loading={loading}
            fetchData={fetchData}
        />
        <div style="display: flex; flex-direction: row; gap: 20px;">
            <div>
                <label for="chart">All:</label>
                <input checked type="checkbox" name="chart" on:change={(e)=>{
                    chartsOpen = e.target.checked
                    exportsOpen = e.target.checked
                    overviewOpen = e.target.checked
                }}>
            </div>
            <div>
                <label for="chart">Chart:</label>
                <input type="checkbox" name="chart" bind:checked={chartsOpen}>
            </div>
            <div>
                <label for="export">Export:</label>
                <input type="checkbox" name="export" bind:checked={exportsOpen}>
            </div>
            <div>
                <label for="overview">Overview:</label>
                <input type="checkbox" name="overview" bind:checked={overviewOpen}>
            </div>
        </div>
    </div>
    <div>
        <WatchListConfigurationForm
            watchListName={watchListName} 
            bind:watchList={watchList} 
        />
    </div>
</div>


{#if loading }
    <div class="container" style="display: flex; justify-content: center; align-items: center;">
        <img src={loadingIndicatorGif} />
    </div>
{:else}
    {#if chartsOpen}
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
                        bind:monthlyTotalReturns={monthlyTotalReturnsNoDrip}
                    />
                {/if}
                {#if 'drip' == chartState}
                    <Chart 
                        title='Total Returns with DRIP at market price'
                        chartId='drip'
                        bind:datasets={dripDatasets}
                        bind:monthlyTotalReturns={monthlyTotalReturnsDrip}
                    />
                {/if}
                {#if 'drip_at_nav' == chartState}
                    <Chart 
                        title='Total Returns with DRIP at NAV price'
                        chartId='drip_at_nav'
                        datasets={dripAtNavDatasets}
                        bind:monthlyTotalReturns={monthlyTotalReturnsDripAtNav}
                    />
                {/if}
            </div>
        </div>
    {/if}

    {#if overviewOpen}
        <Overview 
            data={onlyWithDividends} 
        />
    {/if}

    {#if exportsOpen}
        <Exporter csvs={csvs} />
    {/if}
{/if}
