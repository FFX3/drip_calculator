<script lang="ts">
    import { writeText } from '@tauri-apps/api/clipboard';
    import { save } from '@tauri-apps/api/dialog';
    import { writeTextFile } from '@tauri-apps/api/fs';

    export let csvs: {ticker: string, csv: string}[] = []
</script>

<div style="display: flex; gap: 20px;">
    {#each csvs as csv}
        <div class="container">
            <h3>{csv.ticker.toUpperCase()}</h3>
            <button on:click={async ()=>{
                await writeText(csv.csv)
                alert(csv.ticker + " csv copied to clipboard")
            }}>copy</button>
            <button on:click={async ()=>{
                const filePath = await save({
                    defaultPath: `~/Documents/${csv.ticker}-data.csv`,
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
