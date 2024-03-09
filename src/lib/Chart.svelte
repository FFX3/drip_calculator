<script lang="ts">
    import Chart, { type ChartItem } from 'chart.js/auto';
    import { onDestroy, onMount } from 'svelte'
    import 'chartjs-adapter-moment';

    export let title: string
    export let chartId: string
    export let datasets: any[]
    export let monthlyTotalReturns: { ticker: string, data: { total_return: number, month: number }[] }[]

    Chart.defaults.color = 'white'

    let canvas: HTMLElement | null

    onMount(()=>{
        canvas = document.getElementById(chartId)
        console.log('bingo', monthlyTotalReturns[0])
    })

    onDestroy(()=>{
        chart?.destroy()
    })

    let chart: Chart | null
    $: chart = canvas ? (()=>{
        chart?.destroy()
        return new Chart(canvas as ChartItem, {
            data: { datasets: datasets },
            options: {
                plugins: {
                    legend: {
                        position: 'left',
                    },
                },
                scales: {
                    x: {
                        type: 'time',
                    },
                },
                parsing: false,
            }
        })
    })() : null

        
</script>

<h2 style="text-align: center;">{ title }</h2>

<div style="width: 100%;">
    <canvas id={chartId}></canvas>
</div>

<div style="width: 100%; display: flex; flex-direction: row; text-align: center;">
    <div>
        <div>Ticker</div>
        {#each monthlyTotalReturns as ticker}
            <div style="border: 1px white solid; padding: 4px;">
                {ticker.ticker.toUpperCase()}
            </div>
        {/each}
    </div>
    {#if monthlyTotalReturns && monthlyTotalReturns[0]}
        {#each Array(monthlyTotalReturns[0].data.length) as _, index (index)}
            <div>
                <div>{monthlyTotalReturns[0].data[index].month}</div>
                    {#each monthlyTotalReturns as ticker}
                        <div style="border: 1px white solid; padding: 4px;">
                            {ticker.data[index].total_return.toFixed(2)}
                        </div>
                    {/each}
            </div>
        {/each}
    {/if}
</div>

