<script setup>
import { ref, computed, onMounted, inject } from 'vue'

import UseFetch from '../components/UseFetch.vue'

const useFetch = ref(null)

const apiKey = ref('Not Set')
const _apiKey = ref('Not Set')
const invalidKey = ref(false)

const apiLookup = inject('apiLookup')

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
  return apiKey.value !== _apiKey.value || invalidKey.value === true
})

const updateDisabled = computed(
  () => !apiKeyChangePending.value || !apiLookup.value
)

const apiLabelClass = computed(() => {
  return !apiLookup?.value ? 'disabled' : ''
})

onMounted(async () => {
  let res = await useFetch.value.fetch('http://localhost:5000/api_key', 'GET')
  if (!res) {
    invalidKey.value = true
  } else {
    _apiKey.value = apiKey.value = res?.result
    try {
      res = await useFetch.value.fetch(
        'http://localhost:5000/vocab/translate?from_lang=en&to_lang=it&word=test',
        'GET'
      )
      if (!res?.result) invalidKey.value = true
    } catch (e) {
      console.log(e.message)
    }
  }
})
</script>

<template>
  <UseFetch ref="useFetch" />
  <div class="block">
    <div class="row">
      <label>Translation Source</label>
      <input type="radio" id="autoLookup" :value="true" v-model="apiLookup" />
      <label for="autoLokkup">API Call</label>
      <input
        type="radio"
        id="manualLookup"
        :value="false"
        v-model="apiLookup"
      />
      <label for="manualLookup">Manual Entry</label>
    </div>
  </div>
  <div class="row">
    <label for="apiKey" :class="apiLabelClass" :disabled="!apiLookup"
      >API Key</label
    >
    <input type="text" id="apiKey" v-model="_apiKey" :disabled="!apiLookup" />
    <button class="button" @click="setApiKey" :disabled="updateDisabled">
      Update
    </button>
    <label class="warning" v-if="invalidKey">The API Key is invalid</label>
  </div>
  <div class="row">
    <button class="button">Import Vocabulary File</button>
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

.block {
  width: 35%;
  margin-top: 10px;
  padding: 5px;
}

input {
  margin-left: 5px;
  margin-right: 5px;
  margin-top: auto;
  margin-bottom: auto;
}

.disabled {
  color: gray;
}

input[type='text'] + label {
  float: left;
}
</style>
