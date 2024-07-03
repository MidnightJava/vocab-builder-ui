<script setup>
import { inject, computed, ref, provide, onMounted, watch } from 'vue'
import { usePagination } from 'use-vue3-easy-data-table'

import UseFetch from '../components/UseFetch.vue'
import VocabTable from 'vue3-easy-data-table'
import AddVocab from './AddVocab.vue'

const apiKey = ref('Not Set')
const _apiKey = ref('Not Set')
const invalidKey = ref(false)
const csvImportSelected = ref(0)
const csvExportSelected = ref(0)
const jsonImportSelected = ref(0)
const jsonExportSelected = ref(0)

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

const useFetch = ref(null)

const dataTable = ref()

let role = ref('add')
provide('role', role)

const rowsPerPage = 15

const currentPageNumber = ref(1)

let show = ref(false)
provide('show', show)

let fromWord = ref('')
provide('fromWord', fromWord)
let toWord = ref('')
provide('toWord', toWord)

const {
  currentPageFirstIndex,
  currentPageLastIndex,
  clientItemsLength,
  maxPaginationNumber,
  isFirstPage,
  isLastPage,
  nextPage,
  prevPage,
  updatePage,
} = usePagination(dataTable)

const fromLang = inject('fromLang')
const toLang = inject('toLang')
const vocab = inject('vocab')

const headers = [
  {
    text: fromLang.value.name,
    value: fromLang.value.name?.toLowerCase(),
    width: 200,
    sortable: true,
  },
  {
    text: toLang.value.name,
    value: toLang.value.name?.toLowerCase(),
    width: 200,
    sortable: true,
  },
  { text: 'Add/Update', value: 'opts', width: 165, sortable: false },
]

const getItems = computed(() => {
  if (vocab) {
    const data = vocab.value || {}
    let count = 1
    const items = Object.entries(data).map(entry => {
      return {
        id: count++,
        [toLang.value?.name?.toLowerCase()]: entry[0],
        [fromLang.value?.name?.toLowerCase()]: entry[1].translations,
        opts: false,
      }
    })
    for (let i = items.length; i % 15 != 0; i++) {
      items.push({
        id: 0,
        [toLang.value?.name?.toLowerCase()]: '',
        [fromLang.value?.name?.toLowerCase()]: [],
        opts: false,
      })
    }
    return items
  } else {
    return []
  }
})

const searchValue = ref('')
const searchField = ref([
  toLang.value?.name?.toLowerCase(),
  fromLang.value?.name?.toLowerCase(),
])
const itemsSelected = ref([])
const showOpt = ref(0)

const deleteSelected = async () => {
  const toDelete = []

  for (const item of itemsSelected.value) {
    toDelete.push({ key: item[toLang.value?.name?.toLowerCase()] })
  }

  await useFetch.value.fetch(
    'http://localhost:5000/vocab/delete_entry',
    'POST',
    toDelete
  )
  const res = await useFetch('http://localhost:5000/vocab/get_all')
  vocab.value = res
  itemsSelected.value = []
  vocab.value = res
}

const deleteEntry = async item => {
  await useFetch.value.fetch(
    'http://localhost:5000/vocab/delete_entry',
    'POST',
    {
      key: item[toLang.value.name?.toLowerCase()],
    }
  )
  const res = await useFetch.value.fetch('http://localhost:5000/vocab/get_all')
  vocab.value = res
}

const editEntry = item => {
  role.value = 'edit'
  toWord.value = item[toLang.value.name.toLowerCase()]
  fromWord.value = item[fromLang.value.name.toLowerCase()].join(',')
  show.value = true
}

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

const csvImportFileSelect = e => {
  csvImportSelected.value = e.target.files.length
}

const jsonImportFileSelect = e => {
  jsonImportSelected.value = e.target.files.length
}

const downloadFile = (data, type) => {
  // Replace all '\n' characters with '\r\n' to ensure correct line breaks in CSV
  const csvData = type === 'csv' ? data.replace(/\n/g, '\r\n') : data
  const blob = new Blob([csvData], { type: `text/${type}` })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'download.csv'
  a.click()
}

