import init, { MatrixUsize } from "mathrs";

function printMe() {
  console.log("I get called from print.js!");
}

async function sumMatrix() {
  await init();
  console.log("Normal init");
  try {
    console.log("trying...");
    const matrixA = new MatrixUsize([1, 1, 1, 1], 2, 2);
    const matrixB = new MatrixUsize([2, 2, 2, 2], 2, 2);
    const result = MatrixUsize.sum(matrixA, matrixB);
    let i = 0;
    let j = 0;
    for (i = 0; i < result.rows; i++) {
      for (j = 0; j < result.columns; j++) {
        console.log(result.get(i, j));
      }
    }
  } catch (err) {
    console.error("error", err);
  }
}

export { printMe, sumMatrix };
