<!--
┌──────┬───────────────┬─────────┐
│ wrong│               │right    │
│ 0分  │  apple        │  4分     │
├──────┴───────────────┴─────────┤
│         paraphrase   1分       │
└────────────────────────────────┘
-->


<template>
  <div v-for="w in reviewWords" :key="w.id">
    <div class="one">
      <div class="wrong">忘了</div>
      <div class="word_item">{{ w.word }}</div>
      <div class="right">记忆</div>
    </div>
    <div
      class="two"
      @mousedown="clickMouseDown(w)"
      @mouseup="clickMouseUp"
      @dblclick="dbClickParaphrase(w)"
    >
      <div v-if="w.show_paraphrase">{{ w.paraphrase }} "双击学习"</div>
      <div v-else>长按看释义</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from "vue";
import { useWordStore } from "../store/word";
import { storeToRefs } from "pinia";
import { getCustomDate } from "../utils";

export default defineComponent({
  setup() {
    const { reviewWords } = storeToRefs(useWordStore());
    var longPressTimer: any = null;

    // 长按显示释义
    const clickMouseDown = (w: any) => {
      longPressTimer = setTimeout(() => {
        useWordStore().changeShowParaphrase(w.id);
      }, 1000);
    };
    const clickMouseUp = () => {
      clearTimeout(longPressTimer);
    };

    // 双击评分为1
    const dbClickParaphrase = (w: any) => {
      useWordStore().learnWord(w, 1);
    };

    onMounted(() => {
      useWordStore().getReviewWords(getCustomDate(0));
    });

    return {
      reviewWords,
      clickMouseDown,
      clickMouseUp,
      dbClickParaphrase,
    };
  },
});
</script>

<style lang="scss" scoped>
.one {
  display: flex;
  justify-content: space-between;
  .wrong {
    width: 20%;
    text-align: center;
    font-size: xx-large;
    color: #844;
    // border-right: 1px solid black;
    box-shadow: 0 0 6px #444;
  }
  .word_item {
    padding: 10px;
    font-size: x-large;
  }

  .right {
    width: 20%;
    float: right;
    text-align: center;
    font-size: xx-large;
    color: green;
    // border-left: 1px solid black;
    box-shadow: 0 0 6px #444;
  }
}
.two {
  background-color: #ddd;
  border-bottom: 1px solid black;
  text-align: center;
  padding: 30px;
  user-select: none;
  -webkit-user-select: none;
}
</style>