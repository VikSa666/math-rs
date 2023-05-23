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
  <div>
    <textarea name="mat" id="mat" cols="30" rows="10" v-model="mat"></textarea>
    <textarea
      name="result2"
      id="result2"
      cols="30"
      rows="10"
      v-model="result2"
    ></textarea>
  </div>
  <div>
    <button @click="preformGaussReduction">Gauss Reduction</button>
    <button @click="performGaussJordanDeterminant">
      Gauss-Jordan Determinant
    </button>
    <button @click="performGaussJordanInverse">Gauss-Jordan Inverse</button>
  </div>
</template>

<script lang="ts" setup>
import init, { RMatrixF32 } from "math-rs";
import { ref } from "vue";

const matA = ref("");
const matB = ref("");
const result = ref("Nothing yet...");

const performSum = async () => {
  await init();
  result.value = RMatrixF32.checked_sum(
    RMatrixF32.from_string(matA.value, 1e-6),
    RMatrixF32.from_string(matB.value, 1e-6)
  ).to_string();
};

const performSub = async () => {
  await init();
  result.value = RMatrixF32.checked_sub(
    RMatrixF32.from_string(matA.value, 1e-6),
    RMatrixF32.from_string(matB.value, 1e-6)
  ).to_string();
};

const performMul = async () => {
  await init();
  result.value = RMatrixF32.checked_mul(
    RMatrixF32.from_string(matA.value, 1e-6),
    RMatrixF32.from_string(matB.value, 1e-6)
  ).to_string();
};

const mat = ref("");
const result2 = ref("Nothing yet...");

const preformGaussReduction = async () => {
  await init();
  result2.value = RMatrixF32.from_string(mat.value, 1e-6)
    .gaussian_triangulation()
    .to_string();
};

const performGaussJordanDeterminant = async () => {
  await init();
  result2.value = RMatrixF32.from_string(mat.value, 1e-6)
    .determinant_using_gauss()
    .toString();
};

const performGaussJordanInverse = async () => {
  await init();
  result2.value = RMatrixF32.from_string(mat.value, 1e-6)
    .inverse_gauss_jordan()
    .to_string();
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
