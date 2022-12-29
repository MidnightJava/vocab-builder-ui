<script setup>
  import  VocabTable from 'vue3-easy-data-table' ;

  const headers = [
    { text: "English", value: "english", sortable: true },
    { text: "Italian", value: "italian", sortable: true }
  ];

  const items = [
    { "english": "the man", "italian": "l'uomo" },
    { "english": "the woman", "italian": "la donna" },
    { "english": "the duck", "italian": "l'anatra" },
    { "english": "everywhere", "italian": "ovunque" },
    { "english": "the shark", "italian": "lo squalo" },
    { "english": "in the afternoon", "italian": "di pomeriggio" },
    { "english": "today", "italian": "oggie" },
    { "english": "nothing", "italian": "non niente" },
  ];
</script>

<template>
    <VocabTable
      :headers="headers"
      :items="items"
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