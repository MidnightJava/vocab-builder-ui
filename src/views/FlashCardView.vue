<script setup>

import { inject, ref, computed } from 'vue';
import { VueFlip } from 'vue-flip';
import { useFetch } from "../components/fetch.js";

const fromLang = inject("fromLang");
const toLang = inject("toLang")
const started = ref(false);

const fromWord = ref("");
const toWord = ref("");

const slot1 = ref('front');
const slot2 = ref('back');

const flipped = ref(false);

/**
 * TODO:
 * Set word red when marked incorrect
 * Call server to mark word correct or incorrect and last correct time
 * Handle end of word list. Instruct to start again instead of using refresh word list button
 * BugFix: preseve started state when switching tabs
 */

const runTest = () => {
  if (started.value == false) {
    started.value = true;
    selectWords();
  } else {
    nextWord();
  }
  
}

const err = ref();
const res = ref()
const nextWord = () => {
  flipped.value = false;
  useFetch("http://localhost:5000/vocab/next_word", res, err, "GET", null, () => {
    if (err.value) {
      console.log(`Next Word Error: ${err.value}`);
    } else {
      fromWord.value = res.value.result;
      translate();
    }
  })
}

const translate = () => {
  useFetch(`http://localhost:5000/vocab/translate?from_lang=${fromLang.value.id}&to_lang=${toLang.value.id}&word=${fromWord.value}`, res, err, "GET", null, () => {
    if (err.value) {
      console.log(`Translate Error: ${err.value}`);
    } else {
      if ('result' in res.value) {
        toWord.value = res.value.result;
      }
    }
  })
}

const selectWords = () => {
  useFetch("http://localhost:5000/vocab/select_words", ref(), err, "GET", null, () => {
    if (err.value) {
      console.log(`Select Words Error: ${err.value}`);
    } else {
      nextWord();
    }
  })
}

const startButtonText = computed(() => {
  return started.value ? "Next Word" : "Start Vocab Test";
})
</script>

<template>
  <div>
    <vue-flip
      v-model="flipped"
      width="600px"
      height="400px"
      v-if="started"
    >
      <template v-slot:[slot1]>
        <div class="card">
          <div> {{ fromWord }}</div>
          <div class="btn-div">
            <font-awesome-icon  @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
          </div>
        </div>
      </template>
      <template v-slot:[slot2]>
        <div class="card">
          <div> {{ toWord }}</div>
          <div class="btn-div">
            <font-awesome-icon   @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
            <button id="mark-btn">Mark Incorrect</button>
          </div>
        </div>
      </template>
    </vue-flip>
    <div class="btn-div">
      <button @click="runTest">{{ startButtonText }}</button>
      <button v-if="started">Refresh Word List</button>
    </div>
  </div>
</template>

<style scoped>

  .btn-div {
    margin-top: 0px;
    padding-left: 20px;
    padding-right: 20px;
  }


  .card {
    display: grid;
    grid-template-rows: 20% 80%;
    margin-top: 10px;
    padding-top: 20%;
    text-align: center;
    font-size: 3.0rem;
    border-style: solid;
    border-width: 2px;
    height: 50%;
  }

  .front {
    margin-top:20px;
    margin-bottom: 20px;
  }

  button {
    margin-right: 5px;
  }

  .flip-btn {
    float: left;
    margin-top: 15%;
    cursor: pointer;
  }

  #mark-btn {
    float: right;
    margin-top: 20%;
  }

</style>
