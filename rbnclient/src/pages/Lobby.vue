<template>
  <div class="lobby">
    <div class="title">
        <div class="left-group">
            <el-button class="rb" type="default" @click="goHome"><el-icon><ArrowLeft /></el-icon>返回</el-button>
            <el-button class="cr" type="success" @click="back">创建房间</el-button>
        </div>
        {{$t('lobbyTitle')}}
    </div>
    <div class="block20"></div>
    <div class="roomlist">
      <div class="item">
        <div class="name line">房名: jakebless</div>
        <div class="map line">地图: lyon-Geend</div>
        <div class="roomowener line">房主: jakebless</div>
        <div class="roomusers line">人数: 1/2</div>
        <div class="status line">状态: 等待中</div>
        <div class="action">
          <el-button type="primary" @click="back">加入</el-button>
          <el-button type="danger" @click="back">退出比赛</el-button>
        </div>
      </div>
    </div>
  </div>
  <!-- <router-link :to="'/room/' + room">{{room}}</router-link> -->
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();
const goHome = () => {
  router.push("/home");
};

const back = () => {
  history.back();
};

const dataList = ref([]);
onMounted(() => {
  dataList.value = ["room1", "room2", "room3"];
});
</script>

<style lang="less" scoped>
.lobby{
    width: 100%;
    cursor: default;
}
.title{
    font-size: 30px;
    font-weight: bold;
    text-align: center;
    margin-bottom: 20px;
    position: relative;
    margin: 0 auto;
    width: 90%;
    .left-group{
        left: 0;
        position: absolute;
    }
}
.roomlist{
    display: flex;
    width: 90%;
    margin: 0 auto;
    justify-content: flex-start;
    .item{
        width: 200px;
        border: 1px solid #ededed;
        padding: 20px;
        border-radius: 5px;
        background: #f8f8ff;
        box-shadow: 0 0 10px #ededed;
        .line{
            margin-bottom: 10px;
        }
    }
}
</style>>
