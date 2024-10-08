<script setup>
import { inject, ref, watch, computed } from 'vue'
import { VueFlip } from 'vue-flip'
import UseFetch from '../components/UseFetch.vue'

const useFetch = ref(null)

const fromLang = inject('fromLang')
const toLang = inject('toLang')

const fromWord = ref('')
const toWord = ref('')

const vocab = inject('vocab')
const host = inject('host')

const flipped = ref(false)

const reverseWordOrder = ref(false)

const wordCorrect = ref(true)

const wordCount = ref(0)
const totalWords = inject('totalWords')
const port = inject('serverPort')

const minCorrect = inject('minCorrect')
const minAge = inject('minAge')
const part = inject('partOfSpeech')
const partsOfSpeech = inject('partsOfSpeech')
let _minCorrect = ref(5)
let _minAge = ref(15)
let _part = ref(part.value)

const applyChanges = () => {
  minCorrect.value = _minCorrect.value
  minAge.value = _minAge.value
  part.value = _part.value
}

const changesPending = computed(() => {
  console.log(`part: ${_part.value}`)
  return (
    minCorrect.value !== _minCorrect.value ||
    minAge.value !== _minAge.value ||
    part.value !== _part.value
  )
})

const lastWordCount = ref(0)

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

const nextWord = async () => {
  if (lastWordCount.value >= totalWords.value) {
    return
  }
  if (wordCorrect.value) markCorrect()

  flipped.value = reverseWordOrder.value
  wordCorrect.value = true

  try {
    const res = await useFetch.value.fetch(
      `http://${host.value}:${port.value}/vocab/next_word`,
      'GET'
    )
    if ('Error' in res) {
      console.log(`Error calling next_word: ${res.Error}`)
      throw Error()
    } else {
      fromWord.value = res.text
      wordCount.value = res.count
      lastWordCount.value = res.count
      translate()
    }
  } catch (err) {
    console.log(`Fetch call failed`)
  }
}

const translate = () => {
  let translated = []
  const data = vocab.value
  if (!reverseWordOrder.value) {
    //fromWord is in fromLang, and therefore found in value.translations, and the translation is the key
    for (const entry of Object.entries(data)) {
      const key = entry[0]
      const val = entry[1]
      for (const word of val.translations) {
        if (word == fromWord.value) {
          translated.push(key)
          break
        }
      }
    }
  } else {
    //fromWord is in toLang, and therefore found in the keys, with translations in value.translations
    for (const entry of Object.entries(data)) {
      const key = entry[0]
      const val = entry[1]
      if (key == fromWord.value) {
        translated = val.translations.slice()
        break
      }
    }
  }
  toWord.value = translated.join(', ')
}

const markCorrect = async () => {
  await useFetch.value.fetch(
    `http://${host.value}:${port.value}/vocab/mark_correct`,
    'POST',
    {
      text: toWord.value,
    }
  )
}

const setWordOrder = async () => {
  const wordOrder = reverseWordOrder.value ? 'to-from' : 'from-to'
  await useFetch.value.fetch(
    `http://${host.value}:${port.value}/vocab/set_word_order`,
    'POST',
    {
      value: wordOrder,
    }
  )
  const temp = toWord.value
  toWord.value = fromWord.value
  fromWord.value = temp
}

watch(reverseWordOrder, setWordOrder)

// onMounted(() => {
//   nextWord()
// })

const options = computed(() => {
  return partsOfSpeech.value
})

const correctAction = computed(() => {
  return `Mark ${wordCorrect.value ? 'Incorrect' : 'Correct'}`
})
</script>