const exportCsvFile = async () => {
  const res = await useFetch.value.fetch(
    'http://localhost:5000/vocab/export_csv',
    'GET'
  )
  const data = res.file
  downloadFile(data, 'csv')
}

const exportJsonFile = async () => {
  const res = await useFetch.value.fetch(
    'http://localhost:5000/vocab/export_json',
    'GET'
  )
  const data = res.file
  downloadFile(data, 'json')
}
</script>

<template>
  <UseFetch ref="useFetch" />
  <div class="float-container">
    <div class="float-child">
      <VocabTable
        ref="dataTable"
        :headers="headers"
        :items="getItems"
        v-model:items-selected="itemsSelected"
        :rows-per-page="rowsPerPage"
        :search-value="searchValue"
        :search-field="searchField"
        alternating
        border-cell
        hide-footer
      >
        <template #item-opts="item">
          <div
            @mouseout="() => (showOpt = 0)"
            @mouseover="() => (showOpt = item.id)"
          >
            <span v-if="showOpt === item.id && item.id !== 0">
              <button class="del-button" @click="() => editEntry(item)">
                Edit
              </button>
              <button class="del-button" @click="() => deleteEntry(item)">
                Delete
              </button>
            </span>
            <button class="del-button hide" v-else></button>
          </div>
        </template>
        <template #header-opts>
          <font-awesome-icon class="float-right" icon="fa-solid fa-ellipsis" />
        </template>
      </VocabTable>
      <div class="customize-footer">
        <div class="customize-index">
          Now displaying entries {{ currentPageFirstIndex }} ~
          {{ currentPageLastIndex }} of {{ clientItemsLength }}
        </div>

        <span class="customize-page-input">
          Go to Page
          <input
            type="number"
            min="1"
            :max="maxPaginationNumber"
            v-model="currentPageNumber"
            @change="
              () => {
                updatePage(currentPageNumber)
              }
            "
          />
          of {{ Math.ceil(clientItemsLength / rowsPerPage) }}
        </span>

        <div class="customize-pagination">
          <button
            class="prev-page"
            @click="
              () => {
                currentPageNumber--
                prevPage()
              }
            "
            :disabled="isFirstPage"
          >
            Prev Page
          </button>
          <button
            class="next-page"
            @click="
              () => {
                currentPageNumber++
                nextPage()
              }
            "
            :disabled="isLastPage"
          >
            Next Page
          </button>
        </div>
      </div>
    </div>
    <div class="float-child">
      <input type="text" placeholder="Search" v-model="searchValue" />
      <div>
        <input
          type="checkbox"
          id="fromLangCB"
          :value="fromLang.name?.toLowerCase()"
          v-model="searchField"
        />
        <label for="fromLangCB">{{ fromLang.name }} </label>
        <input
          type="checkbox"
          id="toLang"
          :value="toLang.name?.toLowerCase()"
          v-model="searchField"
        />
        <label for="toLang">{{ toLang.name }} </label>
        <button :disabled="!searchValue.length" @click="searchValue = ''">
          Clear
        </button>
      </div>
      <AddVocab></AddVocab>
      <div>
        <button
          id="deleteSelected"
          @click="deleteSelected"
          :disabled="itemsSelected.length == 0"
        >
          Delete Selected Entries
        </button>
        <fieldset class="block">
          <legend>Add Vocab Entry Settings</legend>
          <div class="row">
            <label>Translation Source</label>
            <input
              type="radio"
              id="autoLookup"
              :value="true"
              v-model="apiLookup"
            />
            <label for="autoLokkup">API Call</label>
            <input
              type="radio"
              id="manualLookup"
              :value="false"
              v-model="apiLookup"
            />
            <label for="manualLookup">Manual Entry</label>
          </div>

          <div class="row">
            <label for="apiKey" :class="apiLabelClass" :disabled="!apiLookup"
              >API Key</label
            >
            <input
              type="text"
              id="apiKey"
              v-model="_apiKey"
              :disabled="!apiLookup"
            />
            <button
              class="button"
              @click="setApiKey"
              :disabled="updateDisabled"
            >
              Update
            </button>
            <label class="warning" v-if="invalidKey"
              >The API Key is invalid</label
            >
          </div>
        </fieldset>
        <fieldset class="block">
          <legend>Vocabulary as CSV File (word and translation only)</legend>
          <div class="row">
            <label for="csvImport">Import</label>
            <form
              action="http://localhost:5000/vocab/import_csv"
              method="post"
              enctype="multipart/form-data"
              target="result_tab"
            >
              <input
                type="file"
                id="csvImport"
                accept=".csv"
                name="file"
                @change="csvImportFileSelect"
              />
              <input
                v-show="csvImportSelected > 0"
                type="submit"
                value="Upload"
              />
            </form>
          </div>
          <div class="row">
            <label for="csvExport">Export</label>
            <button id="csvExport" @click="exportCsvFile">Download</button>
          </div>
          <div class="import-label">
            Words from the csv file are added to the existing vocabulary
          </div>
        </fieldset>
        <fieldset class="block">
          <legend>
            Vocabulary as JSON File (includes test history and metadata)
          </legend>
          <div class="row">
            <label for="jsonImport">Import</label>
            <form
              action="http://localhost:5000/vocab/import_json"
              method="post"
              enctype="multipart/form-data"
              target="result_tab"
            >
              <input
                type="file"
                id="jsonImport"
                accept=".json"
                name="file"
                @change="jsonImportFileSelect"
              />
              <input
                v-show="jsonImportSelected > 0"
                type="submit"
                value="Upload"
              />
            </form>
          </div>
          <div class="row">
            <label for="jsonExport">Export</label>
            <button id="jsonExport" @click="exportJsonFile">Download</button>
          </div>
          <div class="import-label">
            The file you import will replace the existing vocabulary. Be sure to
            export a backup and save it before importing a new one.
          </div>
        </fieldset>
      </div>
    </div>
  </div>
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}
.col {
  display: flex;
  flex-direction: column;
  margin-top: 10px;
  align-items: flex-start;
  padding-top: 10px;
}

