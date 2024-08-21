<script setup>
import { inject, computed, ref, watch } from 'vue'
import PartsTable from 'vue3-easy-data-table'
import UseFetch from '../components/UseFetch.vue'

const useFetch = ref(null)

const dataTable = ref()

const headers = [
  { text: 'Name', value: 'name', width: 150, sortable: true },
  { text: 'Delete', value: 'update', sortable: false },
]

const showOpt = ref(0)

const parts = inject('partsOfSpeech')
const data = ref(parts.value)
const showAddItem = ref(false)
const newEntryInp = ref('')
const dupEntry = ref(false)
const getPartsFunc = inject('getPartsFunc')
const newEntryInpElem = ref('')
const host = inject('host')
const port = inject('serverPort')

watch(
  () => parts.value,
  newv => {
    data.value = newv.slice()
  }
)

watch(
  () => newEntryInp.value,
  newv => {
    if (data.value.map(v => v.toLowerCase()).includes(newv.toLowerCase())) {
      dupEntry.value = true
    } else {
      dupEntry.value = false
    }
  }
)

const getItems = computed(() => {
  if (data.value) {
    const _data = data.value || []
    let cnt = 1
    const items = _data.map(part => ({
      idx: cnt++,
      name: part,
    }))
    return items
  } else {
    return []
  }
})

const entryClass = computed(() => {
  return dupEntry.value ? 'red  new-entry-inp' : 'new-entry-inp'
})

const deleteEntry = async item => {
  data.value = data.value.filter(x => x !== item.name)
}

const applyChanges = async () => {
  await useFetch.value.fetch(
    `http://${host.value}:${port.value}/partsofspeech/set`,
    'POST',
    { partsOfSpeech: data.value }
  )
  getPartsFunc()
}

const changesPending = computed(() => {
  const _parts = new Set(parts.value)
  const _data = new Set(data.value)
  const diff = _parts.symmetricDifference(_data)
  return diff.size > 0
})

const addItem = () => {
  showAddItem.value = true
  setTimeout(() => newEntryInpElem.value.focus())
}

const doAddItem = () => {
  data.value.push(newEntryInp.value)
  showAddItem.value = false
  newEntryInp.value = ''
}

const cancelAddItem = () => {
  showAddItem.value = false
  newEntryInp.value = ''
}
</script>

<template>
  <UseFetch ref="useFetch" />
  <label>Assign parts of speech to words for use as a flash card filter</label>
  <div class="float-container">
    <div class="float-child">
      <PartsTable
        ref="dataTable"
        :headers="headers"
        :items="getItems"
        alternating
        border-cell
        hide-footer
      >
        <template #item-update="item">
          <div
            class="button-container"
            @mouseleave="() => (showOpt = 0)"
            @mouseenter="() => (showOpt = item.idx)"
          >
            <span v-if="showOpt === item.idx && item.idx !== 0">
              <button @click="() => deleteEntry(item)">Delete</button>
            </span>
            <button class="button hide" v-else></button>
          </div>
        </template>
      </PartsTable>
      <div class="row">
        <button
          class="button"
          @click="applyChanges"
          :disabled="!changesPending"
        >
          Apply Changes
        </button>
        <button class="button" @click="addItem">Add Entry</button>
      </div>
      <div class="float-child2" v-if="showAddItem">
        <div class="new-entry-container">
          <label for="new-entry-inp">Enter Name:</label>
          <input
            v-model="newEntryInp"
            id="new-entry-inp"
            :class="entryClass"
            type="text"
            ref="newEntryInpElem"
          />
          <button id="add-ok-cancel" :disabled="dupEntry" @click="doAddItem">
            OK
          </button>
          <button id="add-ok-cancel" @click="cancelAddItem">Cancel</button>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped>
.float-container {
  border: 3px solid #fff;
  margin-top: 5px;
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  padding: 0;
}

.button {
  margin-right: 10px;
}

.float-child {
  width: 300px;
  height: 100%;
  float: left;
  padding: 0;
  margin: 0;
}

.float-child2 {
  width: 440px;
  height: 100%;
  float: left;
  padding: 0;
  margin: 0;
}

.row {
  margin-top: 15px;
}

.hide {
  border: none;
  background-color: inherit;
}
.button-container {
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
}

.new-entry-inp {
  margin-left: 5px;
  margin-top: 5px;
  width: 40%;
}

#add-ok-cancel {
  margin: 0;
}

.red {
  color: red;
}

.new-entry-container {
  margin-top: 10px;
  width: 100%;
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 5px;
}
</style>
