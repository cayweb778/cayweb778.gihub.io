<template>
 <Layout style="position:fixed;left:0;top:0;width:100vw;height:100vh;background: url('/hello.jpeg')">
   <Row style="margin:110px auto 0;padding:10px 20px;border-radius: 10px">
     <Col>
       <Row style="margin-left:0px;color:white;font-size:40px;font-weight: 900;">财税达T6</Row>
       <Row style="color:white;font-size:12px;font-weight: 900;color:gray">终端：81.70.47.206:81 调整</Row>
       <Row style="color:white;font-size:12px;font-weight: 900;color:gray">版本：NC WA+ 20221200</Row>
       <Row style="text-align: center;color:white;font-size:25px;margin-top:40px;font-weight: 900;color:gray"><Button @click="openAppFn" style="background:blue;color:white">进入系统</Button></Row>
     </Col>
   </Row>
 </Layout>
</template>
<script setup>
import {Layout,Row,Col,TypographyText} from 'ant-design-vue'
import {useMainStoreWidthOut} from "../../store/modules/main";
import {invoke} from "@tauri-apps/api/tauri";
function openAppFn(){
  useMainStoreWidthOut().openApp()
  window.__TAURI__.window.getCurrent().hide()
}
const aaaa=invoke("hell")
</script>