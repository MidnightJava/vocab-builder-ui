<script setup>
  import { ref, computed, provide, onMounted, watchEffect } from 'vue';
  import { useFetch } from './components/fetch.js'
  import  VocabTableView from './views/VocabTableView.vue' ;
  import FlashCardView from './views/FlashCardView.vue';
  import SettingsView from './views/SettingsView.vue';
  import AvailableLanguagesView from './views/AvailableLanguagesView.vue';

  let currentTab = ref("Available Languages");

  const langs = ref(null);
  const vocab = ref(null);
  const err = ref(null);
  const fromLang = ref("");
  const toLang = ref("");
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
    useFetch(`http://localhost:5000/init?from_lang=${fromLang.value}&to_lang=${toLang.value}`, ref(null), ref(null), "GET", null, () => {
      useFetch('http://localhost:5000/languages/get', langs, err);
      useFetch('http://localhost:5000/vocab/get_all', vocab, err);
    })
  }

  const currentTabComponent = computed(() => {
    return compMap[currentTab.value]
  })

  onMounted(() => {
    watchEffect(async () => {
      init(fromLang, toLang);
    })
  });
</script>

<template>
  <div id="dynamic-component-demo" class="demo">
      <button
        v-for="tab in tabs"
        v-bind:key="tab"
        v-bind:class="['tab-button', { active: currentTab === tab }]"
        v-on:click="currentTab = tab"
      >
        {{ tab }}
      </button>

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
}
.demo {
  height: 80%;
  /* width: 50%; */
  padding-top: 50px;
  padding-bottom: 50px;
  margin-left: 25%;
  /* margin-right: auto; */
}
</style>