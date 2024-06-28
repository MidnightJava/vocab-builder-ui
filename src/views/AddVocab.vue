<template>
  <UseFetch ref="useFetch" />
  <button @click="showModal">Add Entry</button>
  <!-- If the option changed modal component the name
    <MyModal>
    -->
  <Modal
    v-model:visible="show"
    :width="500"
    :title="title"
    :closable="false"
    :cancelButton="{ text: 'Close', onclick: closeModal, loading: false }"
    :draggable="true"
    :mask="true"
    :animation="true"
  >
    <div class="modal">
      <div id="subtitle">Enter text for either language or both</div>
      <div id="textDiv">
        <input
          id="fromLangInp"
          :placeholder="fromLangHint"
          type="text"
          v-model="fromWord"
          ref="fromWordEl"
        />
        <button
          @click="
            () => {
              fromWord = ''
              msg = ''
            }
          "
          :disabled="!fromWord.length"
        >
          Clear
        </button>
        <input
          id="toLangInp"
          :placeholder="toLangHint"
          type="text"
          v-model="toWord"
        />
        <button
          @click="
            () => {
              toWord = ''
              msg = ''
            }
          "
          :disabled="!toWord.length"
        >
          Clear
        </button>
      </div>
      <div id="buttonDiv">
        <button :disabled="lookupDisabled" @click="lookup">
          Lookup Translation
        </button>
        <button :disabled="submitDisabled" @click="postEntry()">Submit</button>
      </div>
      <div id="errMsg">{{ msg }}</div>
    </div>
  </Modal>
</template>

<script setup>
import { ref, computed, inject, watch, nextTick } from 'vue'

const useFetch = ref(null)

import UseFetch from '../components/UseFetch.vue'

import { Modal } from 'usemodal-vue3'

const msg = ref('')

const fromWordEl = ref()

const initialCap = inject('initialCap')
let role = inject('role')

const fromWord = inject('fromWord')
const toWord = inject('toWord')

const fromLang = inject('fromLang')
const toLang = inject('toLang')
const transResult = ref(null)

const vocab = inject('vocab')

let show = inject('show')

const showModal = () => {
  role.value = 'add'
  fromWord.value = ''
  toWord.value = ''
  show.value = true
  msg.value = ''
  nextTick(() => focusInput())
}

const closeModal = () => {
  show.value = false
  msg.value = ''
}

const postEntry = async () => {
  let res = await useFetch.value.fetch(
    `http://localhost:5000/vocab/${
      role.value === 'add' ? 'add_entry' : 'update_entry'
    }`,
    'POST',
    {
      from: fromWord.value.replace(/,$/, ''),
      to: toWord.value,
    }
  )
  transResult.value = res

  res = await useFetch.value.fetch('http://localhost:5000/vocab/get_all', 'GET')
  vocab.value = res
  closeModal()
}

const lookup = async () => {
  let word = null
  let [frl, tol] = [null, null]
  let targetRef = null
  if ((fromWord.value.length > 0) & (toWord.value.length == 0)) {
    if (fromWord.value.includes(',')) {
      msg.value = 'Multiple words are for manual entry only'
      return
    } else {
      msg.value = ''
    }
    ;[frl, tol] = [fromLang.value.id, toLang.value.id]
    word = fromWord.value
    targetRef = toWord
  } else if ((toWord.value.length > 0) & (fromWord.value.length == 0)) {
    if (toWord.value.includes(',')) {
      msg.value = 'Use separate entries to specify multiple tranlations'
      return
    } else {
      msg.value = ''
      ;[frl, tol] = [toLang.value.id, fromLang.value.id]
      word = toWord.value
      targetRef = fromWord
    }
  } else {
    console.log('Invalid input. Both language inputs have content')
    return
  }

  const res = await useFetch.value.fetch(
    `http://localhost:5000/vocab/translate?from_lang=${frl}&to_lang=${tol}&word=${word}`,
    'GET'
  )
  if ('result' in res) {
    targetRef.value = res.result
  }
}

const fromLangHint = computed(() => {
  return `Enter ${fromLang.value.name} word or phrase`
})

const toLangHint = computed(() => {
  return `Enter ${toLang.value.name} word or phrase`
})

const lookupDisabled = computed(() => {
  return (
    (fromWord.value.length > 0 && toWord.value.length > 0) ||
    (fromWord.value.length === 0 && toWord.value.length == 0)
  )
})

const submitDisabled = computed(() => {
  return fromWord.value.length === 0 || toWord.value.length === 0
})

const title = computed(() => {
  return `${initialCap(role.value)} Vocabulary Entry`
})

watch(fromWord, newVal => {
  msg.value = ''
  let l = newVal.split(',')
  for (let word of l) {
    //Remove extraneous spaces from words in list except last one
    if (word.trim() != word && word !== l.slice(-1)[0]) {
      l[l.indexOf(word)] = word.trim()
    }
  }
  if (l != newVal.split(',')) {
    fromWord.value = l.join(',')
  }
})

const focusInput = () => {
  fromWordEl.value?.focus()
}
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

input[type='text'] {
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
