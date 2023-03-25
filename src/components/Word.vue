<template>
  <div class="word">
    <div class="word-list">
      <WordList />
    </div>
    <div class="dialogue">
      <Dialogue />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import WordList from "./WordList.vue";
import Dialogue from "./Dialogue.vue";

export default defineComponent({
  components: {
    WordList,
    Dialogue,
  },
  setup() {
    return {};
  },
});
</script>


<style lang="scss" scoped>
.word {
  display: flex;
  height: 100vh;
  width: 100%;
  background-color: #f0f2e5;
}
.word-list {
  height: 100vh;
  width: 40%;
  overflow-y: scroll;
}
.dialogue {
  overflow-y: scroll;
  height: 100vh;
  width: 60%;
  background-color: #f0f2e5;
}
</style>