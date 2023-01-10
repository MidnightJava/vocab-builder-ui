<script setup>
  import { inject, computed, ref } from 'vue';
  import { usePagination, useRowsPerPage } from "use-vue3-easy-data-table";

  import { useFetch } from "../components/fetch.js"
  import  VocabTable from 'vue3-easy-data-table' ;
  import AddVocab from "./AddVocab.vue";

  const dataTable = ref();

  const rowsPerPage = 15;

  const currentPageNumber = ref(1);

  const {
    currentPageFirstIndex,
    currentPageLastIndex,
    clientItemsLength,
    maxPaginationNumber,
    currentPaginationNumber,
    isFirstPage,
    isLastPage,
    nextPage,
    prevPage,
    updatePage,
  } = usePagination(dataTable);

  const {
    rowsPerPageOptions,
    rowsPerPageActiveOption,
    updateRowsPerPageActiveOption,
  } = useRowsPerPage(dataTable);

  const updateRowsPerPageSelect = (e) => {
    updateRowsPerPageActiveOption(Number((e.target).value));
  };

  const fromLang = inject("fromLang");
  const toLang = inject("toLang");
  const vocab = inject('vocab');

  const headers = [
    { text: fromLang, value: fromLang.toLowerCase(), sortable: true },
    { text: toLang, value: toLang.toLowerCase(), sortable: true },
    { text: "Action", value: "opts", sortable: false }
  ];

  const getItems = computed( () => {
    if (vocab) {
      const data = vocab.value || {};
      let count = 1;
      return Object.entries(data).map((entry) => {
        return {id: count++, [toLang.toLowerCase()]: entry[0], [fromLang.toLowerCase()]: entry[1].translations, opts: false }
      })
    } else {
      return []
    }
    
  })

  const searchValue = ref("");
  const searchField = ref([toLang.toLowerCase(), fromLang.toLowerCase()]);
  const itemsSelected =  ref([]);
  const showOpt = ref(0)

  const deleteSelected = () => {
    for (const item of itemsSelected.value) {
      delete vocab.value[item[toLang.toLowerCase()]]
    }

    useFetch('http://localhost:5000/vocab/delete_entry', ref(null), ref(null), "POST", itemsSelected.value, () => {
      console.log("Deleted ")
    })
  }

  const delKey = ref("");
  const err = ref(null)
  const deleteEntry = (item) => {
    useFetch('http://localhost:5000/vocab/delete_entry', delKey, err, "POST", {"key": item[toLang.toLowerCase()]}, () => {
      if (err.value) {
        console.log(err.value)
      } else {
        let res = delKey.value;
        delete vocab.value[res['deleted']];
        console.log(`Deleted ${res['deleted']}`)
      }
      
    })
   
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
        <template #item-opts="item" >
          <div @mouseout="() => showOpt = 0" @mouseover="() => showOpt = item.id">
              <button class="del-button" @click="() => deleteEntry(item)" v-if="showOpt === item.id">
                Delete
              </button>
              <button class="del-button hide" v-else></button>
          </div>
        </template>
        <template #header-opts>
          <font-awesome-icon class="float-right" icon="fa-solid fa-ellipsis" />
        </template>
      </VocabTable>
      <div class="customize-footer">
        <div class="customize-index">
          Now displaying entries {{currentPageFirstIndex}} ~ {{currentPageLastIndex}} of {{clientItemsLength}}
        </div>

        <span class="customize-page-input">
          Go to page
          <input
            type="number"
            min="1" :max="Math.ceil(clientItemsLength/rowsPerPage)"
            v-model="currentPageNumber"
            @change="() => {
              updatePage(currentPageNumber)
              }"
          >
            of {{ Math.ceil(clientItemsLength/rowsPerPage) }}
        </span>
      
        <div class="customize-pagination">
          <button class="prev-page" @click="() => {currentPageNumber--; prevPage()}" :disabled="isFirstPage">prev page</button>
          <button class="next-page" @click="() => {currentPageNumber++; nextPage()}" :disabled="isLastPage">next page</button>
        </div>
      </div>
    </div>
    <div class="float-child">
      <input type="text" placeholder="Search" v-model="searchValue" />
      <div>
        <input type="checkbox" id="fromLangCB" :value="fromLang.toLowerCase()" v-model="searchField" />
        <label for="fromLangCB">{{ fromLang }} </label>
        <input type="checkbox" id="toLang" :value="toLang.toLowerCase()" v-model="searchField" />
        <label for="toLang">{{ toLang }} </label>
        <button :disabled="!searchValue.length" @click="searchValue = ''">Clear</button>
      </div>
      <div>
        <button id="deleteSelected" @click="deleteSelected" :disabled="itemsSelected.length == 0">Delete Selected Entries</button>
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
}

.float-child {
    width: 400px;
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
    margin-left: 30px;
    width: 60px;
    height: 20px;
  }

  .hide {
    border: none;
    background-color: inherit;
  }

  .float-right {
    margin-left: 35%;
  }

  button {
    margin-left:5px;
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
  
}
</style>