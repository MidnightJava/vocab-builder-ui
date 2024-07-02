<script setup>
import { ref } from 'vue'

import UseFetch from '../components/UseFetch.vue'

const useFetch = ref(null)
</script>

<template>
  <UseFetch ref="useFetch" />
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
</style>
