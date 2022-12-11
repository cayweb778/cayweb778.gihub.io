<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
// let abc=window.__TAURI__.window.WebviewWindow
import {Layout,Row,Col} from 'ant-design-vue'
</script>

<template>
 <Layout>
   <Row>最小化</Row>
   <Row>打开财税达</Row>
   <Row>打印机队列</Row>
   <Row>数据浏览器</Row>
   <Row>关于财税达</Row>
 </Layout>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
