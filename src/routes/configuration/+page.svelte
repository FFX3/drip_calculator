<script lang="ts">
    import 'chartjs-adapter-moment';
    import { writeTextFile, readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
    import TickerConfigForm from '$lib/ticker-config-form.svelte';
    
    type TickerConfiguaration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

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

    function configureTicker(config: TickerConfiguaration){
        configuredTickers[config.ticker] = config
        saveTickerConfiguration()
    }

    function deleteTicker(ticker: string){
        delete configuredTickers[ticker]
        saveTickerConfiguration()
    }

    function saveTickerConfiguration(){
        configuredTickers = {...configuredTickers}
        writeTextFile({ path: 'tickers.conf', contents: JSON.stringify(configuredTickers) }, { dir: BaseDirectory.AppConfig });
    }

</script>

<div style="margin-bottom: 20px;" class="container">
    <a href="/">comparision</a>
</div>

<div class="container">
    <h2>Add ticker</h2>
    <div style="width: 700px; display: flex; flex-wrap: wrap;  gap: 40px;">
            <div class="container" style="background-color: #535C91;">
            <TickerConfigForm 
                clearOnSubmit={true}
                submitButtonText='Add'
                addTicker={configureTicker}
            />
        </div>
    </div>
</div>

<div class="container">
    <h2>Edit tickers</h2>
    <div style="display: flex; flex-wrap: wrap;  gap: 40px; padding: 40px; max-height: 600px; overflow-y: scroll;">
        {#each configuredTickersArray as config }
            <div class="container" style="background-color: #535C91;">
                <h3>{config.ticker.toUpperCase()}</h3>
                <button style="margin-bottom: 14px;" on:click={()=>{
                    deleteTicker(config.ticker)
                }}>Delete</button>
                <TickerConfigForm 
                    {...config}
                    addTicker={configureTicker}
                />
            </div>
        {/each}

    </div>
</div>
