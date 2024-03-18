<script lang="ts">
    import { writeText } from '@tauri-apps/api/clipboard';
    import { save } from '@tauri-apps/api/dialog';
    import { writeTextFile } from '@tauri-apps/api/fs';
    import { SettingsManager } from 'tauri-settings';

    type Schema = {
        defaultExportPath: string
    }

    const settingsManager = new SettingsManager<Schema>(
        { // defaults
            defaultExportPath: "bingo"
        },
        { // options
            fileName: 'user-settings'
        }
    )

    let defaultExportPath = ''
    settingsManager.initialize().then(async()=>{
        await settingsManager.syncCache()
        defaultExportPath = settingsManager.getCache('defaultExportPath')
    })


    export let csvs: {ticker: string, csv: string}[] = []

</script>

<div class="container">
    <div>
        <label>default export path</label>
        <input bind:value={defaultExportPath} />
    </div>
    <div style="display: flex; gap: 20px; flex-wrap: wrap;">
        

        {#each csvs as csv}
            <div class="container">
                <h3>{csv.ticker.toUpperCase()}</h3>
                <button on:click={async ()=>{
                    await writeText(csv.csv)
                    alert(csv.ticker + " csv copied to clipboard")
                }}>copy</button>
                <button on:click={async ()=>{
                    settingsManager.setCache("defaultExportPath", defaultExportPath)
                    await settingsManager.syncCache()
                    const filePath = await save({
                        defaultPath: `${defaultExportPath}\\${csv.ticker}-data.csv`,
                        filters: [{
                            name: 'csv',
                            extensions: ['csv'],
                        }]
                    });
                    if(filePath){
                        await writeTextFile(filePath, csv.csv);
                    }

                }}>export</button>
            </div>
        {/each}
    </div>
</div>
