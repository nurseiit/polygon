import { useEffect, useRef } from "react";
import { Universe } from "../../wasm-library";
import { memory } from "../../wasm-library/index_bg.wasm";

const CELL_SIZE_PX = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

function App() {
	const canvasRef = useRef<HTMLCanvasElement>(null);

	useEffect(() => {
		if (canvasRef.current == null) {
			return;
		}

		const universe = Universe.new();
		const width = universe.width();
		const height = universe.height();

		canvasRef.current.height = (CELL_SIZE_PX + 1) * height + 1;
		canvasRef.current.width = (CELL_SIZE_PX + 1) * width + 1;

		const context = canvasRef.current.getContext("2d");

		const drawGrid = () => {
			if (context == null) {
				return;
			}

			context.beginPath();
			context.strokeStyle = GRID_COLOR;

			// vertical lines
			for (let i = 0; i <= width; i += 1) {
				context.moveTo(i * (CELL_SIZE_PX + 1) + 1, 0);
				context.lineTo(
					i * (CELL_SIZE_PX + 1) + 1,
					height * (CELL_SIZE_PX + 1) + 1,
				);
			}

			// horizontal lines
			for (let i = 0; i <= height; i += 1) {
				context.moveTo(0, i * (CELL_SIZE_PX + 1) + 1);
				context.lineTo(
					width * (CELL_SIZE_PX + 1) + 1,
					i * (CELL_SIZE_PX + 1) + 1,
				);
			}

			context.stroke();
		};

		const drawCells = () => {
			if (context == null) {
				return;
			}

			const getIdx = (row: number, col: number): number => row * width + col;

			const cellsPtr = universe.cells_ptr();
			const cells = new Uint8Array(
				memory.buffer,
				cellsPtr,
				(width * height) / 8,
			);

			const isAlive = (idx: number): boolean => {
				const byte = idx >> 3;
				const bit = idx % 8;
				return Boolean(cells[byte] & (1 << bit));
			};

			context.beginPath();

			for (let row = 0; row < height; row += 1) {
				for (let col = 0; col < width; col += 1) {
					const idx = getIdx(row, col);

					context.fillStyle = isAlive(idx) ? ALIVE_COLOR : DEAD_COLOR;
					context.fillRect(
						col * (CELL_SIZE_PX + 1) + 1,
						row * (CELL_SIZE_PX + 1) + 1,
						CELL_SIZE_PX,
						CELL_SIZE_PX,
					);
				}
			}

			context.stroke();
		};

		const renderLoop = () => {
			drawGrid();
			drawCells();

			universe.tick();

			requestAnimationFrame(renderLoop);
		};

		requestAnimationFrame(renderLoop);
	}, []);

	return <canvas ref={canvasRef} />;
}

export default App;
