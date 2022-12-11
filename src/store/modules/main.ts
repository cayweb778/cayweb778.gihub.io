import {defineStore} from 'pinia';
import {store} from '/@/store';
import {getAddr} from "../../funs";
import {invoke} from "@tauri-apps/api/tauri";
import router from "../../router";


import {
    CloseRequestedEvent,
    CursorIcon,
    FileDropEvent,
    LogicalPosition,
    LogicalSize,
    Monitor,
    PhysicalPosition,
    PhysicalSize,
    ScaleFactorChanged,
    Theme,
    TitleBarStyle,
    UserAttentionType,
    WebviewWindow,
    WebviewWindowHandle,
    WindowLabel,
    WindowManager,
    WindowOptions,
    appWindow,
    availableMonitors,
    currentMonitor,
    getAll,
    getCurrent,
    primaryMonitor
    } from '@tauri-apps/api/window'
export const useMainStore = defineStore({
    id: 'useMainStore',
    state: (): any => ({
        showHello:false,

    }),
    getters: {
        getShowHello(state){
            return state.showHello
        }
    },
    actions: {
        setShowHello(e){
            return this.showHello=e
        },
        closeApp(){
            WebviewWindow.getByLabel('财税达ERP-NC企业管理软件').close()
        },
        async openApp(){
            console.log(99999999)
            invoke("go_appaaa", {name:"http:81.70.47.206:81/nc" });
            this.setShowHello(false)
            // window.__TAURI__.window.getCurrent().hide()
            return

            var appWindow = window.__TAURI__.window.appWindow
            var windowLabel = appWindow.label
            // 创建新窗口
            async function createWin(options) {
                // 系统参数配置
                const windowConfig = {
                    label: null,            // 窗口唯一label
                    title: '',              // 窗口标题
                    url: '',                // 路由地址url
                    tabbingIdentifier: windowLabel,
                    fileDropEnabled:true,
                    acceptFirstMouse:true,
                    // userAgent:true,
                    // skipTaskbar:false,
                    minWidth: null,         // 窗口最小宽度
                    minHeight: null,        // 窗口最小高度
                    x: null,                // 窗口相对于屏幕左侧坐标
                    y: null,                // 窗口相对于屏幕顶端坐标
                    resizable: true,        // 是否支持缩放
                    maximized: false,       // 最大化窗口
                    decorations: true,     // 窗口是否无边框及导航条
                    alwaysOnTop: false,     // 置顶窗口
                }

                const args = Object.assign({}, windowConfig, options)

                // 判断窗口是否存在
                // const existWin = getAll().find(w => w.label == args.label)
                // if(existWin) {
                //   if(existWin.label.indexOf('main') == -1) {
                //     await existWin?.unminimize()
                //     await existWin?.setFocus()
                //     return
                //   }
                //   await existWin?.close()
                // }

                // 创建窗口对象
                let win = new WebviewWindow(args.label, args)

                // 是否最大化
                if(args.maximized && args.resizable) {
                    win.maximize()
                }

                // 窗口创建完毕/失败
                win.once('tauri://created', async() => {
                    console.log('window create success!')
                })

                win.once('tauri://error', async() => {
                    console.log('window create error!')
                })
            }


            await createWin({
                label: '财税达ERP-NC企业管理软件',
                title: '财税达ERP-NC企业管理软件',
                url: 'http://81.70.47.206:81/nc',
                maximized: true,
                width: 320,
                height: 420,
                resizable: false,
                alwaysOnTop: false,
            })


        },
        async openFooter(){
            // this.setShowHello(false)
            // window.__TAURI__.window.getCurrent().hide()
            var appWindow = window.__TAURI__.window.appWindow
            var windowLabel = appWindow.label
            debugger
            // 创建新窗口
            async function createWin(options) {
                // 系统参数配置
                const windowConfig = {
                    label: null,            // 窗口唯一label
                    title: '',              // 窗口标题
                    url: '',                // 路由地址url
                    tabbingIdentifier: windowLabel,
                    // fileDropEnabled:true,
                    // acceptFirstMouse:true,
                    // userAgent:true,
                    // skipTaskbar:false,
                    minWidth: null,         // 窗口最小宽度
                    minHeight: null,        // 窗口最小高度
                    x: null,                // 窗口相对于屏幕左侧坐标
                    y: null,                // 窗口相对于屏幕顶端坐标
                    resizable: true,        // 是否支持缩放
                    maximized: false,       // 最大化窗口
                    decorations: true,     // 窗口是否无边框及导航条
                    alwaysOnTop: false,     // 置顶窗口
                }

                const args = Object.assign({}, windowConfig, options)

                // 判断窗口是否存在
                // const existWin = getAll().find(w => w.label == args.label)
                // if(existWin) {
                //   if(existWin.label.indexOf('main') == -1) {
                //     await existWin?.unminimize()
                //     await existWin?.setFocus()
                //     return
                //   }
                //   await existWin?.close()
                // }

                // 创建窗口对象
                let win = new WebviewWindow(args.label, args)

                // 是否最大化
                if(args.maximized && args.resizable) {
                    win.maximize()
                }

                // 窗口创建完毕/失败
                win.once('tauri://created', async() => {
                    console.log('window create success!')
                })

                win.once('tauri://error', async() => {
                    console.log('window create error!')
                })
            }


            await createWin({
                label: '托盘hello',
                title: '托盘hello-NC企业管理软件',
                url: 'http://localhost:1420/tuopan',
                maximized: false,
                width: 320,
                height: 420,
                x:window.screen.width-320-40,
                y:window.screen.height-420-80,
                decorations:false,
                resizable: false,
                alwaysOnTop: false,
            })


        },
        async openAbc(){
            // window.__TAURI__.window.getCurrent().hide()
            var appWindow = window.__TAURI__.window.appWindow
            var windowLabel = appWindow.label

            // 创建新窗口
            async function createWin(options) {
                // 系统参数配置
                const windowConfig = {
                    label: null,            // 窗口唯一label
                    title: '',              // 窗口标题
                    url: '',                // 路由地址url
                    tabbingIdentifier: 'aaasdsa',
                    // fileDropEnabled:true,
                    // acceptFirstMouse:true,
                    // userAgent:true,
                    // skipTaskbar:false,
                    minWidth: null,         // 窗口最小宽度
                    minHeight: null,        // 窗口最小高度
                    x: null,                // 窗口相对于屏幕左侧坐标
                    y: null,                // 窗口相对于屏幕顶端坐标
                    resizable: true,        // 是否支持缩放
                    maximized: false,       // 最大化窗口
                    decorations: true,     // 窗口是否无边框及导航条
                    alwaysOnTop: false,     // 置顶窗口
                }

                const args = Object.assign({}, windowConfig, options)

                // 判断窗口是否存在
                // const existWin = getAll().find(w => w.label == args.label)
                // if(existWin) {
                //   if(existWin.label.indexOf('main') == -1) {
                //     await existWin?.unminimize()
                //     await existWin?.setFocus()
                //     return
                //   }
                //   await existWin?.close()
                // }

                // 创建窗口对象
                let win = new WebviewWindow(args.label, args)

                // 是否最大化
                if(args.maximized && args.resizable) {
                    win.maximize()
                }

                // 窗口创建完毕/失败
                win.once('tauri://created', async() => {
                    console.log('window create success!')
                })

                win.once('tauri://error', async() => {
                    console.log('window create error!')
                })
            }


            await createWin({
                label: 'ddd',
                title: 'd-NC企业管理软件',
                url: 'chrome://flags/#block-insecure-private-network-requests',
                maximized: true,
                width: 320,
                height: 420,

                decorations:false,
                resizable: false,
                alwaysOnTop: true,
            })
        }
    }
});

if (WebviewWindow.getByLabel("财税达ERP-NC企业管理软件")==null){
    useMainStoreWidthOut().setShowHello(true)
}
// Need to be used outside the setup
export function useMainStoreWidthOut() {
    return useMainStore(store);
}

