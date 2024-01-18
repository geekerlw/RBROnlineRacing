
import { createRouter, createWebHashHistory } from 'vue-router'

import Home from './pages/Home.vue';
import Lobby from './pages/Lobby.vue';
import Room from "./pages/Room.vue";


// 创建 router
const router = createRouter({
  history: createWebHashHistory(),
  routes: [ // 定义路由
    {
      path: '/',
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
