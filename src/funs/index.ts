import {invoke} from "@tauri-apps/api/tauri";

export async function getAddr() {
    const addrJSON= await invoke("getCacheIpAddr", {name: name.value});
    return JSON.parse(addrJSON)
}

export function setupConfigKeydownListener(){
    // let webview=null
    // window.addEventListener("keydown",(e)=>{
    //     var WebviewWindow = window.__TAURI__.window.WebviewWindow
    //     webview = new WebviewWindow('theUniqueLabel', {
    //         url: 'http://localhost:1420/config',
    //     })
    // })
}