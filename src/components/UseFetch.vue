<script setup>
import { inject } from 'vue'
const connected = inject('connected')
const initialConnection = inject('initialConnection')
const useFetch = async (url, method = 'GET', payload = null) => {
  const opts = { method }
  if (payload) {
    opts.headers = {
      'Content-Type': 'application/json',
    }
    opts.body = JSON.stringify(payload)
  }
  try {
    const res = await fetch(url, opts)

    if (res.status != 200) {
      connected.value = false //Added May 4.2025 make sure it doesn't break something
      return res.statusText
    }
    const json = await res.json()
    if (!json || 'Error' in json) {
      connected.value = false
    } else {
      connected.value = true
      initialConnection.value = true
    }
    return json
  } catch {
    connected.value = false
  }
}

const useFetch2 = async (url, method = 'GET', payload = null) => {
  const opts = { method }
  if (payload) {
    opts.headers = {
      'Content-Type': 'multipart/form-data',
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

defineExpose({ fetch: useFetch, fetchFile: useFetch2 })
</script>
<template><div></div></template>
