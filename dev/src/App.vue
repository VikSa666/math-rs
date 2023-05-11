<template>
  <h1>Marhaba!</h1>
  <div>
    <textarea name="matA" id="matA" cols="30" rows="10" v-model="matA" />
    <textarea name="matB" id="matB" cols="30" rows="10" v-model="matB" />
    <span>=></span>
    <textarea
      name="result"
      id="result"
      cols="30"
      rows="10"
      v-model="result"
    ></textarea>
  </div>
  <div>
    <button @click="performSum">+</button>
    <button @click="performSub">-</button>
    <button @click="performMul">*</button>
  </div>
</template>

<script lang="ts" setup>
import init, { MatrixUsize } from "math-rs";
import { ref } from "vue";

const matA = ref("");
const matB = ref("");
const result = ref("Nothing yet...");

const performSum = async () => {
  await init();
  result.value = MatrixUsize.sum(
    MatrixUsize.from_string(matA.value),
    MatrixUsize.from_string(matB.value)
  ).to_string();
};

const performSub = async () => {
  await init();
  alert("Not yet implemented");
};

const performMul = async () => {
  await init();
  alert("Not yet implemented");
};
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
