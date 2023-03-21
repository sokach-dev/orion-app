
import { createWebHistory, createRouter } from "vue-router";

import Home from "../components/Home.vue";
import Word from "../components/Word.vue";
import Story from "../components/Story.vue";
import ListenSpoken from "../components/ListenSpoken.vue";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            name: "Home",
            component: Home,
            children: [
                {
                    path: "word",
                    name: "word",
                    component: Word,
                },
                {
                    path: "story",
                    name: "story",
                    component: Story,
                },
                {
                    path: "listen_spoken",
                    name: "listen_spoken",
                    component: ListenSpoken,
                },
            ]
        }]
});

export default router;