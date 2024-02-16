<script lang="ts">
    type TickerConfiguaration = {
        ticker: string,
        mic: string,
        dripAtNav: boolean,
        color: string
    }

    export let addTicker: (conf: TickerConfiguaration)=>void

    export let ticker: string = ""
    export let mic: string = ""
    export let dripAtNav: boolean = false
    export let color: string = ""

    export let submitButtonText: string = "Save"
    export let clearOnSubmit: boolean = false

    function addTickerConfiguration(event: SubmitEvent){
        event.preventDefault()
        addTicker({
            ticker,
            mic,
            dripAtNav,
            color
        })
        if(clearOnSubmit){
            ticker = ''
            mic = ''
            color = ''
            dripAtNav = false
        }
    }

</script>

<style>
    .config-form {
        width: 200px; 
        display: flex; 
        flex-direction: column; 
        align-items: center;
    }

    .config-form__input-container{
        width: 100%;
        display: flex;
        justify-content: space-between;
        margin-bottom: 12px;
    }
    .config-form__input-container > input{
        width: 70px;
    }
</style>

<form class="config-form" on:submit={addTickerConfiguration}>
    <div class="config-form__input-container">
        <label for="ticker">Ticker:</label>
        <input required bind:value={ticker} name="ticker"/>
    </div>
    <div class="config-form__input-container">
        <label for="mic">MIC code:</label>
        <input required bind:value={mic} name="mic" />
    </div>
    <div class="config-form__input-container">
        <label for="drip_at_nav">Drip at nav:</label>
        <input bind:checked={dripAtNav} name="drip_at_nav"  type="checkbox"/>
    </div>
    <div class="config-form__input-container">
        <label for="color">Color:</label>
        <div style="width: 70px; padding: 0px;">
            <input required style="width: 100%;" bind:value={color} name="color" />
            <input required bind:value={color} name="color_picker" type="color" />
        </div>
    </div>
    <input type="submit" value={submitButtonText}>
</form>

