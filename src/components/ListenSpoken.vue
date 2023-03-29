<template>
  <div>Listen Spoken</div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from "vue";
import { useWordStore } from "../store/word";

export default defineComponent({
  setup() {
    onMounted(() => {
      // useWordStore().queryDict("hello");
    });
    return {};
  },
});
</script>