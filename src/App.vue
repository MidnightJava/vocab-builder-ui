<script setup>
import { ref, computed, provide, watch, watchEffect } from 'vue'
import UseFetch from './components/UseFetch.vue'
import VocabTableView from './views/VocabTableView.vue'
import FlashCardView from './views/FlashCardView.vue'
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
const partOfSpeech = ref('Any')
const partsOfSpeech = ref([])
const apiLookup = ref(true)
const totalWords = ref(0)
const host = ref('localhost')
const retries = ref(0)

provide('langs', langs)
provide('vocab', vocab)
provide('fromLang', fromLang)
provide('toLang', toLang)
provide('connected', connected)
provide('lookup', lookup)
provide('minCorrect', minCorrect)
provide('minAge', minAge)
provide('partOfSpeech', partOfSpeech)
provide('partsOfSpeech', partsOfSpeech)
provide('apiLookup', apiLookup)
provide('totalWords', totalWords)
provide('host', host)

const env = import.meta.env
/* VITE_SERVER_PORT is read directly by the frontend and
backend. The fontend also queries the backend for the value,
but this does not work in Windows. This superfluous query is 
left in place in case Windows supports the printenv command
some day.
*/
const portVal = env?.VITE_SERVER_PORT
const port = portVal ? ref(portVal) : ref(null)
provide('serverPort', port)

try {
  const Command = window.__TAURI__.shell.Command
  const readEnvVariable = async variableName => {
    const commandResult = await new Command('printenv', variableName).execute()
    if (commandResult.code !== 0) {
      return console.log(
        `Unable to get server port. Command returned ${commandResult.stderr}`
      )
    }

    port.value = commandResult.stdout
    console.log(`Set port to ${port.value}`)
  }
  readEnvVariable('VITE_SERVER_PORT')
} catch {
  //will fail outside tauri build
}

const compMap = {
  'Available Languages': AvailableLanguagesView,
  'Saved Vocabulary': VocabTableView,
  'Flash Cards': FlashCardView,
}

const tabs = Object.keys(compMap)

const selectWords = async () => {
  const res = await useFetch.value
    .fetch(`http://${host.value}:${port.value}/vocab/select_words`, 'GET')
    .catch(err => console.log(`Select words error: ${err.message}`))
  totalWords.value = Number(res?.Result)
}

const init = async (fromLang = fromLang.value, toLang = toLang.value) => {
  await useFetch.value
    .fetch(
      `http://${host.value}:${port.value}/init?from_lang=${fromLang.id}&to_lang=${toLang.id}&min_correct=${minCorrect.value}&min_age=${minAge.value}&part_of_speech=${partOfSpeech.value}`,
      'GET'
    )
    .catch(err => console.log(`Init error: ${err.message}`))
  let res = await useFetch.value
    .fetch(`http://${host.value}:${port.value}/languages/get`, 'GET')
    .catch(err => console.log(`Init error: ${err.message}`))
  langs.value = res
  if (!Object.keys(defLangs?.value || {})?.length) {
    res = await useFetch.value
      .fetch(`http://${host.value}:${port.value}/languages/get_defaults`)
      .catch(err => console.log(`Init error: ${err.message}`))
    defLangs.value = res
    res = await useFetch.value
      .fetch(`http://${host.value}:${port.value}/vocab/get_all`)
      .catch(err => console.log(`Init error: ${err.message}`))
    vocab.value = res
  } else {
    res = await useFetch.value
      .fetch(`http://${host.value}:${port.value}/vocab/get_all`)
      .catch(err => console.log(`Init error: ${err.message}`))
    vocab.value = res
  }
  selectWords()

  getPartsOfSpeech()
  currentTab.value = 'Available Languages'
}

const getPartsOfSpeech = async () => {
  const res = await useFetch.value
    .fetch(`http://${host.value}:${port.value}/partsofspeech/get`, 'GET')
    .catch(err => console.log(`Get parts of speech error: ${err.message}`))
  partsOfSpeech.value = res
}

provide('getPartsFunc', getPartsOfSpeech)

const currentTabComponent = computed(() => {
  return compMap[currentTab.value]
})

const setFromId = name => {
  let found = false
  for (let id of Object.keys(langs?.value || {})) {
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
  for (let id of Object.keys(langs?.value || {})) {
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
  return s?.charAt(0).toUpperCase() + s?.slice(1)
}

provide('initialCap', initialCap)

watch(defLangs, newVal => {
  fromLang.value.name = initialCap(newVal?.from)
  setFromId(newVal?.from)
  toLang.value.name = initialCap(newVal?.to)
  setToId(newVal?.to)
})

watch(connected, newVal => {
  if (newVal == false) {
    console.log('Disconnected')
  }
})

watch(apiLookup, async newVal => {
  await useFetch.value
    .fetch(`http://${host.value}:${port.value}/apilookup/set`, 'POST', {
      api_lookup: newVal,
    })
    .catch(err => console.log(`API lookup error: ${err.message}`))
})

watchEffect(async () => {
  if (port.value) {
    init(fromLang.value, toLang.value)
  }
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
            try {
              init(fromLang, toLang)
              retries += 1
            } catch (e) {
              console.log('Init failed')
            }
          }
        "
      >
        Reconnect
      </button>
    </div>
    <div v-if="retries > 1" class="retry-label">
      Check the application log. Perhaps the port is already in use. Either stop
      the process that is using the port identified in the log or set the
      environment variable SEVER_PORT to point to an available port.
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
.retry-label {
  margin-top: 20px;
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
  width: 90%;
}

.top-nav {
  text-align: center;
}
</style>
