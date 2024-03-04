<template>
  <div class="test">
    <div>
      <el-button @click="back">返回</el-button>
    </div>
    <div>
      <el-button @click="loadconfig">loadconfig</el-button>
      <el-button @click="setconfig">testSet</el-button>
    </div>
  </div>


</template>

<script setup>
import { save_store_config, load_store_config } from '../../reados'
const back = () => {
  history.back()
};
const setconfig = () => {
  save_store_config()
}
const loadconfig = () => {
  load_store_config('server_addr').then((res) => {
    console.log(res)
  })
}
</script>

<style lang="less" scoped>
  .test {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    height: 100%;
  }
</style>