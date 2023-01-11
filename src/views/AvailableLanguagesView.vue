<script setup>

import { inject, computed, ref } from 'vue'
import  LangsTable from 'vue3-easy-data-table' ;
import { usePagination } from "use-vue3-easy-data-table";

const dataTable = ref();

const rowsPerPage = 15;

const currentPageNumber = ref(1);

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
} = usePagination(dataTable);

const headers = [
  { text: "ID", value: "id", sortable: true },
  { text: "Name", value: "name", width: 200, sortable: true },
  { text: "Native Name", value: "nativeName", width: 200, sortable: true },
];

const data = inject("langs")

const getItems = computed( () => {
  if (data) {
    const _data = data.value || [];
    const items = Object.keys(_data).map(id => 
      ({id, name: _data[id]["name"], nativeName: _data[id]["nativeName"]})
    );
    for (let i = items.length; i % 15 != 0; i++) {
      items.push({id: "", name: "", nativeName: ""});
    }
    return items;
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
            min="1" :max="maxPaginationNumber"
            v-model="currentPageNumber"
            @change="() => {
              updatePage(currentPageNumber)
              }"
          >
            of {{ maxPaginationNumber }}
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
    width: 500px;
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
