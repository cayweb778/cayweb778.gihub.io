<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "../../components/Greet.vue";
// let abc=window.__TAURI__.window.WebviewWindow



import {onMounted, ref} from 'vue'
import {useDesktopStoreWidthOut} from "../../store/modules/hello";
import {invoke} from "@tauri-apps/api/tauri";
import {
  Form,
  Button,
  Select,
  Input,
  Space,
  Row,
    Col,
  TypographyText,
  TypographyLink,
    DatePicker,
  Descriptions,
  DescriptionsItem
} from 'ant-design-vue'

const Item = Form.Item
const cacheDir = ref('')
const hello = ref('127.0.0.1:3000')
import {readDir, removeDir, BaseDirectory} from '@tauri-apps/api/fs';
const printStr=ref()
async function getPrintLn(){
  // addr.value=await invoke("get_printers_all")
  // console.log(addr.value)
  const aaaa=await invoke("get_printers_all")
  printStr.value = JSON.parse(await invoke("get_printers_all"));
  console.log(printStr.value)
}

function openAbc() {
  addr.value = hello.value
  useDesktopStoreWidthOut().goApp(hello.value)
}

const options = ref([
  {key: '1', label: '总账 localhost:3500', value: '1'},
  {key: '1', label: '总账 localhost:3500', value: '1'},
  {key: '1', label: '总账 localhost:3500', value: '1'},
  {key: '1', label: '总账 localhost:3500', value: '1'},
])
const addr = ref()
const abc11 = ref()
onMounted(async () => {
  addr.value = JSON.parse(await invoke("get_cache_ip_addr_api")).data;
  console.log(addr.value)
  hello.value = addr.value
  console.log(addr.value)
})

function funasd() {
  if (hello.value == '127.0.0.1:3000') {
    hello.value = '81.70.47.206:81'
    return
  }
  if (hello.value == '81.70.47.206:81') {
    hello.value = '127.0.0.1:3000'
    return
  }
  hello.value = '127.0.0.1:3000'

}
function openFooterFn(){
  useMainStoreWidthOut().openFooter()
}
import {appWindow} from '@tauri-apps/api/window';

import {readDir, removeDir, BaseDirectory} from '@tauri-apps/api/fs';
import {useMainStoreWidthOut} from "../../store/modules/main";


async function clearCache() {
  await invoke("clear_cache");
  // await removeDir('org.boozsoft.ncapp', {dir: BaseDirectory.LocalData,recursive:true})
  //     .then((e)=>{
  //       useDesktopStoreWidthOut().goApp(hello.value)
  //       abc11.value=e
  //     })
  //     .catch((e)=>{
  //       useDesktopStoreWidthOut().goApp(hello.value)
  // abc11.value=e
  // })
  // console.log("dddd")
  // console.log(aaaa)
  // const aaa=await appLocalDataDir()
  console.log("wujie")
  // const ddddd= await removeDir('', {dir: BaseDirectory.Cache,recursive:true})
  //      .then(() => {
  //
  //        useDesktopStoreWidthOut().goApp(hello.value)
  //      })
  //      .finally(() => {
  //
  //        useDesktopStoreWidthOut().goApp(hello.value)
  //      });
}
async function makeInfo(){
  await invoke("generate", {name: hello.value});
}
async function closeApp(){
  // useMainStoreWidthOut().closeApp()
 await invoke("close_app")
}
async function openApp(){
  // useMainStoreWidthOut().openApp()
  useMainStoreWidthOut().setShowHello(false)
  // window.__TAURI__.window.getCurrent().hide()
  // await invoke("generate", {name: hello.value});
  await invoke("go_appaaa", {name:hello.value });
  // openAbc()
}
async function getC() {
  console.log(BaseDirectory)
  // debugger
  // const  AppConfig=await readDir('', { dir: BaseDirectory.  AppConfig,recursive:true });
  // debugger
  // const  AppData=await readDir('', { dir: BaseDirectory.  AppData,recursive:true });
  function windowAndLinux() {
    // const  AppLocalData=await readDir('', { dir: BaseDirectory.  AppLocalData,recursive:true });
    removeDir('databases', {dir: BaseDirectory.AppLocalData, recursive: true})
    removeDir('localstorage', {dir: BaseDirectory.AppLocalData, recursive: true})
    removeDir('EBWebView', {dir: BaseDirectory.AppLocalData, recursive: true})

  }

  function macos() {
    removeDir('/Users/cayweb/Library/WebKit/财税达T6', {recursive: true})
  }

  function T(fn: any) {
    try {
      fn()
    } catch {
      console.log('可能平台信息不一致')
    }
  }

  try {
    await windowAndLinux()
  } catch {
    console.log('可能平台信息不一致')
  }
  try {
    await macos()
  }catch {
    console.log('可能平台信息不一致')
  }


  // T(() =>))
  // T(()=>))
  // AppLocalData.forEach(it=>{
  //   // await readDir('', { dir: BaseDirectory.  AppLocalData,recursive:true });
  //
  // })
  // console.log(AppLocalData)
  // cacheDir.value=AppLocalData
  // const  AppCache=await readDir('', { dir: BaseDirectory.  AppCache,recursive:true });
  // const  AppLog=await readDir('', { dir: BaseDirectory.  AppLog,recursive:true });
  // addr.value = JSON.parse(await invoke("get_cache_dir")).data;
}
// onMounted(()=>{
//   var WebviewWindow = window.__TAURI__.window.WebviewWindow
//   var appWindow = window.__TAURI__.window.appWindow
//   var windowLabel = appWindow.label
//   var windowLabelContainer = document.getElementById('window-label')
//   windowLabelContainer.innerText = 'This is the ' + windowLabel + ' window.'
//
//   var container = document.getElementById('container')
//
//   function createWindowMessageBtn(label) {
//     var tauriWindow = WebviewWindow.getByLabel(label)
//     var button = document.createElement('button')
//     button.innerText = 'Send message to ' + label
//     button.addEventListener('click', function () {
//       tauriWindow.emit('clicked', 'message from ' + windowLabel)
//     })
//     container.appendChild(button)
//   }
//
//   // global listener
//   window.__TAURI__.event.listen('clicked', function (event) {
//     responseContainer.innerHTML +=
//         'Got ' + JSON.stringify(event) + ' on global listener\n\n'
//   })
//   window.__TAURI__.event.listen('tauri://window-created', function (event) {
//     createWindowMessageBtn(event.payload.label)
//   })
//
//   var responseContainer = document.getElementById('response')
//   // listener tied to this window
//   appWindow.listen('clicked', function (event) {
//     responseContainer.innerText +=
//         'Got ' + JSON.stringify(event) + ' on window listener\n\n'
//   })
//
//   var createWindowButton = document.createElement('button')
//   createWindowButton.innerHTML = 'Create window'
//   createWindowButton.addEventListener('click', function () {
//     var webviewWindow = new WebviewWindow(
//         Math.random().toString().replace('.', ''),
//         {
//           tabbingIdentifier: windowLabel
//         }
//     )
//     webviewWindow.once('tauri://created', function () {
//       responseContainer.innerHTML += 'Created new webview'
//     })
//     webviewWindow.once('tauri://error', function (e) {
//       responseContainer.innerHTML += 'Error creating new webview'
//     })
//   })
//   container.appendChild(createWindowButton)
//
//   var globalMessageButton = document.createElement('button')
//   globalMessageButton.innerHTML = 'Send global message'
//   globalMessageButton.addEventListener('click', function () {
//     // emit to all windows
//     window.__TAURI__.event.emit('clicked', 'message from ' + windowLabel)
//   })
//   container.appendChild(globalMessageButton)
//
//   var allWindows = window.__TAURI__.window.getAll()
//   for (var index in allWindows) {
//     var label = allWindows[index].label
//     if (label === windowLabel) {
//       continue
//     }
//     createWindowMessageBtn(label)
//   }
// })
function openSingleAppDebug() {

}
function helloabc(){
  useMainStoreWidthOut().openAbc()
}
</script>

