<template>
  <div class="word">
    <div class="word-list">
      <WordList />
    </div>
    <div>
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


<style lang="sass" scoped>
.word
    display: flex
    height: 100vh
    width: 100vw
    background-color: #f0f2e5
.word-list
    height: 100vh
    width: 15vw
    background-color: #f0f2a5
    overflow-y: scroll
</style>