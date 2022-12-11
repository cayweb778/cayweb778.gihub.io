<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
// let abc=window.__TAURI__.window.WebviewWindow
import { readDir , BaseDirectory } from '@tauri-apps/api/fs';
async function  abssss(){
  console.log(BaseDirectory)
  debugger
  const  AppConfig=await readDir('', { dir: BaseDirectory.  AppConfig,recursive:true });
  debugger
  const  AppData=await readDir('', { dir: BaseDirectory.  AppData,recursive:true });
  const  AppLocalData=await readDir('', { dir: BaseDirectory.  AppLocalData,recursive:true });
  const  AppCache=await readDir('', { dir: BaseDirectory.  AppCache,recursive:true });
  const  AppLog=await readDir('', { dir: BaseDirectory.  AppLog,recursive:true });
}
</script>

<template>
  <div class="container">
    <h1>Welcome to 泊舟NC!<button @click="abssss">hello</button></h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="../../assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>点击 NC, 泊舟, 微服务 学习更多.</p>

    <p>
      初次使用:

      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
        >指定服务地址</a
      >
    </p>

    <Greet @ok="emit('ok',$event)" />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