<template>
  <div class="helloconfig"
       style="height:300px;overflow: scroll;text-align: center;border-radius:30px;position:fixed;left:0;width:100vw;top:0;height:100vh;background:#2b2b2b;color:white">
   <DatePicker/>
    <space direction="vertical" style="padding:10px 5px">
      <h2 style="color:white;padding:0;margin:0;position:absolute;width:100vw;text-align:center">财税达配置中心</h2>

      <descriptions bordered title="" :size="'small'">
        <template #extra>
          <Button type="primary">编辑</Button>
        </template>

        <descriptions-item :span="4" label="当前服务器地址">

          <Row>
            <Input style="width:300px" v-model:value="hello"/>
            <Button @click="funasd">🚀</Button>

          </Row>
          <Row>
            <Button @click="helloabc">激活单应用调试特性</Button>
            <Button @click="closeApp">关闭app</Button>
            <Button @click="getC">清除缓存(需要重启)</Button>
            <Button @click="makeInfo">生成信息</Button>
            <Button @click="openApp">开启应用</Button>
          </Row>
        </descriptions-item>
        <DescriptionsItem label="打印控件配置">
          <Button>配置</Button>
        </DescriptionsItem>
        <!--        <DescriptionsItem label="表格控件配置"><Button>配置</Button></DescriptionsItem>-->
        <!--        <DescriptionsItem label="财税达文档"><Button>配置</Button></DescriptionsItem>-->
        <descriptions-item label="财税达官网">
          <Button>配置</Button>
        </descriptions-item>
        <descriptions-item label="泊舟官网">
          <Button>配置</Button>
        </descriptions-item>
<!--        <descriptions-item label="清除缓存">-->
<!--          <Button @click="clearCache">清理</Button>-->
<!--        </descriptions-item>-->
        <descriptions-item label="卸载">
          <Button>配置</Button>
        </descriptions-item>
        <descriptions-item label="退出">
          <Button>配置</Button>
        </descriptions-item>
<!--        <descriptions-item label="清除缓存">-->
<!--          <Button @click="closeApp">关闭应用</Button>-->
<!--          <Button @click="getC">清除缓存</Button>-->
<!--          <Button @click="openApp">开启应用</Button>-->
<!--          {{ cacheDir }}-->
<!--        </descriptions-item>-->
        <descriptions-item label="获取打印机列表">
          <Button @click="getPrintLn">获取打印机列表</Button>

        </descriptions-item>

        <descriptions-item label="打开托盘菜单">
          <Button @click="openFooterFn">打开托盘菜单</Button>

        </descriptions-item>
        <!--        <descriptions-item label="单应用调试">-->
        <!--         <Select style="width:300px" :options="options"></Select>-->
        <!--          <Button @click="openSingleAppDebug">确认</Button>-->
        <!--        </descriptions-item>-->

      </descriptions>
      <Row></Row>
    </space>
    <div style="color:white;height:100px;width:100vw;background:black">调试信息:::{{ printStr }}</div>
  </div>
</template>

<style scoped>
/*.helloconfig :deep(*) {*/
/*  color: white !important;*/
/*}*/

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
