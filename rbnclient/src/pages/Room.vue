<template>
  <div class="room">
    <div class="title">
      <div class="left-group">
        <el-button class="rb" type="default" @click="back"><el-icon>
            <ArrowLeft />
          </el-icon>返回大厅</el-button>
      </div>
      比赛房间
    </div>
    <div class="roommsgbox">
      <div class="sub-title">基本信息</div>
      <div class="msgwrapper msgfirst">
        <div class="name line">房间名: {{ room.name }}</div>
        <div class="roomowner line">房主: {{ room.owner }}</div>
        <div class="password line">密码: 123{{  }}</div>
        <div class="roomplayers line">人数: {{ room.players }}/8</div>
        <div class="stage line">赛道: {{ room.stage }}</div>
        <div class="stagelength line">赛道长度： 15km</div>
        <div class="reloadtype line">路面类型：Tarmac</div>
        <div class="wetdry line">湿滑情况： Dry</div>
        <div class="weatherstatus line">天气状况： Good</div>
        <div class="weathertype line">天气类型：asdas</div>
        <div class="damage line">车辆损坏： Realistic</div>  
      </div>
      <div class="sub-title">
        车辆和调教
      </div>
      <div class="msgwrapper">
        <div class="car line sel">车辆选择：
          <el-select size="small" v-model="car" disabled placeholder="请选择车辆" filterable>
            <el-option
              v-for="item in carList"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            ></el-option>
          </el-select>
          <span class="limit">车辆已限定</span>
        </div>
        <div class="set line sel">调校选择：
          <el-select size="small" v-model="car" placeholder="请选择车辆" filterable>
            <el-option
              v-for="item in carList"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            ></el-option>
          </el-select>
        </div>
        <div class="type line sel">轮胎类型：
          <el-select size="small" v-model="car" placeholder="请选择车辆" filterable>
            <el-option
              v-for="item in carList"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            ></el-option>
          </el-select>
        </div>
      </div>
      <div class="sub-title">
        房间状态
      </div>
      <div class="statewrapper">
        <div class="disabled" v-if="false">比赛中...</div>
        <div class="state">当前状态：等待比赛中/比赛中/比赛准备中/比赛加载中</div>
        <div class="block10"></div>
        <div class="action">
          房主操作：
          <el-button type="primary">开始比赛并准备</el-button>
          <el-button type="primary">修改比赛</el-button>
          <el-button type="primary">随机地图</el-button>
          <span class="owentips">当前你是房主哦</span>
        </div>
        <div class="block15"></div>
        <div class="action">
          成员操作：
          <el-button type="primary">催房主开始</el-button>
          <el-button type="warning">退出比赛</el-button>
          <el-button type="primary">准备</el-button>
        </div>
      </div>
      <div class="block15"></div>
      <div class="sub-title">
        房间成员
      </div>
      <div class="msgwrapper members">
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>        
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
        <div class="line">
          <span>1</span>
          <span>jakebless</span>
          <span>未就绪</span>
        </div>
      </div>

    </div>


  </div>
</template>

<script setup>
import { ref, reactive } from 'vue';
const car = ref('')
const carList = [
  {
    value: 'car1',
    label: 'car1'
  },
  {
    value: 'car2',
    label: 'car2'
  }
]

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
.car .limit{
  margin-left: 10px;
  font-size: 12px;
}
.line.sel{
  display: flex;
  align-items: center;
  .el-select{
    width: 200px;
  }
}
.sub-title{
  font-size: 16px;
  font-weight: bold;
  text-align: left;
  width: 90%;
  margin: 0 auto 20px;
  border-top: 1px solid #dbdbdb;
  padding-top: 15px;
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
.msgwrapper.msgfirst{
  .line{
    width: 33%;
  }

}
.statewrapper{
  width: 90%;
  margin: 0 auto;
  text-align: left;
  position: relative;
  .disabled{
    position: absolute;
    left: -2%;
    top: -10%;
    width: 104%;
    height: 120%;
    background-color: #c5c5c5;
    opacity: 0.5;
    z-index: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    color: #000;
  }
}
.title {
  font-size: 24px;
  font-weight: bold;
  text-align: center;
  position: relative;
  margin: 0 auto 20px;
  width: 90%;

  .left-group {
    left: 0;
    position: absolute;
  }
}
.owentips{
  color: #409eff;
  margin-left: 10px;
}
.members{
  .line{
    display: flex;
    width: 30%;
    margin-right: 2%;
    padding-bottom: 5px;
    border-bottom: 1px solid #dbdbdb;
    span{
      width: 30%;
      &:nth-child(1){
        width: 10%;
      }
      &:nth-child(2){
        width: 60%;
      }
    }
  }
}
</style>