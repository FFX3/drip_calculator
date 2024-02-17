<script lang="ts">
    import Chart, { type ChartItem } from 'chart.js/auto';
    import { onDestroy, onMount } from 'svelte'
    import 'chartjs-adapter-moment';

    export let chartId: string
    export let datasets: any[]

    Chart.defaults.color = 'white'

    let canvas: HTMLElement | null

    onMount(()=>{
        canvas = document.getElementById(chartId)
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

<div style="width: 800px;"><canvas id={chartId}></canvas></div>
