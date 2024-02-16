import { invoke } from '@tauri-apps/api/tauri'

function formatDateToYYYYMMDD(date: Date) {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0'); // Adding 1 to month as it's zero-based
    const day = String(date.getDate()).padStart(2, '0');

    return `${year}-${month}-${day}`;
}

export async function buildDataset(ticker: string, mic: string, start: Date, end: Date, initialPositionValue: number, color: string, dripAtNav = false) {
    const data: any = await invoke('fetch_data', {
        ticker: ticker,
        mic: mic,
        startDate: formatDateToYYYYMMDD(start),
        endDate: formatDateToYYYYMMDD(end),
        initialPositionValue,
        dripAtNav,
    })
    const { 
        //no_drip, drip, drip_at_nav,
        no_drip_total_return, drip_total_return, drip_at_nav_total_return, csv
    } = data

    const label = ticker.toUpperCase()

    return {
        noDrip: {
            label: label,
            type: 'line',
            data: no_drip_total_return,
            borderColor: color,
            backgroundColor: color,
        },
        drip: {
            label: label,
            type: 'line',
            data: drip_total_return,
            borderColor: color,
            backgroundColor: color,
        },
        dripAtNav: dripAtNav ? {
            label: label,
            type: 'line',
            data: drip_at_nav_total_return,
            borderColor: color,
            backgroundColor: color,
        } : null,
        csv,
        ticker,
    }

}