.float-container {
  border: 3px solid #fff;
  padding: 20px;
  display: grid;
  grid-template-columns: 60% auto;
}

.float-child {
  width: 630px;
  float: left;
  padding: 20px;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  #deleteSelected {
    margin-top: 10px;
    margin-left: 0;
  }
  .del-button {
    margin-left: 5px;
    width: 60px;
    height: 20px;
  }

  .hide {
    border: none;
    background-color: inherit;
    width: 130px;
  }

  .float-right {
    margin-left: 35%;
  }

  button {
    margin-left: 5px;
  }

  .block {
    margin-top: 40px;
    width: 80%;
    padding: 5px;
  }

  .customize-footer {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 10px;
  }

  .customize-index {
    flex: 1 1 auto;
    padding: 10px;
  }

  .customize-page-input {
    display: block;
  }

  .customize-page-input input {
    width: 30px;
  }

  .header-item {
    background-color: yellow;
    background: yellow;
    display: flex;
    justify-content: space-between;
  }

  label {
    margin-top: auto;
    margin-bottom: auto;
  }

  .import-label {
    margin-top: 10px;
    color: blue;
  }

  .warning {
    color: red;
  }

  .row {
    display: flex;
    flex-direction: row;
    margin-top: 10px;
    gap: 10px;
    align-items: center;
    justify-content: flex-start;
  }

  input {
    margin-left: 5px;
    margin-right: 5px;
    margin-top: auto;
    margin-bottom: auto;
  }

  input[type='submit'] {
    margin-top: 5px;
    margin-left: 0;
  }

  .disabled {
    color: gray;
  }

  input[type='text'] + label {
    float: left;
  }

  #csvImport {
    margin-top: 5px;
    margin-left: 0;
  }

  #csvExport {
    margin-top: 5px;
    margin-left: 0;
  }
}
</style>
