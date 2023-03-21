<template>
  <div class="dialogue-container">
    <div class="dialogue">
      <div v-for="d in wordDialogs" :key="d.id" :class="get_dialogue(d)">
        <p>{{ d.content }}</p>
      </div>
    </div>
    <div class="input-box">
      <input type="text" placeholder="Type your message here" />
      <button>Send</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from "vue";
import { useWordStore } from "../store/word";
import { storeToRefs } from "pinia";

export default defineComponent({
  setup() {
    const { wordDialogs } = storeToRefs(useWordStore());

    onMounted(() => {
      useWordStore().init();
    });

    const get_dialogue = (dialogue: any) => {
      if (dialogue.person == "orion") {
        return "speaker2";
      } else {
        return "speaker1";
      }
    };

    return { wordDialogs, get_dialogue };
  },
});
</script>


<style lang="sass" scoped>
.dialogue-container
    width: 100vh
    margin: auto

.dialogue
    background-color: #f2f2f2
    border-radius: 5px
    padding: 10px
    margin-bottom: 20px

.speaker1
    background-color: #e6ffb3
    padding: 10px
    border-radius: 5px
    margin-bottom: 5px
    text-align: right

.speaker2
    background-color: #b3e6ff
    padding: 10px
    border-radius: 5px
    margin-bottom: 5px
    text-align: right
    text-align: left

.input-box
    position: fixed
    bottom: 0
    padding-bottom: 10px
    padding-left: 10px

    & input[type="text"]
        flex: 1
        padding: 10px
        border-radius: 5px 0 0 5px
        border: none

    & button
        padding: 10px
        border-radius: 0 5px 5px 0
        border: none
        background-color: #4CAF50
        color: white
        cursor: pointer
</style>