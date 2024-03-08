import { readTextFile, BaseDirectory, readDir, writeTextFile, removeFile } from '@tauri-apps/api/fs';

export type TickerConfiguration = {
    ticker: string,
    mic: string,
    dripAtNav: boolean,
    color: string
}

export type WatchListConfiguration = {
    [key: string]: TickerConfiguration
} 

export async function getWatchListConfigByName(name: string) {
    return await readTextFile(`ticker-confd/${name}`, { dir: BaseDirectory.AppConfig }).then(contents=>{
        if("" == contents){
            return false
        } else {
            return JSON.parse(contents)
        }
    })
}

export async function getWatchListNames() {
    const result = await readDir('ticker-confd', { dir: BaseDirectory.AppConfig, recursive: false });
    if(!result) { throw 'getWatchListNames error' } 
    return result.map(dir=>dir.name!).filter(n=>typeof n == 'string')
}

function saveWatchListConfiguration(name: string, watchList: WatchListConfiguration){
    writeTextFile({ path: 'ticker-confd/' + name, contents: JSON.stringify(watchList) }, { dir: BaseDirectory.AppConfig });
}

export async function configureTicker(watchListName: string, config: TickerConfiguration){
    let watchList = await getWatchListConfigByName(watchListName)
    watchList[config.ticker] = config
    saveWatchListConfiguration(watchListName, watchList)
    return {...watchList}
}

export async function deleteTicker(watchListName: string, ticker: string){
    let watchList = await getWatchListConfigByName(watchListName)
    delete watchList[ticker]
    saveWatchListConfiguration(watchListName, watchList)
    return {...watchList}
}

export function createWatchListConfiguration(name: string){
    saveWatchListConfiguration(name, {})
}

export async function deleteWatchListConfiguration(name: string){
    return await removeFile('ticker-confd/' + name , { dir: BaseDirectory.AppConfig });
}
