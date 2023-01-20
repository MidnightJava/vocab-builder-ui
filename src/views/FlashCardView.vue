<script setup>

import { inject, ref, computed } from 'vue';
import { VueFlip } from 'vue-flip';

const fromLang = inject("fromLang");
const toLang = inject("toLang")
const started = ref(false);

const fromWord = ref("");
const toWord = ref("");

const slot1 = ref('front');
const slot2 = ref('back');

const flipped = ref(false);

const startTest = () => {
  started.value = true;
}

const flipCommand = computed( () => {
  if (flipped.value) {
    return `Show Original`
  } else  {
    return `Show Translation`
  }
})

const startButtonText = computed(() => {
  return started.value ? "Next Word" : "Start Vocab Test";
})
</script>

<template>
  <div>
    <vue-flip
      v-model="flipped"
      width="600px"
      height="400px"
      v-if="started"
    >
      <template v-slot:[slot1]>
        <div class="card">
          <div class="entry"> {{ fromWord }}</div>
          <div class="btn-div">
            <font-awesome-icon  @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
          </div>
        </div>
      </template>
      <template v-slot:[slot2]>
        <div class="card">
          <div class="entry"> {{ toWord }}</div>
          <div class="btn-div">
            <font-awesome-icon   @click="() => {flipped = !flipped}" class="flip-btn" icon="fa-solid fa-rotate" />
            <button id="mark-btn">Mark Incorrect</button>
          </div>
        </div>
      </template>
    </vue-flip>
    <div class="btn-div">
      <button @click="startTest">{{ startButtonText }}</button>
      <button v-if="started">Refresh Word List</button>
    </div>
  </div>
</template>

<style scoped>

  .btn-div {
    margin-top: 0px;
    padding-left: 20px;
    padding-right: 20px;
  }


  .card {
    display: grid;
    grid-template-rows: 70% 30%;
    margin-top: 30px;
    padding-top: 20%;
    text-align: center;
    font-size: 3.0rem;
    border-style: solid;
    border-width: 2px;
    height: 50%;
  }

  .entry {
    margin-top: 20%;
  }

  .front {
    margin-top:20px;
    margin-bottom: 20px;
  }

  button {
    margin-right: 5px;
  }

  .flip-btn {
    float:left;
    margin-left: 5px;
    margin-top: 10px;
    cursor:auto;
  }

  #mark-btn {
    float: right;
    margin-top: 30px;
  }

</style>
