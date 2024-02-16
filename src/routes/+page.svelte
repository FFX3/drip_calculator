<script lang="ts">
    import Chart, { type ChartItem } from 'chart.js/auto';
    import 'chartjs-adapter-moment';
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte'

    const today = new Date()
    const onYearAgo = new Date(today)
    onYearAgo.setFullYear(today.getFullYear() - 4)

    function formatDateToYYYYMMDD(date: Date) {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0'); // Adding 1 to month as it's zero-based
        const day = String(date.getDate()).padStart(2, '0');

        return `${year}-${month}-${day}`;
    }


    let noDripDatasets: any[] = []
    let chartNoDrip: any = null

    let dripDatasets: any[] = []
    let chartDrip: any = null

    let dripAtNavDatasets: any[] = []
    let chartAtNavDrip: any = null

    onMount(()=>{
        const noDripCanvas = document.getElementById("no_drip")
        const dripCanvas = document.getElementById("drip")
        const dripAtNavCanvas = document.getElementById("drip_at_nav")

        invoke('fetch_data', {
            ticker: "clm",
            mic: "xase",
            startDate: formatDateToYYYYMMDD(onYearAgo),
            endDate: formatDateToYYYYMMDD(today)
        }).then((_data: any)=>{
            console.log(_data)
            const { no_drip, drip, drip_at_nav } = _data
            {
                noDripDatasets = [...noDripDatasets, {
                    label: 'clm'.toUpperCase(),
                    type: 'line',
                    data: no_drip,
                }]

                console.log("canvas", noDripCanvas)
                chartNoDrip = new Chart(noDripCanvas as ChartItem, {
                    data: { datasets: noDripDatasets },
                    options: {
                        scales: {
                            x: {
                                type: 'timeseries',
                            },
                        },
                        parsing: false
                    }
                })
            }
            {
                dripDatasets = [...dripDatasets, {
                    label: 'clm'.toUpperCase(),
                    type: 'line',
                    data: drip,
                }]

                chartDrip = new Chart(dripCanvas as ChartItem, {
                    data: { datasets: dripDatasets },
                    options: {
                        scales: {
                            x: {
                                type: 'timeseries',
                            },
                        },
                        parsing: false
                    }
                })
            }
            {
                dripAtNavDatasets = [...dripAtNavDatasets, {
                    label: 'clm'.toUpperCase(),
                    type: 'line',
                    data: drip_at_nav,
                }]

                chartDripAtNav = new Chart(dripAtNavCanvas as ChartItem, {
                    data: { datasets: dripAtNavDatasets },
                    options: {
                        scales: {
                            x: {
                                type: 'timeseries',
                            },
                        },
                        parsing: false
                    }
                })
            }
        })
    })

</script>

<div style="width: 800px;"><canvas id="no_drip"></canvas></div>
<div style="width: 800px;"><canvas id="drip"></canvas></div>
<div style="width: 800px;"><canvas id="drip_at_nav"></canvas></div>
