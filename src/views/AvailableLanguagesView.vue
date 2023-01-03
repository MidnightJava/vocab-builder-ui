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

</script>

<template>
  <div class="float-container">
    <div class="float-child">
      <LangsTable
        :headers="headers"
        :items="getItems"
        :rows-items=[20,30,50]
        :rows-per-page="20"
        :search-value="searchValue"
        dense
      />
    </div>
    <div class="float-child">
      <span>Search: </span>
      <input type="text" v-model="searchValue" />
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
</style>
