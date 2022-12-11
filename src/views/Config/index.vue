<template>
  <Layout>
    <Hello v-if="showHello"/>
    <Config v-else/>
  </Layout>
</template>
<script setup>
import {Layout,Row,Col} from 'ant-design-vue'
import Hello from './Hello.vue'
import Config from './Config.vue'

import {ref,onMounted,computed} from 'vue'
import {useMainStoreWidthOut} from "../../store/modules/main";
const showHello=computed(()=>useMainStoreWidthOut().getShowHello)

</script>