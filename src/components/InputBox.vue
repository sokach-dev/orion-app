<template>
  <div class="wrapbox">
    <div class="mention">
      <a-mentions
        rows="2"
        v-model:value="content"
        placeholder="input / to mention order"
        :prefix="['/']"
      >
        <!-- @search="onSearch" -->
        <a-mentions-option
          v-for="opt in options"
          :key="opt.order"
          :value="opt.order.substring(1)"
        >
          {{ opt.order }} - {{ opt.commit }}
        </a-mentions-option>
      </a-mentions>
    </div>
    <div class="submit">
      <a-button type="text" size="large" @click="handleSubmit">
        <RocketOutlined :style="{ fontSize: '33px', color: 'gray' }" />
      </a-button>
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import { RocketOutlined } from "@ant-design/icons-vue";
import { message } from "ant-design-vue";

const MOCK_DATA: Record<string, any[]> = {
  "/": [
    { order: "/nd", commit: "添加新单词" },
    { order: "/qd", commit: "查询单词" },
    { order: "/sd", commit: "开始复习" },
    { order: "/dd", commit: "删除单词" },
  ],
};
export default defineComponent({
  components: {
    RocketOutlined,
  },
  setup() {
    const prefix = ref<string>("/");
    const content = ref<string>("");
    const options = computed(() => {
      return MOCK_DATA[prefix.value] || [];
    });

    const handleSubmit = () => {
      if (content.value.startsWith("/")) {
        const order = content.value.slice(0, 3); // get order
        const word = content.value.slice(4); // get word

        if (word.length === 0) {
          message.error("请输入信息");
          return;
        }

        var orderOk = false;

        MOCK_DATA["/"].forEach((item) => {
          if (item.order === order) {
            orderOk = true;
          }
        });

        if (!orderOk) {
          message.error("无法识别的指令");
          return;
        }
        // go to order
      } else {
        // TODO
        // record content
      }

      content.value = "";
    };

    return {
      content,
      options,
      handleSubmit,
    };
  },
});
</script>

<style lang="scss" scoped>
.wrapbox {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  width: 500px;
  height: 100%;
  .mention {
    width: 80%;
    box-shadow: 0 0 10px #999;
  }
  .submit {
    width: 20%;
  }
}
</style>
  
  