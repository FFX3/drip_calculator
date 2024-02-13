function formatDateToYYYYMMDD(date: Date) {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0'); // Adding 1 to month as it's zero-based
    const day = String(date.getDate()).padStart(2, '0');

    return `${year}-${month}-${day}`;
}

async function getMorningstarSecurityMetadata(ticker: string, mic: string) {
    const url = "https://www.morningstar.com/api/v2/cefs/"+mic+"/"+ticker+"/chart"
    const response = await fetch(url);
    const json = await response.text();
    const data = JSON.parse(json);

    return {
        securityID: data.security.securityID,
        sessionToken: data.chart.payload.token,
    }
}

export async function getMorningstarChartData(ticker: string, mic: string, startDate: Date, endDate?: Date) {    
    return;

    endDate = endDate ?? new Date()

    const { securityID, sessionToken } = await getMorningstarSecurityMetadata(ticker, mic)
    const  headers = {
        'Authorization': 'Bearer ' + sessionToken,
        'Accept': "application/json, text/plain, */*",
        'Accept-Encoding': "gzip, deflate, br",
        'Accept-Language': "en-US,en;q=0.5",
        'Connection': "keep-alive",
        'Origin': "https://www.morningstar.com",
        'Referer': "https://www.morningstar.com/cefs/xase/clm/chart",
        'User-Agent': "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0"
    };

    const options = {
        headers: headers,
    };

    let url = "https://www.us-api.morningstar.com/QS-markets/chartservice/v2/timeseries?query="+securityID+":totalReturn,nav,open,high,low,close,volume,previousClose"
        url = url + "&frequency=d"
    url = url + `&startDate=${formatDateToYYYYMMDD(startDate)}`
    url = url + `&endDate=${formatDateToYYYYMMDD(endDate)}`
    url = url + "&trackMarketData=3.6.3"
    url = url + "&instid=DOTCOM"
    const response = await fetch(url, options);
    const json = await response.text();
    const data = JSON.parse(json);
    const chartData = data[0].series
    const values = chartData.map((jsonData: any)=>{
        return [
            jsonData.date,
            jsonData.open,
            jsonData.high,
            jsonData.low,
            jsonData.close,
            jsonData.volume,
            jsonData.nav,
            jsonData.previousClose,
            jsonData.totalReturn
        ]
    })

    return [[
        "date",
        "open",
        "high",
        "low",
        "close",
        "volume",
        "nav",
        "previousClose",
        "totalReturn"
    ], ...values]
}
