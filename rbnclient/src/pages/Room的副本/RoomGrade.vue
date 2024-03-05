<template>
  <div class="racingState">
    <div class="block30"></div>
    <div class="grade-content" id="gradeContent">
      <div class="roomMsg">
        比赛成绩
      </div>
      <div class="stateDate">
        Svince | 2024/02/04 22:00
      </div>
      <div class="user userready">
        <div class="item num1">
          <span class="num">
            <el-icon class="icon">
              <Medal />
            </el-icon>
          </span>
          <span class="player">player1</span>
          <span class="end">11.111</span>
          <span class="car">HydunWRC1.6</span>
        </div>
        <div class="item num2">
          <span class="num">
            <el-icon class="icon">
              <Medal />
            </el-icon>
          </span>
          <span class="player">player1</span>
          <span class="end">11.111</span>
          <span class="car">HydunWRC1.6</span>
        </div>
        <div class="item num3">
          <span class="num">
            <el-icon class="icon">
              <Medal />
            </el-icon>
          </span>
          <span class="player">player1</span>
          <span class="end">11.111</span>
          <span class="car">HydunWRC1.6</span>
        </div>
        <div class="item">
          <span class="num">4</span>
          <span class="player">player1</span>
          <span class="end">11.111</span>
          <span class="car">HydunWRC1.6</span>
        </div>
        <div class="item">
          <span class="num">5</span>
          <span class="player">player1</span>
          <span class="end">11.111</span>
          <span class="car">HydunWRC1.6</span>
        </div>
      </div>
    </div>

    <div class="action">
      <el-button type="primary">知道了</el-button>
      <el-button type="primary" @click="download">下载成绩单</el-button>
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
<script setup>
import { onMounted } from 'vue';
import domtoimage from 'dom-to-image';
import { downloadDir } from '@tauri-apps/api/path';
import { writeBinaryFile } from '@tauri-apps/api/fs';
import { ElMessage } from 'element-plus';
onMounted(() => {
});


const download = () => {
  var node = document.getElementById('gradeContent');
  domtoimage.toPng(node)
    .then(function (dataUrl) {

      let base64Data = dataUrl.replace(/^data:image\/\w+;base64,/, "");
      // 将 base64 数据解码
      let binary = window.atob(base64Data);
      let len = binary.length;
      let buffer = new Uint8Array(len);
      for (let i = 0; i < len; i++) {
        buffer[i] = binary.charCodeAt(i);
      }

      downloadDir().then((r) => {
        writeBinaryFile(`${r}/grade.png`, buffer).then(() => {
          // toast，已经下载到某某路径
          ElMessage({
            message: '已下载到' + `${r}/grade.png`,
            grouping: true,
            type: 'success',
          })
        });
      });
    })
    .catch(function (error) {
      console.error('oops, something went wrong!', error);
    });
}

</script>
<style lang="less" scoped>
.racingState {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: #fff;
}

.roomMsg {
  font-weight: bold;
}

.statebar {
  margin-top: 30px;
  color: #0006b3;
}

#gradeContent{
  padding-top: 30px;
  background: #fff;
  padding-bottom: 30px;
}

.user {
  position: relative;
  margin: 20px auto 0;
  width: 400px;
  text-align: left;
  border: 1px solid #c9c9c9;
  border-radius: 5px;
  padding: 10px;

  .item {
    margin-bottom: 15px;
    color: #383838;
    display: flex;
    justify-content: flex-start;
    border-bottom: 1px solid #dadada;
    padding-bottom: 10px;
    padding-top: 5px;

    &:last-child {
      border-bottom: none;
      margin-bottom: 0;
    }

    .num {
      width: 30px;
      text-align: center;
      margin-right: 10px;
    }

    .player {
      width: 120px;
    }

    .end {
      width: 100px;
    }

    // .car {
    //   calc(100% - 170px)
    // }
  }
}

.num1 .icon {
  color: #FFD700;
}

.num2 .icon {
  color: #C0C0C0;
}

.num3 .icon {
  color: #B87333;
}

.finish {
  color: rgb(57, 154, 0);
}

.gaming {
  color: #0006b3;
}

.stateDate {
  margin-top: 20px;
  font-size: 14px;
  color: #818181;
}
</style>