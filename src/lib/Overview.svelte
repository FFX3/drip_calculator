<script lang="ts">
    
    export let data: {
        ticker: string,
        data: {
            no_drip: any
            drip: any,
            drip_at_nav?: any | null
        }[]
    }[] = []

    let state: 'drip' | 'no_drip' | 'drip_at_nav' = 'no_drip'

    $: _data = data.map((ticker)=>{
        console.log(ticker.data)
        return {
            ticker: ticker.ticker,
            data: ticker.data.map(entry=>entry[state])
        }
    }).filter(({ data })=>{
        return null != data[0]
    })


    let lastXMonths: number = 3
    

    // Room for improvement
    $: filteredData = _data.map((ticker)=>{
        const { data } = ticker
        const [year, month, day] = data[data.length-1].date.split('-')

        const endDate = new Date()
        endDate.setFullYear(year)
        endDate.setMonth(month)
        endDate.setDate(day)
        let xMonthsEarlier = new Date(endDate)
        xMonthsEarlier.setMonth(endDate.getMonth() - lastXMonths + 1) // +1 because last 3 months intuitively means last 3 dividends for shares that payout monthly

        return {
            ticker: ticker.ticker,
            data: ticker.data.filter((entry)=>{
                const [year, month, day] = entry.date.split('-')
                const date = new Date()
                date.setFullYear(year)
                date.setMonth(month)
                date.setDate(day)
                return date > xMonthsEarlier
            }).toReversed()
        }
    })

    $: estimatedYearlyIncome = filteredData.map((ticker)=>{
        return (ticker.data.reduce((carry, entry)=>{
            return carry + entry.dividend
        },0) * 12 / lastXMonths).toFixed(2)
    })

    $: geometrictReturns = !!_data ? _data.map(({ data })=>{
        const { total_return } = data[data.length-1]
        let start = new Date()
        let end = new Date()
        {
            const [year, month, day] = data[data.length-1].date.split('-')
            end.setFullYear(year)
            end.setMonth(month)
            end.setDate(day)
        }
        {
            const [year, month, day] = data[0].date.split('-')
            start.setFullYear(year)
            start.setMonth(month)
            start.setDate(day)
        }    
        const  millisecondsPerDay = 24 * 60 * 60 * 1000;
        const difference = ((end - start) / millisecondsPerDay)
        return ((((1 + total_return)**(365/(difference))) - 1) * 100).toFixed(2)
    }) : []

</script>
<style>
    table, th, td {
        border: 1px solid white;
        border-collapse: collapse;
    }
    td, th {
        padding: 10px;
    }
</style>

<div class="container">
    <label for="lastXMonths">Last X Months (for income estimate)</label>
    <br>
    <input name="lastXMonths" bind:value={lastXMonths}>

    <div style="display: flex; flex-direction: row; gap: 20px;">
        <div>
            <label for="no_drip">No drip:</label>
            <input type="radio" name="no_drip" value="no_drip" bind:group={state}>
        </div>
        <div>
            <label for="drip">Drip:</label>
            <input type="radio" name="drip" value="drip" bind:group={state}>
        </div>
        <div>
            <label for="drip_at_nav">Drip at nav:</label>
            <input type="radio" name="drip_at_nav" value="drip_at_nav" bind:group={state}>
        </div>
    </div>
</div>

<div style="display: flex; gap: 50px;">
{#if _data}
    {#each _data as ticker, i }
        <div class="container">
            <h2>{ticker.ticker.toUpperCase()}</h2>

            <p>Yearly estimated income: ${estimatedYearlyIncome[i]}</p>

            <p>Yearly geometric average return: {geometrictReturns[i]}%</p>

            <table style="border-spacing: 10px; width: 400px;">
                <thead>
                    <tr>
                        <th>Date</th>
                        <th>Dividend</th>
                        <th>Position Value</th>
                        <th>Total Return</th>
                    </tr>
                </thead>
                <tbody>
                    {#each ticker.data.toReversed() as entry }
                        <tr>
                            <td>{entry.date}</td>
                            <td>{entry.dividend.toFixed(2)}</td>
                            <td>{entry.position_value.toFixed(2)}</td>
                            <td>{(entry.total_return * 100).toFixed(2)}%</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/each}
{/if}
</div>
