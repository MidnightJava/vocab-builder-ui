<script setup>

import { inject, computed, ref } from 'vue'
import  LangsTable from 'vue3-easy-data-table' ;

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
        :headers="headers"
        :items="getItems"
        v-model:items-selected="itemsSelected"
        :rows-items=[20,30,50]
        :rows-per-page="20"
        :search-value="searchValue"
        :search-field="searchField"
        alternating
        border-cell
      />
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
</style>
