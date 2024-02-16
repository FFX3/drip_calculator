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

<a href="/">comparision</a>

<div style="width: 700px; display: flex; flex-wrap: wrap;  gap: 40px; padding: 40px;">
    <TickerConfigForm 
        clearOnSubmit={true}
        submitButtonText='Add'
        addTicker={configureTicker}
    />
</div>

<div style="width: 700px; display: flex; flex-wrap: wrap;  gap: 40px; padding: 40px; max-height: 600px; overflow-y: scroll;">
    {#each configuredTickersArray as config }
        <div>
            <TickerConfigForm 
                {...config}
                addTicker={configureTicker}
            />
            <button style="margin-top: 14px;" on:click={()=>{
                deleteTicker(config.ticker)
            }}>Delete</button>
        </div>
    {/each}

</div>
