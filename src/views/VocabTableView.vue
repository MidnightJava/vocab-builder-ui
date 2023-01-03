<script setup>
  import { inject, computed, ref } from 'vue';
  import  VocabTable from 'vue3-easy-data-table' ;

  const fromLang = inject("fromLang");
  const toLang = inject("toLang");
  const vocab = inject('vocab');

  const headers = [
    { text: fromLang, value: fromLang.toLowerCase(), sortable: true },
    { text: toLang, value: toLang.toLowerCase(), sortable: true }
  ];

  const getItems = computed( () => {
    if (vocab) {
      const data = vocab.value || {};
      return Object.entries(data).map((entry) => {
        return {[toLang.toLowerCase()]: entry[0], [fromLang.toLowerCase()]: entry[1].translations }
      })
    } else {
      return []
    }
    
  })

  const searchValue = ref();

</script>

<template>
  <div class="float-container">
    <div class="float-child">
      <VocabTable
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
}
</style>