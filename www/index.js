// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/fractal_rs_bg";
import { Universe } from "wasm-game-of-life";

let universe;
const canvas = document.getElementById("game-of-life-canvas");
let zoomFactor = 1.0;
let width = window.innerWidth;
let height = window.innerHeight;
let x = BigInt(-Math.floor(width / 4));
let y = BigInt(0);
let relativeMoveFactor = 50;

const generateUniverse = () => {
    width = window.innerWidth;
    height = window.innerHeight;
    console.log("Generating new universe", { width, height, x, y, zoomFactor });
    universe = Universe.new(width, height, y, x, zoomFactor);

    canvas.height = height;
    canvas.width = width;
};

const drawCells = () => {
    console.log("drawing Cells");
    const cellsRPtr = universe.cells_r();
    console.log("cellsRPtr", cellsRPtr);
    const cellsR = new Uint8Array(memory.buffer, cellsRPtr, width * height);

    const cellsGPtr = universe.cells_g();
    console.log("cellsGPtr", cellsGPtr);
    const cellsG = new Uint8Array(memory.buffer, cellsGPtr, width * height);

    const cellsBPtr = universe.cells_b();
    const cellsB = new Uint8Array(memory.buffer, cellsBPtr, width * height);

    const ctx = canvas.getContext("2d");
    ctx.beginPath();

    const getIndex = (row, column) => {
        return row * width + column;
    };

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            const r = cellsR[idx];
            const g = cellsG[idx];
            const b = cellsB[idx];
            ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
            ctx.fillRect(col, row, 1, 1);
        }
    }

    ctx.stroke();
};

const render = () => {
    console.log("render");
    requestAnimationFrame(() => {
        generateUniverse();
        drawCells();
    });
};

render();

// addEventListener("resize", render);

addEventListener("keyup", (event) => {
    if (event.key === "+") {
        console.log("Zoom in");
        zoomFactor = universe.zoom_in();
        console.log({ zoomFactor });
        universe.update();
    } else if (event.key === "-") {
        console.log("Zoom out");
        zoomFactor = universe.zoom_out();
        console.log({ zoomFactor });
        universe.update();
    } else if (event.key == "w") {
        console.log("Move Up");
        y = universe.move_vertical(
            BigInt(-Math.floor(height / relativeMoveFactor))
        );
        console.log({ y });
    } else if (event.key == "s") {
        // down arrow
        console.log("Move Down");
        y = universe.move_vertical(
            BigInt(Math.floor(height / relativeMoveFactor))
        );
        console.log({ y });
    } else if (event.key == "a") {
        // left arrow
        console.log("Move Left");
        x = universe.move_horizontal(
            BigInt(-Math.floor(width / relativeMoveFactor))
        );
        console.log({ x });
    } else if (event.key == "d") {
        // right arrow
        console.log("Move Right");
        x = universe.move_horizontal(
            BigInt(Math.floor(width / relativeMoveFactor))
        );
        console.log({ x });
    } else {
        return;
    }
    requestAnimationFrame(() => {
        drawCells();
    });
});
