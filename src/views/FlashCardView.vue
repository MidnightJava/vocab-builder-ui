<script setup>

import { inject, ref, computed } from 'vue';
import { VueFlip } from 'vue-flip';

const fromLang = inject("fromLang");
const toLang = inject("toLang")

const slot1 = ref('front');
const slot2 = ref('back');

const flipped = ref(false);

const flipCommand = computed( () => {
  if (flipped.value) {
    return `Back to ${fromLang.value.name} Word`
  } else  {
    return `Show ${toLang.value.name} Translation`
  }
})
</script>

<template>
  <div>
    <h1>Vocabulary Flashcard test</h1>
    <vue-flip
      class="container"
      v-model="flipped"
      width="600px"
      height="400px"
    >
      <template v-slot:[slot1]>
        {{ fromLang.name }} Word
      </template>
      <template v-slot:[slot2]>
        {{ toLang.name }} Word
      </template>
    </vue-flip>
    <div>
      <button @click="() => {flipped = !flipped}">{{ flipCommand }}</button>
    </div>
  </div>
</template>

<style scoped>
  .container {
    border-style: solid;
    border-width: 2px;
    background-color: yellow;
  }
</style>
