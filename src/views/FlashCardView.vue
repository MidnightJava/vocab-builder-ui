<script setup>

import { inject, ref, computed, watch } from 'vue';
import { VueFlip } from 'vue-flip';
import { useFetch } from "../components/fetch.js";

const fromLang = inject("fromLang");
const toLang = inject("toLang")
const started = ref(false);

const fromWord = ref("");
const toWord = ref("");

const vocab = inject("vocab");

const slot1 = ref('front');
const slot2 = ref('back');

const flipped = ref(false);

const reverseWordOrder = ref(false);

/**
 * TODO:
 * Handle either word order.
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

const nextWordErr = ref();
const nextWordRes = ref()
const nextWord = () => {
  flipped.value = false;
  useFetch("http://localhost:5000/vocab/next_word", nextWordRes, nextWordErr, "GET", null, () => {
    if (nextWordErr.value) {
      console.log(`Next Word Error: ${nextWordErr.value}`);
    } else {
      fromWord.value = nextWordRes.value.result;
      translate();
    }
  })
}

const translate = () => {
  const translated = [];
  const data = vocab.value;
  if (!reverseWordOrder.value) {
    //fromWord is in fromLang, and therefore found in value.translations, and the translation is the key
    for (const entry of Object.entries(data)) {
      const key = entry[0];
      const val = entry[1];
      for (const word of val.translations) {
        if (word == fromWord.value) {
          translated.push(key);
          break;
        }
      }
    }
  } else {
    //fromWord is in toLang, and therefore found in the keys, with translations in value.translations
    for (const key in Object.keys(data)) {
      if (key == fromWord.value) {
        translated = data[key].translations.slice();
        break;
      }
    }
  }
  toWord.value = translated.join(",");
}

const selErr = ref();
const selectWords = () => {
  useFetch("http://localhost:5000/vocab/select_words", ref(), selErr, "GET", null, () => {
    if (selErr.value) {
      console.log(`Select Words Error: ${selErr.value}`);
    } else {
      nextWord();
    }
  })
}

const setWordOrderErr = ref();

const setWordOrder = callback => {
 const wordOrder = reverseWordOrder.value ? "to-from" : "from-to";
  useFetch("http://localhost:5000/vocab/set_word_order", ref(), setWordOrderErr, "POST", {"value": wordOrder}, callback);
}

const startButtonText = computed(() => {
  return started.value ? "Next Word" : "Start Vocab Test";
})

watch(reverseWordOrder, newVal => {
  console.log(`Reverse is ${newVal}`);
  setWordOrder(() => {
    if (newVal == true) {
    [slot1.value, slot2.value] = ['front', 'back'];
  } else {
    [slot1.value, slot2.value] = ['back', 'front'];
  }
  });
 
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
      <div v-if="started">
        <span id="word-order-grp">Present Words in: 
          <input name="word-order" type="radio" :value="false" v-model="reverseWordOrder" />{{ fromLang.name }}
          <input name="word-order" type="radio" :value="true" v-model="reverseWordOrder"  />{{ toLang.name }}
        </span>
      </div>
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

  #word-order-grp {
    margin-top: 5px;
  }

</style>
