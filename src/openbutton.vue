<template>
  <div style="z-index: 1000000000;" @click="openOptionPage">
  <div tiao ref="tiao" style="width:18px;position:fixed;right:0;top:10px;height:100%;overflow: hidden"
       @mouseleave="onmouseleave"
       @mouseenter="onmouseenter">
    <div  @click="onmouseleave(tiao,true)"  style="cursor:pointer;height: 54px;width: 39px;padding: 11px 0px 0px 6px;box-sizing: border-box;position: fixed;right: 0px;top: calc(((100% - 30px) / 2) - 50px);opacity: 3.5;background: rgb(0 150 199);color: white;/* color: black; */cursor: pointer;border-radius: 0px 5px 5px 0px;font-size: 20px;">
      <SettingFilled />
    </div>
  </div>
    <div   class="left_sider_main"  v-show="showPromissSiteBar">
      <slot></slot>
    </div>
  </div>
</template>

<script setup lang="ts">

import {SettingFilled} from '@ant-design/icons-vue';
import {inject, nextTick, onMounted, ref} from "vue";
import {provide} from "vue";
const showOption=ref(false)


function onmouseenter(){
  if(showOption.value){
    return
  }
  if(tiao.value!=null){
    tiao.value.style.opacity='1'
  }
}
function onmouseleave (e,imm) {
  if(imm){
    e.style.opacity = '0'
  }

  let dom = e.target
  if (window.widthTimeout != null) {
    window.clearTimeout(window.widthTimeout)
  }
  window.widthTimeout = setTimeout(() => dom.style.opacity = '0', 1000)
}

const tiao=ref()

const emit=defineEmits(['clickThis'])
function openOptionPage() {
  emit('clickThis','')
  showPromissSiteBar.value = true
}
const showPromissSiteBar = ref(false)

onMounted(()=>{

  window.widthTimeout = setTimeout(() => {
    tiao.value.style.opacity = '0'
  }, 1000)
})
provide('showOption', showOption)
provide('leftSiderFun', {
  showOpenButton() {
    showOption.value = false

    setTimeout(()=>{
      showPromissSiteBar.value = false
    },1000)
  }
})
</script>

<style scoped>
.left_sider {
  height: 100vh;
  background: green;
}
.left_sider_main {
  min-width: 228px;
  width: 228px;
  height: 100%
}
</style>
