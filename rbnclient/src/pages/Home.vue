<template>
  <div class="home">
    <el-row justify="center">
      <el-col :span="24">
        <h2 class="text-center">{{ $t("hello") }}</h2>
      </el-col>
    </el-row>
    <div class="block20"></div>
    <el-row justify="center">
      <router-link to="/lobby">
        <el-button type="primary">去对战大厅</el-button></router-link
      >
    </el-row>
    <div class="block30"></div>
    <el-row justify="center">
      <div class="forpay">
        <img class="" :src="payimage" alt="" />
      </div>
      <div class="block20"></div>
      <el-col :span="24">
        <h4 class="text-center">软件免费捐助自愿，目前仅支持微信扫一扫哦～</h4>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import payimage from "../assets/appreciate.png";
</script>

<style lang="less" scoped>
  .forpay{
    width: 300px;
    margin: 0 auto;
    img{
      width: 100%;
    }
  }
</style>
