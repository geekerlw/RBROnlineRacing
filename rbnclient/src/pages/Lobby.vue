<template>
  <div class="lobby">
    <div class="title">
      <div class="left-group">
        <el-button class="rb" type="default" @click="goHome"
          ><el-icon><ArrowLeft /></el-icon>返回</el-button
        >
        <el-button class="cr" type="success" @click="showCreateRace"
          >创建房间</el-button
        >
      </div>
      {{ $t("lobbyTitle") }}
    </div>
    <div class="racelist">

    <el-card class="item"  shadow="hover" v-for="(race, index) in raceList" :key="race">
      <div class="name line">房名: {{ race.name }}</div>
      <div class="stage line">地图: {{ race.stage }}</div>
      <div class="owner line">房主: {{ race.owner }}</div>
      <div class="players line">人数: {{ race.players }}/8</div>
      <div class="status line">状态: {{ stateText(race.state) }}</div>
      <div class="action">
        <el-button type="primary" v-if="race.state == '等待中'" @click="back"
          >加入房间</el-button
        >
        <el-button type="primary" v-if="race.state == '等待中'" @click="gorace"
          >进入房间</el-button
        >
        <el-button type="danger" v-if="race.state == '进行中'" @click="back"
          >退出比赛</el-button
        >
      </div>
    </el-card>
    </div>
  </div>
  <CreateOrEditRace ref="createRef"></CreateOrEditRace>
  <!-- <router-link :to="'/room/' + room">{{room}}</router-link> -->
</template>

<script setup>
import { ref, onMounted, computed } from "vue";
import { useRouter } from "vue-router";
import CreateOrEditRace from "../components/CreateOrEditRace.vue";

const router = useRouter();
const goHome = () => {
  router.push("/home");
};
const back = () => {
  history.back();
};
const gorace = () => {
  router.push('/room/1');
}

const roomMockList = [
  {
    name: "jakebless",
    stage: "lyon-Geend",
    owner: "jakebless",
    players: 5,
    state: "等待中",
  },
];

const raceList = ref([]);

const stateText = (state) => {
  switch (state) {
    case "等待中":
      return "等待中";
    case "进行中":
      return "进行中";
    case "已结束":
      return "已结束";
    default:
      return "未知";
  }
};

const createRef = ref(null);
const showCreateRace = () => {
  createRef.value.showCreateRace();
};

const ssadsds = ref(null);

onMounted(() => {
  raceList.value = roomMockList;
});
</script>

<style lang="less" scoped>
.lobby {
  width: 100%;
  cursor: default;
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
.racelist {
  display: flex;
  width: 90%;
  margin: 0 auto;
  justify-content: flex-start;
  .item {
    width: 240px;
    // border: 1px solid rgb(225, 229, 250);
    // padding: 20px;
    // border-radius: 5px;
    // background: #ebeeff;
    // box-shadow: 0 0 10px #dddddd;
    .line {
      margin-bottom: 10px;
    }
  }
}
</style>
