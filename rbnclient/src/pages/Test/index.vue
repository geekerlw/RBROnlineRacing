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
import { } from '../../reados'

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