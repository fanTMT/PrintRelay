import { createRouter, createWebHistory, type RouteRecordRaw } from "vue-router";



const routes:Array<RouteRecordRaw> = [
    {
        path:"/",
        component:()=>import('../components/History.vue')
    },
    {
        path:"/login",
        component:()=>import('../components/Login.vue')
    }
]
const router = createRouter({
    history:createWebHistory(),
    routes 
});


export default router;