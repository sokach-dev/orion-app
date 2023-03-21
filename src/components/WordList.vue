
<template>
  <div v-for="w in wordList" :key="w">
    <div>{{ w }}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

export default defineComponent({
  setup() {
    const wordList = ref<string[]>([
      "apple",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "banana",
      "cat",
      "dog",
      "egg",
    ]);
    return { wordList };
  },
});
</script>