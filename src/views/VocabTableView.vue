<script setup>
  import { inject, computed } from 'vue';
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

</script>

<template>
    <VocabTable
      :headers="headers"
      :items="getItems"
    />
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
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