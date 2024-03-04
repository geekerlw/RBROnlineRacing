
import { createRouter, createWebHashHistory } from 'vue-router'

import Home from './pages/Home/index.vue';
import Lobby from './pages/Lobby/index.vue';
import Room from "./pages/Room/index.vue";
import Test from "./pages/Test/index.vue";


// 创建 router
const router = createRouter({
  history: createWebHashHistory(),
  routes: [ // 定义路由
    {
      path: '/',
      redirect: '/home',
    },
    {
      path: '/home',
      component: Home,
    }, 
    {
      path: "/lobby",
      component: Lobby
    },
    {
      path: "/room/:roomId",
      component: Room,
    },
    {
      path: "/test",
      component: Test,
    }
  ]
})

export default router;
