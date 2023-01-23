<script setup>
  import { ref, computed, provide, onMounted, watchEffect, watch } from 'vue';
  import { useFetch2 } from './components/fetch.js'
  import  VocabTableView from './views/VocabTableView.vue' ;
  import FlashCardView from './views/FlashCardView.vue';
  import SettingsView from './views/SettingsView.vue';
  import AvailableLanguagesView from './views/AvailableLanguagesView.vue';

  let currentTab = ref("Available Languages");

  const langs = ref({});
  const defLangs = ref({});
  const vocab = ref(null);
  const err = ref(null);
  const fromLang = ref({name: "", id: ""});
  const toLang = ref({name: "", id: ""});
  provide("langs", langs);
  provide("vocab", vocab);


  provide("fromLang", fromLang);
  provide("toLang", toLang);

  const compMap = {
    "Available Languages": AvailableLanguagesView,
    "Saved Vocabulary": VocabTableView,
    "Flash Cards": FlashCardView,
    "Settings": SettingsView
  }

  const tabs = Object.keys(compMap);
 
  const init = (fromLang, toLang) => {
    useFetch2(`http://localhost:5000/init?from_lang=${fromLang.value.id}&to_lang=${toLang.value.id}`, "GET")
    .then( ()=> {
      useFetch2('http://localhost:5000/languages/get', "GET")
      .then(res => {
        langs.value = res;
      })
      .then(() => {
        if (!Object.keys(defLangs.value).length) {
          useFetch2('http://localhost:5000/languages/get_defaults')
          .then((res) => {
            defLangs.value = res;
            useFetch2('http://localhost:5000/vocab/get_all')
            .then(res => {
              vocab.value = res;
            })
          })
        } else {
          useFetch2('http://localhost:5000/vocab/get_all')
          .then(res => {
            vocab.value = res;
          })
        }
      })
    })
    .catch(err => {
      console.log(`fetch returned an error: ${err}`);
    })
  }

  const currentTabComponent = computed(() => {
    return compMap[currentTab.value]
  })

  const setFromId = (name) => {
    let found = false;
    for (let id of Object.keys(langs.value)) {
      if (langs.value[id].name?.toLowerCase() === name.toLowerCase()) {
        fromLang.value.id = id;
        found = true;
        break;
      }
    }
    if (!found) {
      fromLang.value.id = "";
    }
}

provide("setFromIdFunc", setFromId)

const setToId = (name) => {
  let found = false;
  for (let id of Object.keys(langs.value)) {
    if (langs.value[id].name?.toLowerCase() === name.toLowerCase()) {
      toLang.value.id = id;
      found = true;
      break;
    }
  }
  if (!found) {
    toLang.value.id = "";
  }
}

provide("setToIdFunc", setToId)

const initialCap = (s) => {
  return s.charAt(0).toUpperCase() + s.slice(1);
}

provide('initialCap', initialCap);

watch(defLangs, (newVal) => {
  fromLang.value.name = initialCap(newVal.from);
  setFromId(newVal.from);
  toLang.value.name = initialCap(newVal.to);
  setToId(newVal.to);
});

onMounted(() => {
  watchEffect(async () => {
    init(fromLang, toLang);
  })
});
</script>

<template>
  <div id="top-level-app">
    <div class="top-nav">
      <button
        v-for="tab in tabs"
        v-bind:key="tab"
        v-bind:class="['tab-button', { active: currentTab === tab }]"
        v-on:click="currentTab = tab"
      >
        {{ tab }}
      </button>
    </div>

    <component v-bind:is="currentTabComponent" class="tab"></component>
  </div>
</template>

<style scoped>

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
  background: #e0e0e0;
}
.tab-button.active {
  background: #e0e0e0;
}
.tab {
  padding: 10px;
  background-color: rgb(240, 240, 240);
  width: 80%;
  margin-left: 10%;
}
#top-level-app {
  height: 80%;
  padding-top: 50px;
  padding-bottom: 50px;
  margin-left: 10%;
  width: 80%;
}

.top-nav {
  text-align: center;
}
</style>