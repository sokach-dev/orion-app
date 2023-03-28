<template>
  <div class="dialogue-container">
    <div class="dialogue" @scroll="onScroll">
      <div v-for="d in wordDialogs" :key="d.id" :class="get_dialogue(d)">
        <pre>{{ d.id }} -- {{ d.content }}</pre>
      </div>
    </div>
    <div class="input-box">
      <InputBox />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from "vue";
import { useWordStore } from "../store/word";
import { storeToRefs } from "pinia";
import InputBox from "./InputBox.vue";

export default defineComponent({
  components: {
    InputBox,
  },
  setup() {
    const { wordDialogs } = storeToRefs(useWordStore());
    var isLoading = false;

    onMounted(() => {
      loading();
    });

    const loading = async () => {
      if (!isLoading) {
        isLoading = true;

        setTimeout(() => {
          useWordStore().loadDialogs();
          isLoading = false;
        }, 1000);
      }
    };

    const onScroll = (e: any) => {
      const wrapper = document.querySelector(".dialogue");
      if (wrapper) {
        if (e.target.scrollTop < -40) {
          loading();
        }
      } else {
        console.log("wrapper is null");
      }
    };

    const get_dialogue = (dialogue: any) => {
      if (dialogue.person == "me") {
        return "speaker_right";
      } else {
        return "speaker_left";
      }
    };

    return { wordDialogs, get_dialogue, onScroll };
  },
});
</script>


<style lang="scss" scoped>
.dialogue-container {
  margin: auto;
}

.dialogue {
  background-color: #f2f2f2;
  border-radius: 5px;
  padding: 10px;
  margin-bottom: 50px;
  // padding-bottom: 50px;

  height: 100vh;
  overflow-y: auto;
}
.speaker_right {
  background-color: #e6ffb3;
  padding: 10px;
  border-radius: 5px;
  margin-bottom: 5px;
  text-align: right;
}

.speaker_left {
  background-color: #b3e6ff;
  padding: 10px;
  border-radius: 5px;
  margin-bottom: 5px;
  text-align: left;
}

.input-box {
  position: fixed;
  bottom: 0;
  padding-bottom: 10px;
  padding-left: 50px;
}
</style>