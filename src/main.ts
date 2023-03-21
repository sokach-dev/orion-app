import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import 'ant-design-vue/dist/antd.css';
import Antd from 'ant-design-vue';
import router from "./router";
import { createPinia } from "pinia";

const pinia = createPinia();

createApp(App).use(pinia).use(router).use(Antd).mount("#app");
