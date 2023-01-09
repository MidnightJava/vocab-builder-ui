<template>
    <p>
      <button @click="showModal">Add Entry</button>
    </p>
    <!-- If the option changed modal component the name
    <MyModal>
    -->
    <Modal v-model="isShow" :close="closeModal">
      <div class="modal">
        <div>Lookup meaning of either word, or specify a translation manually</div>
        <div id="textDiv">
            <input id="fromLangInp" :placeholder="fromLangHint" type="text" v-model="fromWord" />
            <input id="toLangInp" :placeholder="toLangHint" type="text" v-model="toWord" />
        </div>
        <div id="buttonDiv">
            <button :disabled="lookupDisabled" @click="lookup">Lookup Translation</button>
            <button  @click="closeModal">{{ submitText }}</button>
        </div>
      </div>
    </Modal>
  </template>
  
  <script setup>
    import { ref, computed, inject } from 'vue'

    const isShow = ref(false);
    const fromWord = ref("");
    const toWord = ref("");

    const fromLang = inject("fromLang");
    const toLang = inject("toLang");

    const showModal = () => {
        isShow.value = true;
    }

    const closeModal = () => {
        isShow.value = false;
    }

    const lookup = () => {

    }

    const fromLangHint = computed( () => {
        return `Enter ${toLang} word or phraase`;
    })

    const toLangHint = computed( () => {
        return `Enter ${fromLang} word or phrase`;
    })

    const lookupDisabled = computed( () => {
        return  (fromWord.value.length > 0 && toWord.value.length > 0) ||
                (fromWord.value.length === 0 && toWord.value.length == 0);
    })

    const submitText = computed( () => {
        if  (fromWord.value.length === 0 || toWord.value.length === 0) {
            return "Cancel";
        } else {
            return "Submit";
        }
    })
  </script>
  
  <style scoped lang="scss">
    .modal {
        width: 400px;
        padding: 10px;
        box-sizing: border-box;
        background-color: #fff;
        font-size: 20px;
        text-align: center;
    }

    input {
        // margin-bottom: 5px;
        margin-bottom: 5px;
        width: 350px;
    }

    button {
        margin-top: 5px;
        margin-right: 5px;
        // margin-left: 0;
        // text-align: left;
    }
    
    #buttonDiv {
        display: flex;
        justify-content: left;
    }

    #textDiv {
        margin-top: 10px;
    }

    div {
        font-size: 0.7em;
    }
  </style>