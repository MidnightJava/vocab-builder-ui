<script setup>

import { inject, computed } from 'vue'
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

</script>

<template>
 <LangsTable
  :headers="headers"
  :items="getItems"
 />
</template>

<style>
</style>
