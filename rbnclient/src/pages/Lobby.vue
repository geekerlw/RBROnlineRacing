<template>
  <div class="lobby">
    <div>
        <button @click="back()">上一页</button>
    </div>
        大厅：～～～～
        <div>
            roomlist
            <div v-for="room in dataList" :key="room">
                <router-link :to="'/room/' + room">{{room}}</router-link>
            </div>
        </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";

const back = () => {
    history.back()
}

const dataList = ref([]);
onMounted(() => {
    dataList.value = ['room1', 'room2', 'room3']
})

</script>

<style>

</style>