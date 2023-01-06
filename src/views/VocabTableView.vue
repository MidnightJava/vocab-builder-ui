<script setup>
  import { inject, computed, ref } from 'vue';
  import { useFetch } from "../components/fetch.js"
  import  VocabTable from 'vue3-easy-data-table' ;

  const fromLang = inject("fromLang");
  const toLang = inject("toLang");
  const vocab = inject('vocab');

  const headers = [
    { text: fromLang, value: fromLang.toLowerCase(), sortable: true },
    { text: toLang, value: toLang.toLowerCase(), sortable: true },
    { text: "", value: "opts", sortable: false }
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

  const searchValue = ref();
  const searchField = ref([]);
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

  const doShowIcon = (show, item) => {
    console.log("Show icon: " + show)
    item.opts = show;
  }

  const showOpts = () => {
    console.log("Show Opts")
  }

</script>

<template>
  <div class="float-container">
    <div class="float-child">
      <VocabTable
        :headers="headers"
        :items="getItems"
        v-model:items-selected="itemsSelected"
        :rows-items=[20,30,50]
        :rows-per-page="20"
        :search-value="searchValue"
        :search-field="searchField"
        dense>
        <template #item-opts="{id}" >
          <font-awesome-icon id="optFa" icon="fa-solid fa-bars" @click="showOpts" v-if="showOpt == id" />
          <div id="optDiv" v-else @mouseover="() => showOpt = id" @mouseleave="() => showOpt = 0">
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
          </div>
        </template>
      </VocabTable>
    </div>
    <div class="float-child">
      <input type="text" placeholder="Search" v-model="searchValue" />
      <div>
        <input type="checkbox" id="fromLangCB" :value="fromLang.toLowerCase()" v-model="searchField" />
        <label for="fromLangCB">{{ fromLang }} </label>
        <input type="checkbox" id="toLang" :value="toLang.toLowerCase()" v-model="searchField" />
        <label for="toLang">{{ toLang }} </label>
      </div>
      <button id="deleteSelected" @click="deleteSelected" :disabled="itemsSelected.length == 0">Delete Selected Items</button>
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
  #optDiv {
    min-width: 5px;
    width: 100%;
    height: 100%;
    display: block;
    z-index: 10;
  }

  #optFa {
    z-index: 0;
    margin-left: 10px;
    width: 100%;
  }
}
</style>