
import { createRouter, createWebHashHistory } from 'vue-router'

import Home from './pages/Home/index.vue';
import Lobby from './pages/Lobby/index.vue';
import Room from "./pages/Room/index.vue";


// 创建 router
const router = createRouter({
  history: createWebHashHistory(),
  routes: [ // 定义路由
    {
      path: '/',
      redirect: '/lobby',
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
  ]
})

export default router;
