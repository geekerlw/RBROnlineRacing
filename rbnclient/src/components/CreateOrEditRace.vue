<template>
  <!-- 利用element-plus的弹窗组件实现 -->
  <el-dialog
    :title="editRaceId ? '修改比赛' : '创建房间'"
    v-model="dialogVisible"
    width="800px"
    fullscreen
    top="5vh"
    class="dlog"
  >
    <el-form :model="form" :rules="rules" label-width="100px">
      <div class="group-title">房间设定</div>
      <el-form-item label="房间名" prop="name">
        <el-input v-model="form.name"></el-input>
      </el-form-item>
      <el-form-item label="房间密码" prop="passwd">
        <el-input v-model="form.passwd"></el-input>
      </el-form-item>

      <div class="group-title">比赛设定</div>
      <el-form-item label="比赛赛道" prop="stage" class="multitem">
        <el-select v-model="form.stage" placeholder="请选择地图" filterable @change="changeStage">
          <el-option
            v-for="item in stageListOptions"
            :key="item.id"
            :label="item.name"
            :value="item.id"
          ></el-option>
        </el-select>
        <el-button class="line-btn" @click="randomStage" type="primary"
          >随机一下</el-button
        >
      </el-form-item>
      <!-- 车辆选择 select 支持搜索 -->
      <el-form-item label="车辆选择" prop="car" class="multitem">
        <el-select v-model="form.car" placeholder="请选择车辆" filterable>
          <el-option
            v-for="item in carListOptions"
            :key="item.id"
            :label="item.name"
            :value="item.id"
          ></el-option>
        </el-select>
        <el-checkbox v-model="form.car_fixed">
          <span slot="label">限定车辆</span>
        </el-checkbox>
      </el-form-item>
      <!-- 车辆损坏 select -->
      <el-form-item label="车辆损坏" prop="damage">
        <el-select v-model="form.damage" placeholder="请选择车辆损坏">
          <el-option
            v-for="item in damageListOptions"
            :key="item.id"
            :label="item.value"
            :value="item.id"
          ></el-option>
        </el-select>
      </el-form-item>
      <div class="group-title">条件设定</div>
      <el-form-item label="天气类型" prop="skytype">
        <el-select v-model="form.skytype" :placeholder="sktTypePlaceholder">
          <el-option
            v-for="item in skytypeListOptions"
            :key="item.id"
            :label="item.value"
            :value="item.id"
          ></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="湿滑情况" prop="wetness">
        <el-select v-model="form.wetness" placeholder="">
          <el-option
            v-for="item in wetnessListOptions"
            :key="item.id"
            :label="item.value"
            :value="item.id"
          ></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="天气状况" prop="weather">
        <el-select v-model="form.weather" placeholder="">
          <el-option
            v-for="item in weatherListOptions"
            :key="item.value"
            :label="item.value"
            :value="item.id"
          ></el-option>
        </el-select>
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="createHandle"> 确认 </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, reactive, onMounted, computed } from "vue";
import { ElMessage } from "element-plus";
import {
  load_game_stage_skytype_options,
} from "../reados/index.js";
import { createRace } from "../api/index.js"
import { useGameConfig } from "../store/index.js";

const {
  stageListOptions,
  carListOptions,
  damageListOptions,
  wetnessListOptions,
  weatherListOptions,
} = useGameConfig();

// emit created
const emit = defineEmits(['created'])

const dialogVisible = ref(false);

const skytypeListOptions = ref([]);


const sktTypePlaceholder = computed(() => {
  if (!form.stage) {
    return "请先选择赛道";
  }
  if (skytypeListOptions.value.length === 0) {
    return "该赛道无天气类型";
  }
  return "请选择天气类型";
});

onMounted(() => {
  console.log("onMounted");
});

const changeStage = (val) => {
  load_game_stage_skytype_options(Number(val)).then((res) => {
    if (res) {
      console.log("skytypeListOptions", res);
      skytypeListOptions.value = JSON.parse(res);
      console.log("skytypeListOptions", skytypeListOptions.value);
    }
  });
};

// 判断是否是编辑模式
const editRaceId = ref("");

const showCreateRace = (editRace) => {
  if (editRace) {
    editRaceId.value = editRace.id;
  }
  dialogVisible.value = true;
  console.log("showCreateRace");
};
const hideCreateRace = () => {
  dialogVisible.value = false;
};

const rules = [];
const resetdata = {
  name: "",
  passwd: "",
  stage: "",
  car: "",
  car_fixed: false,
  damage: null,
  wetness: null,
  weather: null,
  skytype: null,
};
const form = reactive(JSON.parse(JSON.stringify(resetdata)));
const restData = () => {
  Object.assign(form, resetdata);
  editRaceId.value = "";
};

const randomStage = () => {
  const index = Math.floor(Math.random() * stageListOptions.length);
  form.stage = stageListOptions[index].id;
};

const createHandle = () => {
  if (!form.name) {
    return ElMessage.error("请输入房间名");
  }
  // if (!form.passwd) {
  //   return ElMessage.error("请输入房间密码");
  // }
  // 判断stage
  if (!form.stage) {
    return ElMessage.error("请选择赛道");
  }
  const stage = stageListOptions.find((item) => item.id === form.stage);
  let stageType = "";
  // stage.snow = '100', stage.tarmac = '0', stage.gravel = '0', then stageType = 'snow'
  console.log(stage)
  if (stage.snow > stage.tarmac && stage.snow > stage.gravel) {
    stageType = "snow";
  } else if (stage.tarmac > stage.snow && stage.tarmac > stage.gravel) {
    stageType = "tarmac";
  } else if (stage.gravel > stage.snow && stage.gravel > stage.tarmac) {
    stageType = "gravel";
  }
  if (!form.car) {
    return ElMessage.error("请选择车辆");
  }
  const car = carListOptions.find((item) => item.id === form.car);
  if (form.damage == null) {
    return ElMessage.error("请选择车辆损坏");
  }
  let skyType = null;
  if (skytypeListOptions.value.length != 0 && form.skytype == null) {
    return ElMessage.error("请选择天气类型");
  } else {
    skyType = skytypeListOptions.value.find((item) => item.id === form.skytype);
  }
  if (form.wetness == null) {
    return ElMessage.error("请选择湿滑情况");
  }
  if (form.weather == null) {
    return ElMessage.error("请选择天气状况");
  }
  const data = {
    info: {
      name: form.name,
      owner: "",
      stage: stage.name,
      stage_id: Number(stage.id),
      stage_type: stageType,
      stage_len: Number(stage.length),
      car_fixed: form.car_fixed,
      car: car.name,
      car_id: Number(car.id),
      damage: Number(form.damage),
      weather: form.weather,
      wetness: form.wetness,
    },
    locked: false
  };
  if (form.passwd) {
    data.passwd = form.passwd;
    data.locked = true;
  }
  if (skyType) {
    data.info.skytype = skyType.value;
    data.info.skytype_id = Number(skyType.id);
  } else {
    data.info.skytype = "Default";
    data.info.skytype_id = 0;
  }
  console.log("createHandle", data);
  createRace(data).then((res) => {
    if (res) {
      ElMessage.success("创建成功");
      hideCreateRace();
      restData();
      emit('created');
    }
  });
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

.multitem :deep(.el-form-item__content) {
  display: flex;
  justify-content: space-between;
}

.multitem :deep(.el-select) {
  width: calc(100% - 110px);
}

:global(.el-dialog__body) {
  padding-top: 0;
  padding-bottom: 0px;
}

// .line-btn {
//   margin-top: 10px;
// }
</style>
