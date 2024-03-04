<template>
    <el-dialog v-model="props.visible" title="配置游戏安装根目录" :fullscreen="true" :show-close="false">
        <el-form ref="form" :model="form" label-width="80px">
            <div class="tips">
                请输入RBR安装根目录，例如：C:\Program Files (x86)\RBR
            </div>
            <div class="block30"></div>
            <el-input v-model="gamePath" placeholder="请输入RBR安装根目录,  建议在资源管理器里复制"></el-input>
            <!-- 确定 -->
            <div class="block30"></div>
            <el-button type="primary" size="large" @click="handleConfirm">&nbsp;&nbsp;配置&nbsp;&nbsp;</el-button>
        </el-form>
    </el-dialog>
</template>

<script setup>
import { ref, reactive, defineProps, defineEmits } from "vue";
const props = defineProps(["visible"]);
const gamePath = ref('');
const emit = defineEmits(['finish'])

const handleConfirm = () => {
    console.log('gamePath: ', gamePath.value);
    if (gamePath.value) {
        emit('finish', gamePath.value);
    }
}
</script>