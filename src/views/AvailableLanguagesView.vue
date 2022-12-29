<script setup>
import { useFetch } from '../components/fetch.js'

import  LangsTable from 'vue3-easy-data-table' ;

const headers = [
  { text: "Name", value: "name", sortable: true },
  { text: "ID", value: "id", sortable: true }
];

const { data } = useFetch('http://localhost:5000/languages')
// let dataVal = data.value;

const _data = [
  {"name": "English", "id": "en"},
]
</script>

<template>
 <LangsTable
  :headers="headers"
  :items=_data
 />
</template>

<style>
</style>
