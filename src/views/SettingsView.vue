<script setup>
import { ref, computed, onMounted } from 'vue'

import UseFetch from '../components/UseFetch.vue'

const useFetch = ref(null)

const apiKey = ref('Not Set')
const _apiKey = ref('Not Set')
const invalidKey = ref(false)

/* TODO:
Restart the server after setting a valid API key (i.e. it gets saved in the .env file).
Otherwise, when the user reloads the page, the server will instantiate a new translator
client with the API key originally set in the environment. */
const setApiKey = async () => {
  const res = await useFetch.value.fetch(
    'http://localhost:5000/api_key/set',
    'POST',
    { api_key: _apiKey.value }
  )
  if (res.result !== _apiKey.value) {
    invalidKey.value = true
  } else {
    invalidKey.value = false
    apiKey.value = _apiKey.value
  }
}

const apiKeyChangePending = computed(() => {
  return apiKey.value !== _apiKey.value
})

onMounted(async () => {
  const res = await useFetch.value.fetch('http://localhost:5000/api_key', 'GET')
  if (!res) {
    invalidKey.value = true
  } else {
    _apiKey.value = apiKey.value = res?.result
  }
})
</script>

<template>
  <UseFetch ref="useFetch" />
  <div class="row">
    <button class="button">Import Vocabulary File</button>
  </div>
  <div class="row">
    <label for="apiKey">API Key</label>
    <input type="text" id="apiKey" v-model="_apiKey" />
    <button class="button" @click="setApiKey" :disabled="!apiKeyChangePending">
      Update
    </button>
    <label class="warning" v-if="invalidKey">The API Key is invalid</label>
  </div>
</template>

<style scoped>
.row {
  display: flex;
  flex-direction: row;
  margin-top: 10px;
  align-items: center;
}
.button {
  border-radius: 3px;
  margin-top: auto;
  margin-bottom: auto;
}

label {
  margin-top: auto;
  margin-bottom: auto;
}

.warning {
  color: red;
}

input {
  margin-left: 5px;
  margin-right: 5px;
  margin-top: auto;
  margin-bottom: auto;
}
</style>
