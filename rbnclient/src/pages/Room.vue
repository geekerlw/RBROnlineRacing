<template>
    <div class="room">
        <div>
            <button @click="back()">上一页</button>
        </div>
        房间：～～～～
    </div>
</template>

<script setup>
const back = () => {
    history.back()
}
</script>

<style>

</style>