<template>
  <UseFetch ref="useFetch" />
  <div class="container">
    <div class="center-container">
      <div class="block">
        <legend>Word Selection Criteria</legend>
        <div class="row">
          <label class="left-label" for="minCorrect">Less than</label>
          <input
            type="number"
            id="minCorrect"
            value="5"
            v-model="_minCorrect"
          />
          <label class="right-label" for="minCorrect"
            >consecutive correct</label
          >
        </div>
        <div class="row">
          <label class="left-label" for="minAge">Last correct at least</label>
          <input type="number" id="minAge" value="15" v-model="_minAge" />
          <label class="right-label" for="minAge">days ago</label>
        </div>
        <div class="row">
          <label class="left-label" for="part">Part of Speech</label>
          <select id="part" v-model="_part">
            <option value="Any">Any</option>
            <option
              v-for="option in options"
              :value="option"
              totalWords
              v-bind:key="option"
            >
              {{ option }}
            </option>
          </select>
        </div>
        <div class="row">
          <button
            class="button"
            @click="applyChanges"
            :disabled="!changesPending"
          >
            Apply Changes
          </button>
        </div>
      </div>
      <div>
        <vue-flip
          v-model="flipped"
          width="700px"
          height="400px"
          class="card-container"
        >
          <template v-slot:front>
            <div v-if="!reverseWordOrder" class="card">
              <div class="card-title">Original</div>
              <div class="word">{{ fromWord }}</div>
              <div class="card-bottom-div">
                <font-awesome-icon
                  @click="
                    () => {
                      flipped = !flipped
                    }
                  "
                  class="flip-btn"
                  icon="fa-solid fa-rotate"
                />
                <div class="word-count">
                  {{ wordCount }} of {{ totalWords }}
                </div>
              </div>
            </div>
            <div v-else class="card">
              <div class="card-title">Translation</div>
              <div :class="wordCorrect ? 'word' : 'word red-font'">
                {{ toWord }}
              </div>
              <div class="card-bottom-div">
                <font-awesome-icon
                  @click="
                    () => {
                      flipped = !flipped
                    }
                  "
                  class="flip-btn"
                  icon="fa-solid fa-rotate"
                />
                <div class="word-count">
                  {{ wordCount }} of {{ totalWords }}
                </div>
                <div class="btn-wrapper">
                  <button
                    id="mark-btn"
                    @click="() => (wordCorrect = !wordCorrect)"
                  >
                    {{ correctAction }}
                  </button>
                </div>
              </div>
            </div>
          </template>
          <template v-slot:back>
            <div v-if="!reverseWordOrder" class="card">
              <div class="card-title">Translation</div>
              <div :class="wordCorrect ? 'word' : 'word red-font'">
                {{ toWord }}
              </div>
              <div class="card-bottom-div">
                <font-awesome-icon
                  @click="
                    () => {
                      flipped = !flipped
                    }
                  "
                  class="flip-btn"
                  icon="fa-solid fa-rotate"
                />
                <div class="word-count">
                  {{ wordCount }} of {{ totalWords }}
                </div>
                <div class="btn-wrapper">
                  <button
                    id="mark-btn"
                    @click="() => (wordCorrect = !wordCorrect)"
                  >
                    {{ correctAction }}
                  </button>
                </div>
              </div>
            </div>
            <div v-else class="card">
              <div class="card-title">Original</div>
              <div class="word">{{ fromWord }}</div>
              <div class="card-bottom-div">
                <font-awesome-icon
                  @click="
                    () => {
                      flipped = !flipped
                    }
                  "
                  class="flip-btn"
                  icon="fa-solid fa-rotate"
                />
                <div class="word-count">
                  {{ wordCount }} of {{ totalWords }}
                </div>
              </div>
            </div>
          </template>
        </vue-flip>
      </div>
    </div>
    <div class="bottom-div">
      <div class="word-order-div">
        <span
          >Present Words in:
          <input
            name="word-order"
            type="radio"
            :value="false"
            v-model="reverseWordOrder"
          />{{ fromLang.name }}
          <input
            name="word-order"
            type="radio"
            :value="true"
            v-model="reverseWordOrder"
          />{{ toLang.name }}
        </span>
      </div>
      <button @click="nextWord" class="btn">Next Word</button>
    </div>
  </div>
</template>

<style scoped>
.container {
  margin-top: 5%;
  display: flex;
  flex-direction: column;
}

.center-container {
  display: flex;
  flex-direction: row;
  align-items: flex-start;
  justify-content: center;
  gap: 15px;
}

.card-container {
  width: 60%;
}

.card-bottom-div {
  margin-left: 20%;
  width: 60%;
  margin-top: 0px;
  padding-left: 20px;
  padding-right: 20px;
  display: grid;
  align-items: center;
  grid-template-columns: 33% 33% 33%;
}

.bottom-div {
  margin-left: 5%;
  width: 100%;
  margin-top: 0px;
  padding-left: 20px;
  padding-right: 20px;
  display: flex;
  flex-direction: row;
  align-items: flex-end;
}

.btn {
  margin-left: 20px;
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
  background-color: rgb(250, 250, 250);
}

.block {
  width: 25%;
  margin-top: 10px;
  padding: 5px;
}

.row {
  margin-top: 15px;
}

.right-label {
  margin-left: 5px;
}

.left-label {
  margin-right: 5px;
}

.word-count {
  font-size: 1.5rem;
  color: gray;
}

.card-title {
  text-align: start;
  margin-top: 15px;
  margin-left: 15px;
  font-size: 1.5rem;
  color: gray;
}

.word {
  margin-top: 15%;
}

.front {
  margin-top: 20px;
  margin-bottom: 20px;
}

.button {
  border-radius: 3px;
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

#minAge {
  width: 37px;
}

#minCorrect {
  width: 30px;
}

legend {
  font-weight: 800;
}
</style>
