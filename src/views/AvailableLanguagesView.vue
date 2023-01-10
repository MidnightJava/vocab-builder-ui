<script setup>

import { inject, computed, ref } from 'vue'
import  LangsTable from 'vue3-easy-data-table' ;
import { usePagination, useRowsPerPage } from "use-vue3-easy-data-table";

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

const headers = [
  { text: "ID", value: "id", sortable: true },
  { text: "Name", value: "name", sortable: true },
  { text: "Native Name", value: "nativeName", sortable: true },
];

const data = inject("langs")

const getItems = computed( () => {
  if (data) {
    const _data = data.value || [];
    return Object.keys(_data).map(id => 
      ({id, name: _data[id]["name"], nativeName: _data[id]["nativeName"]})
    )
  } else {
    return []
  }
  
})



const searchValue = ref();
const searchField = ref([]);
const itemsSelected =  ref([])

</script>

<template>
  <div class="float-container">
    <div class="float-child">
      <LangsTable
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
      />
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
        <input type="checkbox" id="idCB" value="id" v-model="searchField" />
        <label for="idCB">ID </label>
        <input type="checkbox" id="nameCB" value="name" v-model="searchField" />
        <label for="nameCB">Name </label>
        <input type="checkbox" id="nativeNameCB" value="nativeName" v-model="searchField" />
        <label for="nativeNameCB">Native Name</label>
      </div>
    </div>
</div>
</template>

<style>
.float-container {
    border: 3px solid #fff;
    padding: 20px;
}

.float-child {
    width: 400px;
    float: left;
    padding: 20px;
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
</style>
