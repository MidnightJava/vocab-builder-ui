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

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
