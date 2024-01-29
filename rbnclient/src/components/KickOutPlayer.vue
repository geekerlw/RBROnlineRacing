<template>
  <el-dialog title="踢人" v-model="dialogVisible" width="800px" fullscreen top="5vh" class="dlog">

  </el-dialog>
</template>
<script setup>
import { ref, reactive } from "vue";
const dialogVisible = ref(false);
const show = () => {
  dialogVisible.value = true;
  console.log("showCreateRace");
};
const hide = () => {
  dialogVisible.value = false;
};


defineExpose({
  show,
  hide,
});
</script>

