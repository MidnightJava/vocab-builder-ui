<script setup>
  import { ref, computed, provide } from 'vue';
  import { useFetch } from './components/fetch.js'
  import  VocabTableView from './views/VocabTableView.vue' ;
  import FlashCardView from './views/FlashCardView.vue';
  import SettingsView from './views/SettingsView.vue';
  import AvailableLanguagesView from './views/AvailableLanguagesView.vue';

  let currentTab = ref("Available Languages");

  const compMap = {
    "Available Languages": AvailableLanguagesView,
    "Saved Vocabulary": VocabTableView,
    "Flash Cards": FlashCardView,
    "Settings": SettingsView
  }

  const tabs = Object.keys(compMap);
  const { data } = useFetch('http://localhost:5000/languages')
  provide("langs", data)

  const currentTabComponent = computed(() => {
    return compMap[currentTab.value]
  })
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
  border: 1px solid #ccc;
  padding: 10px;
}
</style>