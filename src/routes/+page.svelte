<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    const today = new Date()
    const onYearAgo = new Date(today)
    onYearAgo.setFullYear(today.getFullYear() - 1)

    function formatDateToYYYYMMDD(date: Date) {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0'); // Adding 1 to month as it's zero-based
        const day = String(date.getDate()).padStart(2, '0');

        return `${year}-${month}-${day}`;
    }

    $: data = null

    invoke('fetch_morningstar_data', {
        ticker: "clm",
        mic: "xase",
        startDate: formatDateToYYYYMMDD(onYearAgo),
        endDate: formatDateToYYYYMMDD(today)
    }).then((_data)=>{
        console.log(_data)
        data = _data
    })

</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

