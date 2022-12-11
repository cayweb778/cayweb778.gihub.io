import type { RouteRecordRaw } from 'vue-router';
import type { App } from 'vue';

import { createRouter, createWebHistory } from 'vue-router';

// app router
const router = createRouter({
    history: createWebHistory(import.meta.env.VITE_PUBLIC_PATH),
    routes: [
        {
            path: '/',
            name: 'hello',
            component: () => import('/@/views/Hello/index.vue'),
            meta: {
                title: 'hello',
            },
        },
        {
            path: '/firstPage',
            name: 'firstPage',
            component: () => import('/@/views/FirstPage/index.vue'),
            meta: {
                title: '配置服务地址',
            },
        },
        {
            path: '/config',
            name: 'config',
            component: () => import('/@/views/Config/index.vue'),
            meta: {
                title: '配置参数',
            },
        },
        {
            path: '/app',
            name: 'app',
            component: () => import('/@/views/App/index.vue'),
            meta: {
                title: '应用',
            },
        },
        {
            path: '/tuopan',
            name: 'tuopan',
            component: () => import('/@/views/tuopan/index.vue'),
            meta: {
                title: '托盘',
            },
        }
    ],
    strict: true,
    scrollBehavior: () => ({ left: 0, top: 0 }),
});


export default router;
