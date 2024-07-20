import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";
import { Universe } from "wasm-game-of-life";

const CELL_SIZE = 5 // pixels
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new_randomized();
const width = universe.width()
const height = universe.height()

const canvas = document.getElementById("game-of-life-canvas")
canvas.height = (CELL_SIZE + 1) * height + 1
canvas.width = (CELL_SIZE + 1) * width + 1

const ctx = canvas.getContext('2d')

let frameId = null

const tickSlider = document.getElementById("tick-slider")
const resetButton = document.getElementById("reset-uni-button")
const deadButton = document.getElementById("dead-uni-button")

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column
}

const isBitSet = (n, arr) => {
    const byte = Math.floor(n / 8)
    const mask = 1 << (n % 8)
    return (arr[byte] & mask) === mask
}

const drawCells = () => {
    const cellsPtr = universe.cells()
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8)

    ctx.beginPath();

    // Draw dead cells and alive cells in 2 separate loop in order to
    // enhance the performance where `fillStyle` is the bottleneck
    ctx.fillStyle = DEAD_COLOR
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col)
            if (isBitSet(idx, cells) !== true) {
                continue
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.fillStyle = ALIVE_COLOR
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col)
            if (isBitSet(idx, cells) !== false) {
                continue
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke()
}


const isPaused = () => {
    return frameId === null
}

const drawBoard = () => {
    drawGrid()
    drawCells()
}

const playPauseButton = document.getElementById("play-pause")

const play = () => {
    playPauseButton.textContent = "⏸"
    renderLoop()
}

const pause = () => {
    playPauseButton.textContent = "▶"
    cancelAnimationFrame(frameId)
    frameId = null;
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play()
    } else {
        pause()
    }
})

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect()

    const scaleX = canvas.width / boundingRect.width
    const scaleY = canvas.height / boundingRect.height

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX
    const canvasTop = (event.clientY - boundingRect.top) * scaleY

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1)
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1)

    universe.toggle_cell(row, col)

    drawBoard()
})

resetButton.addEventListener("click", event => {
    universe.reset_init_state()

    drawBoard()
})

deadButton.addEventListener("click", event => {
    universe.reset_cells()
    drawBoard()
})

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps")
        this.frames = []
        this.lastFrameTimeStamp = performance.now()
    }

    render() {
        const now = performance.now()
        const delta = now - this.lastFrameTimeStamp
        this.lastFrameTimeStamp = now
        const fps = 1 / delta * 1000

        this.frames.push(fps)
        if (this.frames.length > 100) {
            this.frames.shift()
        }

        let min = Infinity
        let max = -Infinity
        let sum = 0
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i]
            min = Math.min(this.frames[i], min)
            max = Math.max(this.frames[i], max)
        }
        let mean = sum / this.frames.length

        this.fps.textContent = `
    Frames per Second:
latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}`
            .trim()
    }
}

const renderLoop = () => {
    fps.render()

    const numOfTicks = parseInt(tickSlider.value, 10)
    // console.log(`num of ticks: ${numOfTicks}`)
    for (let i = 0; i < numOfTicks; i++) {
        universe.tick()
    }

    drawBoard()

    frameId = requestAnimationFrame(renderLoop)
}


drawBoard()
play()
