// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/fractal_rs_bg";
import { Universe } from "wasm-game-of-life";

let universe;
const canvas = document.getElementById("game-of-life-canvas");
let zoomFactor = 1.0;
let x = 0;
let y = 0;
let width;
let height;

const generateUniverse = () => {
    width = window.innerWidth;
    height = window.innerHeight;
    console.log("Generating new universe", { width, height, x, y, zoomFactor });
    universe = Universe.new(width, height, y, x, zoomFactor);

    canvas.height = height;
    canvas.width = width;
};

const drawCells = () => {
    const cellsRPtr = universe.cells_r();
    const cellsR = new Uint8Array(memory.buffer, cellsRPtr, width * height);

    const cellsGPtr = universe.cells_g();
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

addEventListener("resize", render);

addEventListener("keyup", (event) => {
    console.log("Keypress", event);
    if (event.key === "+") {
        console.log("Zoom in");
        zoomFactor = universe.zoom_in();
        console.log({ zoomFactor });
    }
    else if (event.key === "-") {
        console.log("Zoom out");
        zoomFactor = universe.zoom_out();
        console.log({ zoomFactor });
    }
    else if (event.key == 'w') {
        console.log("Move Up");
        y = universe.move_up();
        console.log({ y });
    }
    else if (event.key == 's') {
        // down arrow
        console.log("Move Down");
        y = universe.move_down();
        console.log({ y });
    }
    else if (event.key == 'a') {
       // left arrow
         console.log("Move Left");
        x = universe.move_left();
        console.log({ x });
    }
    else if (event.key == 'd') {
       // right arrow
            console.log("Move Right");
        x = universe.move_right();
        console.log({ x });
    }
    else {
        return;
    }
    universe.update();
    requestAnimationFrame(() => {
        drawCells();
    });
});
