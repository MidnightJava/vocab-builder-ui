<script setup>
import { inject, computed, ref, provide } from 'vue'
import { usePagination } from 'use-vue3-easy-data-table'

import { useFetch } from '../components/fetch.js'
import VocabTable from 'vue3-easy-data-table'
import AddVocab from './AddVocab.vue'

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

const deleteSelected = () => {
  const toDelete = []

  for (const item of itemsSelected.value) {
    toDelete.push({ key: item[toLang.value?.name?.toLowerCase()] })
  }

  useFetch('http://localhost:5000/vocab/delete_entry', 'POST', toDelete)
    .then(() => {
      useFetch('http://localhost:5000/vocab/get_all').then(res => {
        itemsSelected.value = []
        vocab.value = res
      })
    })
    .catch(err => {
      console.log(`Fetch returned error: ${err}`)
    })
}

const deleteEntry = item => {
  useFetch('http://localhost:5000/vocab/delete_entry', 'POST', {
    key: item[toLang.value.name?.toLowerCase()],
  })
    .then(() => {
      useFetch('http://localhost:5000/vocab/get_all').then(res => {
        vocab.value = res
      })
    })
    .catch(err => {
      console.log(`Fetch returned error: ${err}`)
    })
}

const editEntry = item => {
  role.value = 'edit'
  toWord.value = item[toLang.value.name.toLowerCase()]
  fromWord.value = item[fromLang.value.name.toLowerCase()].join(',')
  show.value = true
}
</script>

<template>
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
      <div>
        <button
          id="deleteSelected"
          @click="deleteSelected"
          :disabled="itemsSelected.length == 0"
        >
          Delete Selected Entries
        </button>
        <AddVocab></AddVocab>
      </div>
    </div>
  </div>
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
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
    margin-top: 20px;
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
}
</style>
