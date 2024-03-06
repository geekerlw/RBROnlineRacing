<template>
  <div class="test">
    <div>
      <el-button @click="back">返回</el-button>
    </div>
    <div>
      <el-button @click="testGet">test</el-button>
    </div>
  </div>


</template>

<script setup>
import { load_game_stage_options } from '../../reados'
const back = () => {
  history.back()
}
const testGet = () => {
  console.log('testGet')
  load_game_stage_options().then(res => {
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