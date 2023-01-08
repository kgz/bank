// create a function to handling retriving data from config.apiUrl


const config = {
    apiUrl: 'http://127.0.0.1:3030/api/'
}

const fetch_api = (url, options = {}, callback = (_) => {}) => {
    fetch(config.apiUrl + url, options).then(res => res.json()).then(data => {
        callback(data)
    })
}

export default fetch_api