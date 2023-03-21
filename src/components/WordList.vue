
<template>
  <div v-for="w in reviewWords" :key="w">
    <div>{{ w }}</div>
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
      useWordStore().init();
    });

    return { reviewWords };
  },
});
</script>