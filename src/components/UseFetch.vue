<script setup>
import { inject } from 'vue'
const connected = inject('connected')
const useFetch = async (url, method = 'GET', payload = null) => {
  const opts = { method }
  if (payload) {
    opts.headers = {
      'Content-Type': 'application/json',
    }
    opts.body = JSON.stringify(payload)
  }
  const res = await fetch(url, opts)
  if (res.status != 200) {
    return res.statusText
  }
  const json = await res.json()
  if (!json || 'Error' in json) {
    connected.value = false
  } else {
    connected.value = true
  }

  return json
}

defineExpose({ fetch: useFetch })
</script>
<template><div></div></template>
