<script setup>
import { inject, ref, computed } from 'vue'
const lookup = inject('lookup')
const minCorrect = inject('minCorrect')
const minAge = inject('minAge')
let _lookup = ref(false)
let _minCorrect = ref(5)
let _minAge = ref(15)

const applyChanges = () => {
  lookup.value - _lookup.value
  minCorrect.value = _minCorrect.value
  minAge.value = _minAge.value
}

const changesPending = computed(() => {
  return (
    lookup.value !== _lookup.value ||
    minCorrect.value !== _minCorrect.value ||
    minAge.value !== _minAge.value
  )
})
</script>

<template>
  <fieldset class="block">
    <legend>Add Vocabulary Settings</legend>
    <div class="row">
      <label>Translation Lookup</label>
      <input
        type="radio"
        id="autoLookup"
        checked
        value="false"
        v-model="lookup"
      />
      <label for="autoLokkup">Auto</label>
      <input type="radio" id="manualLookup" value="true" v-model="_lookup" />
      <label for="manualLookup">Manual</label>
    </div>
  </fieldset>
  <fieldset class="block">
    <legend>Flash Card Test Settings</legend>
    <div class="row">
      <input type="number" id="minCorrect" value="5" v-model="_minCorrect" />
      <label class="label" for="minCorrect">Min Correct</label>
    </div>
    <div class="row">
      <input type="number" id="minAge" value="15" v-model="_minAge" />
      <label class="label" for="minAge">Min Age (days)</label>
    </div>
  </fieldset>
  <div class="row">
    <button class="button" @click="applyChanges" :disabled="!changesPending">
      Apply Changes
    </button>
  </div>
  <div class="row">
    <button class="button">Import Vocabulary File</button>
  </div>
</template>

<style scoped>
.row {
  margin-top: 15px;
}

.label {
  margin-left: 5px;
}

.block {
  width: 30%;
  margin-top: 10px;
  border: 1px solid black;
}

.button {
  border-radius: 3px;
}
</style>
