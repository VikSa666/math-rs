import _ from "lodash";
import { printMe, sumMatrix } from "./print";

function component() {
  const element = document.createElement("div");
  const btn = document.createElement("button");

  // Lodash, now imported by this script
  element.innerHTML = _.join(["Hello", "webpack"], " ");

  btn.innerHTML = "Click me and check the console!";
  btn.onclick = sumMatrix;

  element.appendChild(btn);

  return element;
}

document.body.appendChild(component());
