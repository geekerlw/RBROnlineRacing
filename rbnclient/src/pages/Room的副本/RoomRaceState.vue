<template>
  <div class="racingState">
    <div class="roomMsg">
      房间号：3asdasdaas
    </div>
    <div class="statebar">
      游戏加载中/比赛进行中/比赛已结束
    </div>
    <div class="user userready">
      <div class="item"><span>player1</span> <span>加载中</span></div>
      <div class="item"><span>player1</span> <span>加载中</span></div>
      <div class="item"><span>player12ssssss</span> <span class="finish">加载完成</span></div>
      <!-- <div class="item"><span>player1</span> <span class="gaming">加载完成</span></div> -->
    </div>
    <div class="block20"> </div>
    <div class="action">
      <el-button type="primary">我要退出比赛</el-button>
    </div>
    <div class="block40"> </div>
    <!-- <div class="action">
      <div>疑似有人卡住的情况？房主可以发起踢人</div>
      <div class="block20"></div>
      <el-button type="primary">踢人</el-button>
    </div> -->
    <!-- 做个弹窗 -->
    
  </div>
</template>
<style lang="less" scoped>
.racingState{
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: #fff;
}
.roomMsg{
  margin-top: 30px;
  font-weight: bold;
}
.statebar{
  margin-top: 30px;
  color: #0006b3;
}
.user{
  position: relative;
  margin: 20px auto 0;
  width: 300px;
  text-align: left;
  border: 1px solid #c9c9c9;
  border-radius: 5px;
  padding: 10px;
  .item{
    margin-bottom: 15px;
    color: #383838;
    display: flex;
    justify-content: space-between;
    border-bottom: 1px solid #dadada;
    padding-bottom: 10px;
    padding-top: 5px;
    &:last-child{
      border-bottom: none;
      margin-bottom: 0;
    }
  }
}
.finish{
  color: rgb(57, 154, 0);
}
.gaming{
  color: #0006b3;
}
</style>