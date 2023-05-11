import _ from "lodash";
import { printMe, sumMatrix } from "./print";

async function component() {
  const element = document.createElement("div");

  // Lodash, now imported by this script
  element.innerHTML = _.join(["Hello", "webpack"], " ");

  btn.innerHTML = "Click me and check the console!";
  const matA = document.getElementById("matA").textContent;
  const matB = document.getElementById("matB").textContent;
  const sum = (document.getElementById("sumButton").onclick = await sumMatrix(
    matA,
    matB
  ));

  document.getElementById("result").textContent = sum;

  element.appendChild(btn);

  return element;
}

document.body.appendChild(component());
