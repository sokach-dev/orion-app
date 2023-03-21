import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import 'ant-design-vue/dist/antd.css';
import Antd from 'ant-design-vue';
import router from "./router";
import store from './store';

createApp(App).use(store).use(router).use(Antd).mount("#app");
