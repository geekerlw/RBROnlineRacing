<template>
  <div class="home">
    <el-row justify="center">
      <el-col :span="24">
        <h2 class="text-center">{{ $t("hello") }}</h2>
      </el-col>
    </el-row>
    <div class="block20"></div>
    <el-row justify="center">

      <!-- 输入框，账号密码，登录按钮 -->
      <!-- 登录部分 -->
      <div class="login" v-if="!globalStore.token">
        <div class="block20"></div>
        <el-form label-position="right" label-width="60px">
          <el-form-item label="用户名">
            <el-input v-model="logInForm.name"></el-input>
          </el-form-item>
          <el-form-item label="密码">
            <el-input type="password" v-model="logInForm.passwd"></el-input>
          </el-form-item>
        </el-form>
        <el-button type="primary" @click="handleLogIn">登录</el-button>
      </div>
      <router-link to="/lobby" v-if="globalStore.token">
        <el-button type="primary">去对战大厅</el-button>
      </router-link>
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
import { ref } from 'vue';
import payimage from "../../assets/appreciate.png";
import { login } from '../../api';
import { ElMessage } from 'element-plus'
import { useGlobalStore } from '../../store'

const globalStore = useGlobalStore();

const logInForm = ref({ name: '', passwd: 'simrallycn' });
const handleLogIn = () => {
  if (!logInForm.value.name || !logInForm.value.passwd) {
    ElMessage({
      message: '请输入用户名和密码',
      grouping: true,
      type: 'error'
    })
    return
  }
  login({
    name: logInForm.value.name,
    passwd: logInForm.value.passwd
  }).then((res) => {
    console.log(res)
    ElMessage({
      message: `登录成功`,
      grouping: true,
      type: 'success',
    })
    globalStore.logined(res, logInForm.name)
  }).catch((err) => {
    console.log(err, 'err')
    ElMessage({
      message: '登录失败，密码可能错了',
      grouping: true,
      type: 'error'
    })
  })
}

</script>
  
<style lang="less" scoped>
.forpay {
  width: 300px;
  margin: 0 auto;

  img {
    width: 100%;
  }
}
</style>
  