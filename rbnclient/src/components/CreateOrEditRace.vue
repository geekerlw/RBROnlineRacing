<template>
  <!-- 利用element-plus的弹窗组件实现 -->
  <el-dialog title="创建房间" v-model="dialogVisible" width="800px" fullscreen="true" top="5vh" class="dlog"  modal="false">
    <el-form :model="form" :rules="rules" label-width="80px">
      <div class="group-title">房间设定</div>
      <el-form-item label="房间名" prop="name">
        <el-input v-model="form.name"></el-input>
      </el-form-item>
      <el-form-item label="房间密码" prop="password">
        <el-input v-model="form.password"></el-input>
      </el-form-item>

      <div class="group-title">比赛设定</div>
      <el-form-item label="比赛赛道" prop="stage" class="multitem">
        <el-select v-model="form.stage" placeholder="请选择地图" filterable>
          <el-option
            v-for="item in stageList"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          ></el-option>
        </el-select>
        <el-button class="line-btn" @click="randomStage" type="primary"
          >随机一下</el-button
        >
      </el-form-item>
      <!-- 车辆损坏 select -->
      <el-form-item label="车辆损坏" prop="damage">
        <el-select v-model="form.damage" placeholder="请选择车辆损坏">
          <el-option
            v-for="item in damageList"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          ></el-option>
        </el-select>
      </el-form-item>
      <!-- 车辆选择 select 支持搜索 -->
      <el-form-item label="车辆选择" prop="car" class="multitem">
        <el-select v-model="form.car" placeholder="请选择车辆" filterable>
          <el-option
            v-for="item in carList"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          ></el-option>
        </el-select>
        <el-checkbox>
          <span slot="label">限定车辆</span>
        </el-checkbox>
      </el-form-item>

      <div class="group-title">条件设定</div>
      <!-- 路面情况 -->
      <el-form-item label="路面情况" prop="road">
        <el-select v-model="form.road" placeholder="">
          <el-option
            v-for="item in roadList"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          ></el-option>
        </el-select>
      </el-form-item>
      <!-- 湿滑情况选择 -->
      <el-form-item label="湿滑情况" prop="湿滑情况">
        <el-select v-model="form.road" placeholder="">
          <el-option
            v-for="item in roadList"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          ></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="比赛时段" prop="路面情况">
        <el-select v-model="form.road" placeholder="">
          <el-option
            v-for="item in roadList"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          ></el-option>
        </el-select>
      </el-form-item>      
      <el-form-item label="比赛时段" prop="路面情况">
        <el-select v-model="form.road" placeholder="">
          <el-option
            v-for="item in roadList"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          ></el-option>
        </el-select>
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary"> 确认 </el-button>
      </span>
    </template>
  </el-dialog>
</template>
<script setup>
import { ref, reactive } from "vue";
const dialogVisible = ref(false);
const showCreateRace = () => {
  dialogVisible.value = true;
  console.log("showCreateRace");
};
const hideCreateRace = () => {
  dialogVisible.value = false;
};

// 判断是否是编辑模式
const editRaceId = ref("");

const stageList = [
  {
    label: "hy",
    value: "hy",
  },
  {
    label: "com",
    value: "com",
  },
];
const damageList = [
  {
    label: "real",
    value: "real",
  },
  {
    label: "sim",
    value: "sim",
  },
];
const carList = [
  {
    label: "car1",
    value: "car1",
  },
  {
    label: "car2",
    value: "car2",
  },
];
const roadList = [
  {
    label: "road1",
    value: "road1",
  },
  {
    label: "road2",
    value: "road2",
  },
];

const rules = [];
const form = reactive({
  name: "11",
  stage: "",
  damage: "",
  car: "",
  road: '',
});

const randomStage = () => {
  const index = Math.floor(Math.random() * stageList.length);
  form.stage = stageList[index].value;
};

defineExpose({
  showCreateRace,
  hideCreateRace,
});
</script>
<style lang="less" scoped>
.group-title {
  margin: 20px 0;
  padding-left: 20px;
  position: relative;
  font-weight: bold;
  text-align: left;

  &::before {
    content: "";
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    height: 1px;
    width: 10px;
    background-color: #409eff; //更改为你想要的颜色
  }
}
.multitem :deep(.el-form-item__content){
  display: flex;
  justify-content: space-between;
}
.multitem :deep(.el-select){
  width: calc(100% - 110px);
}
:global(.el-dialog__body){
  padding-top: 0;
  padding-bottom: 0px;
}
// .line-btn {
//   margin-top: 10px;
// }
</style>
