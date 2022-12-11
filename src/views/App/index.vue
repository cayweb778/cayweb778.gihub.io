<script setup lang="ts">
import SiderButton from '../../openbutton.vue'
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import {useDesktopStore, useDesktopStoreWidthOut} from "../../store/modules/hello";
import {computed} from "vue";
// let abc=window.__TAURI__.window.WebviewWindow
const url=computed(()=>useDesktopStoreWidthOut().getUrl)

function openConfig(){
  var WebviewWindow = window.__TAURI__.window.WebviewWindow
  let webview = new WebviewWindow('theUniqueLabel', {
    url: "config",
  })
}
</script>

<template>
  <div style="position:fixed;z-index:2;">
    <SiderButton @clickThis="openConfig">
    </SiderButton>
  </div>
<!--<iframe :src="url" style="border:none;position: fixed;left:0;top:0;z-index:1;width:100vw;height:100vh;margin:0"></iframe>-->
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
