<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { useI18n } from "vue-i18n";
import { useGlobalStore } from './store/index';
const gobalStore = useGlobalStore()
const { locale } = useI18n();
const changeLanguage = (lang) => {
  locale.value = lang;
  gobalStore.setLanguage(lang)
};
</script>
<template>
  <div class="maincontent">
    <div class="setting-lg">
      <el-dropdown trigger="click">
        <span class="el-dropdown-link">
          {{ $t(gobalStore.language) }}
          <el-icon class="el-icon--right">
            <arrow-down />
          </el-icon>
        </span>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item @click="changeLanguage('en')"
              >english</el-dropdown-item
            >
            <el-dropdown-item @click="changeLanguage('zh')"
              >中文</el-dropdown-item
            >
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
    <router-view />
  </div>
</template>

<style lang="less">
.setting-lg {
  position: absolute;
  z-index: 10;
  right: 20px;
  top: 20px;
  cursor: default;
  display: block;
  .el-dropdown-link {
    outline: none;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100px;
    height: 28px;
    cursor: pointer;
  }
}
.maincontent {
  position: relative;
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
  padding-top: 30px;
  box-sizing: border-box;
  display: flex;
  justify-content: center;
  text-align: center;
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
