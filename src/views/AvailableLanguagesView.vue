<script setup>
import { inject, computed, ref, watch } from 'vue'
import LangsTable from 'vue3-easy-data-table'
import PartsTable from './PartsOfSpeechView.vue'
import { usePagination } from 'use-vue3-easy-data-table'
import UseFetch from '../components/UseFetch.vue'

const useFetch = ref(null)

const dataTable = ref()

const rowsPerPage = 15

const currentPageNumber = ref(1)

const fromLang = inject('fromLang')
const toLang = inject('toLang')

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

const headers = [
  { text: 'ID', value: 'id', sortable: true },
  { text: 'Name', value: 'name', width: 200, sortable: true },
  { text: 'Native Name', value: 'nativeName', width: 200, sortable: true },
  { text: 'Select As:', value: 'select', width: 185, sortable: false },
]

const data = inject('langs')

const getItems = computed(() => {
  if (data) {
    const _data = data.value || []
    let cnt = 1
    const items = Object.keys(_data).map(id => ({
      count: cnt++,
      id,
      name: _data[id]['name'],
      nativeName: _data[id]['nativeName'],
    }))
    for (let i = items.length; i % 15 != 0; i++) {
      items.push({ count: 0, id: '', name: '', nativeName: '' })
    }
    return items
  } else {
    return []
  }
})

const setTo = obj => {
  toLang.value = obj
}

const setFrom = obj => {
  fromLang.value = obj
}

const setFromId = inject('setFromIdFunc')
const setToId = inject('setToIdFunc')

const setDefRes = ref()

const setDefaultLangs = async () => {
  const res = await useFetch.value.fetch(
    `http://localhost:5000/languages/set_defaults`,
    'POST',
    {
      from: fromLang.value.name,
      to: toLang.value.name,
    }
  )
  setDefRes.value = res
}

const searchValue = ref('')
const searchField = ref(['name', 'id'])
const showOpt = ref(0)

watch(
  () => toLang.value.name,
  (newv, oldv) => {
    if (newv == fromLang.value.name) {
      toLang.value.name = oldv
    }
  }
)

watch(
  () => fromLang.value.name,
  (newv, oldv) => {
    if (newv == toLang.value.name) {
      fromLang.value.name = oldv
    }
  }
)
</script>

<template>
  <UseFetch ref="useFetch" />
  <div class="float-container">
    <div class="float-child mx-2">
      <input type="text" placeholder="Search" v-model="searchValue" />
      <div>
        <input type="checkbox" id="idCB" value="id" v-model="searchField" />
        <label for="idCB">ID </label>
        <input type="checkbox" id="nameCB" value="name" v-model="searchField" />
        <label for="nameCB">Name </label>
        <input
          type="checkbox"
          id="nativeNameCB"
          value="nativeName"
          v-model="searchField"
        />
        <label for="nativeNameCB">Native Name</label>
        <button :disabled="!searchValue.length" @click="searchValue = ''">
          Clear
        </button>
      </div>
      <div class="top-margin-10">Select the languages to use</div>
      <div class="top-margin-5 grid-3-col">
        <label>Original:</label>
        <input
          class="lang-inp left-margin-5"
          type="text"
          v-model.lazy="fromLang.name"
          @change="e => setFromId(e.target.value)"
          placeholder="Enter here or select from table"
        />
        <label class="lang-id-label">ID: {{ fromLang.id }}</label>
      </div>
      <div class="grid-3-col">
        <label>Translated:</label>
        <input
          class="lang-inp left-margin-5"
          type="text"
          v-model.lazy="toLang.name"
          @change="e => setToId(e.target.value)"
          placeholder="Enter here or select from table"
        />
        <label class="lang-id-label">ID: {{ toLang.id }}</label>
      </div>
      <button
        class="default-btn"
        :disabled="!toLang.id.length || !fromLang.id.length"
        @click="setDefaultLangs"
      >
        Set as Defaults
      </button>
      <div id="parts-table">
        <PartsTable></PartsTable>
      </div>
    </div>
    <div class="float-child">
      <LangsTable
        ref="dataTable"
        :headers="headers"
        :items="getItems"
        :rows-per-page="rowsPerPage"
        :search-value="searchValue"
        :search-field="searchField"
        alternating
        border-cell
        hide-footer
      >
        <template #item-select="item">
          <div
            @mouseleave="
              () => {
                showOpt = 0
              }
            "
            @mouseenter="
              () => {
                showOpt = item.count
              }
            "
          >
            <span v-if="showOpt === item.count && item.count !== 0">
              <button class="lang-button" @click="() => setFrom(item)">
                Original
              </button>
              <button class="lang-button" @click="() => setTo(item)">
                Translated
              </button>
            </span>
            <button class="lang-button hide" v-else></button>
          </div>
        </template>
      </LangsTable>
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
          of {{ maxPaginationNumber }}
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
  </div>
</template>

<style scoped>
.float-container {
  border: 3px solid #fff;
  padding: 10px;
  display: flex;
  flex-direction: row;
}

.float-child {
  width: 640px;
  float: left;
  padding: 20px;
}

#parts-table {
  margin-top: 20px;
  float: left;
}

#deleteSelected {
  margin-top: 20px;
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
.lang-button {
  height: 20px;
}

button {
  margin-left: 5px;
}

.default-btn {
  margin-left: 0px;
}

.hide {
  border: none;
  background-color: inherit;
  width: 130px;
}

.top-margin-5 {
  margin-top: 5px;
}

.top-margin-10 {
  margin-top: 10px;
}

.grid-3-col {
  display: grid;
  grid-template-columns: 25% 40% auto;
}

.left-margin-5 {
  margin-left: 5px;
}

.lang-inp {
  width: 180px;
  margin-left: 5px;
}

.lang-id-label {
  margin-left: 5px;
}
</style>
