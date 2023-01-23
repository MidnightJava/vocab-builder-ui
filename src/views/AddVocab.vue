<template>
      <button @click="showModal">Add Entry</button>
    <!-- If the option changed modal component the name
    <MyModal>
    -->
    <Modal
        v-model:visible="show"
        :width="500"
        :title="title"
        :closable="false"
        :cancelButton="{text: 'Close', onclick: closeModal, loading: false}"
        :draggable="true"
        :mask="true"
        :animation="true"
    >
      <div class="modal">
        <div id="subtitle">Enter text for either language or both</div>
        <div id="textDiv">
            <input id="fromLangInp" :placeholder="fromLangHint" type="text" v-model="fromWord" />
            <button @click="() => {fromWord = ''; msg = null}" :disabled="!fromWord.length">Clear</button>
            <input id="toLangInp" :placeholder="toLangHint" type="text" v-model="toWord" />
            <button @click="() => {toWord = ''}" :disabled="!toWord.length">Clear</button>
        </div>
        <div id="buttonDiv">
            <button :disabled="lookupDisabled" @click="lookup">Lookup Translation</button>
            <button  :disabled="submitDisabled" @click="postEntry()">Submit</button>
        </div>
        <div id="errMsg">{{ msg }}</div>
      </div>
    </Modal>
  </template>
  
  <script setup>
    import { ref, computed, inject, watch } from 'vue'

    import { useFetch } from '../components/fetch.js'

    import { Modal } from 'usemodal-vue3';

    const msg = ref("");

    const initialCap = inject('initialCap');
    let role = inject('role');

    const fromWord = inject('fromWord');
    const toWord = inject('toWord');

    const fromLang = inject("fromLang");
    const toLang = inject("toLang");
    const transResult = ref(null);
    const errResult = ref(null);

    const vocab = inject("vocab");
    const err = ref(null);

    let show = inject('show');

    const showModal = () => {
        role.value = 'add';
        fromWord.value = "";
        toWord.value = "";
        show.value = true;
        msg.value = "";
    }

    const closeModal = () => {
       show.value = false;
       msg.value = "";
    }

    const postEntry = () => {
        useFetch(`http://localhost:5000/vocab/add_entry`, transResult, errResult, "POST",
            {
                "from": fromWord.value.replace(/\,$/, ""),
                "to": toWord.value
            },
            () => {
                if (errResult.value) {
                    console.log(`Add Entry call returned error: ${errResult.value}`);
                } else {
                    useFetch('http://localhost:5000/vocab/get_all', vocab, err, "GET", null, () => {
                        closeModal();
                    });
                }
            })
    }

    const lookup = () => {
        let word = null;
        let [frl, tol] = [null, null]
        let targetRef = null;
        if (fromWord.value.length > 0 & toWord.value.length == 0) {
            if (fromWord.value.includes(",")) {
                msg.value = "Multiple words are for manual entry only";
                return;
            } else {
                msg.value = "";
            }
            [frl, tol] = [fromLang.value.id, toLang.value.id];
            word = fromWord.value;
            targetRef = toWord;
        } else if (toWord.value.length > 0 & fromWord.value.length == 0) {
            [frl, tol] = [toLang.value.id, fromLang.value.id];
            word = toWord.value;
            targetRef = fromWord;
        } else {
            console.log("Invalid input. Both language inputs have content");
            return;
        }
        useFetch(`http://localhost:5000/vocab/translate?from_lang=${frl}&to_lang=${tol}&word=${word}`, transResult, errResult, "GET", null, () => {
            let res = transResult.value;
            if ('result' in res) {
                targetRef.value = res.result;
            }
            
        })
    }

    const fromLangHint = computed( () => {
        return `Enter ${fromLang.value.name} word or phrase`;
    })

    const toLangHint = computed( () => {
        return `Enter ${toLang.value.name} word or phrase`;
    })

    const lookupDisabled = computed( () => {
        return  (fromWord.value.length > 0 && toWord.value.length > 0) ||
                (fromWord.value.length === 0 && toWord.value.length == 0);
    })

    const submitDisabled = computed( () => {
        return  fromWord.value.length === 0 || toWord.value.length === 0
    })

    const title = computed( () => {
        return `${initialCap(role.value)} Vocabulary Entry`
    })

    watch(fromWord, newVal => {
        msg.value = "";
        let l = newVal.split(",");
        for (let word of l) {
            if (word.trim() != word) {
                l[l.indexOf(word)] = word.trim();
            }
        }
        if (l != newVal.split(",")) {
            fromWord.value = l.join(",")
        }
    });
  </script>
  
  <style lang="scss">
    .modal {
        width: 450px;
        padding: 10px;
        box-sizing: border-box;
        background-color: #fff;
        font-size: 20px;
        text-align: center;
        height: 100%;
    }

    input[type=text] {
        margin-bottom: 5px;
        margin-right: 5px;
        width: 330px;
    }

    button {
        margin-top: 5px;
        margin-right: 5px;
    }
    
    #buttonDiv {
        display: flex;
        justify-content: left;
        margin-left: 5px;
        padding-left: 15px;
    }

    #textDiv {
        margin-top: 10px;
    }

    #subtitle {
        font-size: 0.8em;
    }

    .modal-vue3-footer-ok {
        display: none !important;
    }

    #errMsg {
        margin-top: 10px;
        height: 10px;
        font-size: 0.8rem;
    }
  </style>