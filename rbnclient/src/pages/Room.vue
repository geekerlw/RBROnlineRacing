<template>
  <div class="room">
    <div class="title">
      <div class="left-group">
        <el-button class="rb" type="default" @click="back"><el-icon>
            <ArrowLeft />
          </el-icon>返回</el-button>
      </div>
      {{ $t("比赛房间") }}
    </div>
    <div class="roommsgbox">
      <div class="sub-title">基本信息</div>
      <div class="msgwrapper">
        <div class="name line">房间名: {{ room.name }}</div>
        <div class="roomowner line">房主: {{ room.owner }}</div>
        <div class="password line">密码: {{  }}</div>
        <div class="roomplayers line">人数: {{ room.players }}/8</div>
        <div class="stage line">赛道: {{ room.stage }}</div>
        <div class="stagelength line">赛道长度： 15km</div>
        <div class="reloadtype line">路面类型：Tarmac</div>
        <div class="wetdry line">湿滑情况： Dry</div>
        <div class="damage line">车辆损坏： Realistic</div>  
      </div>
      <div class="block10"></div>
      <div class="sub-title">
        车辆和调教
      </div>
      <div class="msgwrapper">
        <div class="car line">车辆选择：hydun</div>
        <div class="set line">调教选择：</div>
      </div>
      <div class="block10"></div>
      <div class="sub-title">
        房间状态
      </div>
      <div class="statewrapper">
        <div class="state">当前状态：比赛中/比赛准备中/比赛加载中/比赛未开始</div>
        <div class="block20"></div>
        <div class="action">
          <el-button>开始比赛</el-button>
          <el-button type="default">催房主开始</el-button>
          <el-button type="error">退出比赛</el-button>
          <el-button type="ready">准备</el-button>
        </div>

      </div>

    </div>


  </div>
</template>

<script setup>
import { reactive } from 'vue';

const back = () => {
  history.back()
}
const room = reactive({
  name: 'jakebless',
  stage: 'lyon-Geend',
  owner: 'jakebless',
  players: 1,
  state: '等待中'
})

</script>

<style lang="less" scoped>
.room {
  width: 100%;
  cursor: default;
}
.sub-title{
  font-size: 20px;
  font-weight: bold;
  text-align: left;
  width: 90%;
  margin: 0 auto 20px;
  border-bottom: 1px solid #dbdbdb;
  padding-bottom: 8px;
}
.msgwrapper{
  display: flex;
  width: 90%;
  margin: 0 auto;
  justify-content: flex-start;
  flex-wrap: wrap;
  .line{
    text-align: left;
    width: 50%;
    margin-bottom: 10px;
  }
}
.statewrapper{
  width: 90%;
  margin: 0 auto;
  text-align: left;
  .action{}
}
.title {
  font-size: 24px;
  font-weight: bold;
  text-align: center;
  position: relative;
  margin: 0 auto 30px;
  width: 90%;

  .left-group {
    left: 0;
    position: absolute;
  }
}
.roommsgbox{
}
</style>