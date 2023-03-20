<template>
  <a-layout style="min-height: 100vh">
    <a-layout-sider v-model:collapsed="collapsed" collapsible>
      <div class="logo" />
      <a-menu v-model:selectedKeys="selectedKeys" theme="dark" mode="inline">
        <a-menu-item key="1">
          <pie-chart-outlined />
          <span>统计</span>
        </a-menu-item>
        <a-sub-menu key="sub1">
          <template #title>
            <span>
              <user-outlined />
              <span>单词</span>
            </span>
          </template>
          <a-menu-item key="3">复习</a-menu-item>
          <a-menu-item key="4">四级</a-menu-item>
          <a-menu-item key="5">六级</a-menu-item>
        </a-sub-menu>
        <a-sub-menu key="sub2">
          <template #title>
            <span>
              <team-outlined />
              <span>故事</span>
            </span>
          </template>
          <a-menu-item key="6">Openai故事</a-menu-item>
          <a-menu-item key="8">自己编故事</a-menu-item>
        </a-sub-menu>
        <a-menu-item key="9">
          <audio-outlined />
          <span>听说</span>
        </a-menu-item>
      </a-menu>
    </a-layout-sider>
    <word-list />
    <a-layout>
      <a-layout-header style="background: #fff; padding: 0" />
      <a-layout-content style="margin: 0 16px">
        <div
          :style="{ padding: '24px', background: '#fff', minHeight: '360px' }"
        >
          Bill is a cat.
        </div>
      </a-layout-content>
      <a-layout-footer style="text-align: center">
        Ant Design ©2018 Created by Ant UED
      </a-layout-footer>
    </a-layout>
  </a-layout>
</template>

<script lang="ts">
import {
  PieChartOutlined,
  DesktopOutlined,
  UserOutlined,
  TeamOutlined,
  AudioOutlined,
} from "@ant-design/icons-vue";

import { defineComponent, ref } from "vue";

import WordList from "./WordList.vue";

export default defineComponent({
  components: {
    PieChartOutlined,
    DesktopOutlined,
    UserOutlined,
    TeamOutlined,
    AudioOutlined,
    WordList,
  },

  setup() {
    const collapsed = ref<boolean>(false);
    const selectedKeys = ref<string[]>(["1"]);

    return {
      collapsed,
      selectedKeys,
    };
  },
});
</script>

<style>
#components-layout-demo-side .logo {
  height: 32px;
  margin: 16px;
  background: rgba(255, 255, 255, 0.3);
}

.site-layout .site-layout-background {
  background: #fff;
}
[data-theme="dark"] .site-layout .site-layout-background {
  background: #141414;
}
</style>
