<script setup>
import { ref, computed, provide, onMounted, watchEffect, watch } from 'vue'
import UseFetch from './components/UseFetch.vue'
import VocabTableView from './views/VocabTableView.vue'
import FlashCardView from './views/FlashCardView.vue'
import SettingsView from './views/SettingsView.vue'
import AvailableLanguagesView from './views/AvailableLanguagesView.vue'

let currentTab = ref('Available Languages')

const useFetch = ref(null)
const langs = ref({})
const defLangs = ref({})
const vocab = ref(null)
const fromLang = ref({ name: '', id: '' })
const toLang = ref({ name: '', id: '' })
const connected = ref(false)
const lookup = ref(false)
const minCorrect = ref(5)
const minAge = ref(15)
const apiLookup = ref(true)
const totalWords = ref(0)

provide('langs', langs)
provide('vocab', vocab)
provide('fromLang', fromLang)
provide('toLang', toLang)
provide('connected', connected)
provide('lookup', lookup)
provide('minCorrect', minCorrect)
provide('minAge', minAge)
provide('apiLookup', apiLookup)
provide('totalWords', totalWords)

const compMap = {
  'Available Languages': AvailableLanguagesView,
  'Saved Vocabulary': VocabTableView,
  'Flash Cards': FlashCardView,
  Settings: SettingsView,
}

const tabs = Object.keys(compMap)

const selectWords = async () => {
  const res = await useFetch.value.fetch(
    'http://localhost:5000/vocab/select_words',
    'GET'
  )
  totalWords.value = Number(res.Result)
}

const init = async (fromLang, toLang) => {
  await useFetch.value.fetch(
    `http://localhost:5000/init?from_lang=${fromLang.id}&to_lang=${toLang.id}&min_correct=${minCorrect.value}&min_age=${minAge.value}`,
    'GET'
  )
  let res = await useFetch.value.fetch(
    'http://localhost:5000/languages/get',
    'GET'
  )
  langs.value = res
  if (!Object.keys(defLangs.value).length) {
    res = await useFetch.value.fetch(
      'http://localhost:5000/languages/get_defaults'
    )
    defLangs.value = res
    res = await useFetch.value.fetch('http://localhost:5000/vocab/get_all')
    vocab.value = res
  } else {
    res = await useFetch.value.fetch('http://localhost:5000/vocab/get_all')
    vocab.value = res
  }
  selectWords()
  currentTab.value = 'Available Languages'
}

const currentTabComponent = computed(() => {
  return compMap[currentTab.value]
})

const setFromId = name => {
  let found = false
  for (let id of Object.keys(langs.value)) {
    if (langs.value[id].name?.toLowerCase() === name.toLowerCase()) {
      fromLang.value.id = id
      found = true
      break
    }
  }
  if (!found) {
    fromLang.value.id = ''
  }
}

provide('setFromIdFunc', setFromId)

const setToId = name => {
  let found = false
  for (let id of Object.keys(langs.value)) {
    if (langs.value[id].name?.toLowerCase() === name.toLowerCase()) {
      toLang.value.id = id
      found = true
      break
    }
  }
  if (!found) {
    toLang.value.id = ''
  }
}

provide('setToIdFunc', setToId)

const initialCap = s => {
  return s.charAt(0).toUpperCase() + s.slice(1)
}

provide('initialCap', initialCap)

watch(defLangs, newVal => {
  fromLang.value.name = initialCap(newVal.from)
  setFromId(newVal.from)
  toLang.value.name = initialCap(newVal.to)
  setToId(newVal.to)
})

watch(connected, newVal => {
  if (newVal == false) {
    console.log('Disconnected')
  }
})

watch(apiLookup, async newVal => {
  await useFetch.value.fetch('http://localhost:5000/apilookup/set', 'POST', {
    api_lookup: newVal,
  })
})

onMounted(() => {
  watchEffect(async () => {
    init(fromLang.value, toLang.value)
  })
})
</script>

<template>
  <UseFetch ref="useFetch" />
  <div id="top-level-app">
    <div class="top-nav">
      <button
        v-for="tab in tabs"
        v-bind:key="tab"
        v-bind:class="['tab-button', { active: currentTab === tab }]"
        @click="() => (currentTab = tab)"
      >
        {{ tab }}
      </button>
    </div>
    <KeepAlive>
      <component v-bind:is="currentTabComponent"></component>
    </KeepAlive>
    <div class="reconnect-panel" v-if="!connected">
      <div class="reconnect-label">Lost server connection</div>
      <button
        class="reconnect-button"
        @click="
          () => {
            init(fromLang, toLang)
          }
        "
      >
        Reconnect
      </button>
    </div>
  </div>
</template>

<style scoped>
.reconnect-panel {
  display: flex;
  flex-direction: row;
  margin-top: 10px;
}
.reconnect-button {
  margin-left: 20px;
  margin-top: auto;
  border-radius: 3px;
  color: blue;
}
.reconnect-label {
  margin-top: auto;
  color: red;
}
.tab-button {
  padding: 6px 10px;
  border-top-left-radius: 3px;
  border-top-right-radius: 3px;
  border: 1px solid #ccc;
  cursor: pointer;
  background: #f0f0f0;
  margin-bottom: -1px;
  margin-right: -1px;
}
.tab-button:hover {
  background: #fff;
}
.tab-button.active {
  background: lightblue;
}
.tab {
  padding: 10px;
  background-color: rgb(240, 240, 240);
  width: 100%;
  margin: auto;
}
#top-level-app {
  height: 80%;
  padding-top: 50px;
  padding-bottom: 50px;
  margin: auto;
  width: 80%;
}

.top-nav {
  text-align: center;
}
</style>
