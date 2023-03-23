
<template>
  <div v-for="w in reviewWords" :key="w.id">
    <div>{{ w.word }}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from "vue";
import { useWordStore } from "../store/word";
import { storeToRefs } from "pinia";

export default defineComponent({
  setup() {
    const { reviewWords } = storeToRefs(useWordStore());

    onMounted(() => {
      useWordStore().getReviewWords();
    });

    return { reviewWords };
  },
});
</script>