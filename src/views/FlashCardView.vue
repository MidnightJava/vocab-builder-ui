<script setup>

import { inject, ref, watch, onMounted, computed } from 'vue';
import { VueFlip } from 'vue-flip';
import { useFetch } from "../components/fetch.js";

const fromLang = inject("fromLang");
const toLang = inject("toLang")

const fromWord = ref("");
const toWord = ref("");

const vocab = inject("vocab");

const flipped = ref(false);

const reverseWordOrder = ref(false);

const wordCorrect = ref(true);

const wordCount = ref(0);
const totalWords = ref(0)

/**
 * TODO:
 * Handle either word order. DONE
 * Set word red when marked incorrect DONE
 * Unselect words after they're deleted DONE
 * Fnish porting useFecth to useFetch2 DONE
 * Implement server-side data for vocab table
 * Call server to mark word correct or incorrect and last correct time
 * Handle end of word list. Instruct to start again instead of using refresh word list button
 * Show word number
 * Enhance card style and animation DONE
 * BugFix: preseve started state when switching tabs
 */

const nextWord = () => {

  flipped.value = reverseWordOrder.value;
  wordCorrect.value = true;

  useFetch("http://localhost:5000/vocab/next_word", "GET")
  .then(res => {
    fromWord.value = res.text;
    wordCount.value = res.count;
    totalWords.value = res.size;
    translate();
  })
  .catch(err => {
    console.log(`Fetch returned error: ${err}`);
  })
}

const translate = () => {
  let translated = [];
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
    for (const entry of Object.entries(data)) {
      const key = entry[0];
      const val = entry[1];
      if (key == fromWord.value) {
        translated = val.translations.slice();
        break;
      }
    }
  }
  toWord.value = translated.join(",");
}

const selectWords = () => {
  useFetch("http://localhost:5000/vocab/select_words", "GET")
  .then(() => nextWord())
  .catch(err => {
    console.log(`Fetch returned error: ${err}`);
  })
}

const setWordOrder = () => {
 const wordOrder = reverseWordOrder.value ? "to-from" : "from-to";
  useFetch("http://localhost:5000/vocab/set_word_order", "POST", {"value": wordOrder})
  .then(() => nextWord())
  .catch(err => {
    console.log(`Fetch returned error: ${err}`);
  })
}

watch(reverseWordOrder, setWordOrder);
  
onMounted( () => {
  selectWords()
})

const correctAction = computed(() => {
  return `Mark ${wordCorrect.value ? "Incorrect" : "Correct"}`;
})
</script>

<template>
  <div>
    <vue-flip
      v-model="flipped"
      width="700px"
      height="400px"
      class="container"
    >
      <template v-slot:front>
        <div v-if="!reverseWordOrder" class="card">
          <div class="card-title">Original</div>
          <div class="word"> {{ fromWord }}</div>
          <div class="btn-div">
            <font-awesome-icon  @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
            <div class="word-count">{{ wordCount }} of {{ totalWords }}</div>
          </div>
        </div>
        <div v-else class="card">
          <div class="card-title">Translation</div>
          <div :class="wordCorrect ? 'word': 'word red-font'"> {{ toWord }}</div>
          <div class="btn-div">
            <font-awesome-icon   @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
            <div class="word-count">{{ wordCount }} of {{ totalWords }}</div>
            <div class="btn-wrapper"><button id="mark-btn" @click="() => wordCorrect = !wordCorrect">{{ correctAction }}</button></div>
          </div>
        </div>
      </template>
      <template v-slot:back>
        <div v-if="!reverseWordOrder" class="card">
          <div class="card-title">Translation</div>
          <div :class="wordCorrect ? 'word': 'word red-font'"> {{ toWord }}</div>
          <div class="btn-div">
            <font-awesome-icon   @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
            <div class="word-count">{{ wordCount }} of {{ totalWords }}</div>
            <div class="btn-wrapper"><button id="mark-btn" @click="() => wordCorrect = !wordCorrect">{{ correctAction }}</button></div>
          </div>
        </div>
        <div v-else class="card">
          <div class="card-title">Original</div>
          <div class="word"> {{ fromWord }}</div>
          <div class="btn-div">
            <font-awesome-icon  @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
            <div class="word-count">{{ wordCount }} of {{ totalWords }}</div>
          </div>
        </div>
      </template>
    </vue-flip>
    <div class="btn-div">
      <div class="word-order-div">
        <span>Present Words in: 
          <input name="word-order" type="radio" :value="false" v-model="reverseWordOrder" />{{ fromLang.name }}
          <input name="word-order" type="radio" :value="true" v-model="reverseWordOrder"  />{{ toLang.name }}
        </span>
        <button @click="nextWord">Next Word</button>
      </div>
    </div>
  </div>
</template>

<style scoped>

.container {
  margin-left: 20%;
  width: 60%;
}

  .btn-div {
    margin-top: 0px;
    padding-left: 20px;
    padding-right: 20px;
    display: grid;
    align-items: center;
    grid-template-columns: 33% 33% 33%;
  }

  .btn-wrapper {
    display: grid;
    align-items: center;
  }

  .card {
    display: grid;
    grid-template-rows: 10% 70% auto;
    margin-top: 20px;
    text-align: center;
    font-size: 3.2rem;
    border-style: solid;
    border-width: 2px;
    height: 90%;
    margin-left: 10%;
    width: 80%;
    background-color: rgb(250, 250, 250);
  }

  .word-count {
    font-size: 1.5rem;
    color: gray;
  }

  .card-title {
    text-align: start;
    margin-top: 15px;
    margin-left:15px;
    font-size: 1.5rem;
    color: gray;
  }

  .word {
    margin-top: 15%;
  }

  .front {
    margin-top:20px;
    margin-bottom: 20px;
  }

  button {
    margin-right: 5px;
    margin-left: 10px;
    padding: 5px;
    background-color: white;
  }

  .flip-btn {
    float: left;
    cursor: pointer;
    position: relative;
    top: 105;
  }

  .word-order-div {
    margin-top: 10px;
  }

  .red-font {
    color: red;
    
  }

</style>
