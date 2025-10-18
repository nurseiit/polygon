import {
	type MouseEventHandler,
	useCallback,
	useEffect,
	useMemo,
	useRef,
	useState,
} from "react";
import { Universe } from "../../wasm-library";
import { memory } from "../../wasm-library/index_bg.wasm";

const CELL_SIZE_PX = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

function App() {
	const [isPaused, setIsPaused] = useState(false);
	const isPausedRef = useRef<boolean>(false);
	const canvasRef = useRef<HTMLCanvasElement>(null);

	const universe = useMemo(() => Universe.new(), []);

	const togglePlayPause = useCallback(() => {
		isPausedRef.current = !isPaused;
		setIsPaused(!isPaused);
	}, [isPaused]);

	const drawGrid = useCallback(() => {
		const context = canvasRef.current?.getContext("2d");

		if (context == null) {
			return;
		}

		const width = universe.width();
		const height = universe.height();

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
	}, [universe]);

	const drawCells = useCallback(() => {
		const context = canvasRef.current?.getContext("2d");

		if (context == null) {
			return;
		}

		const width = universe.width();
		const height = universe.height();

		const getIdx = (row: number, col: number): number => row * width + col;

		const cellsPtr = universe.cells_ptr();
		const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

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
	}, [universe]);

	useEffect(() => {
		const canvas = canvasRef.current;

		if (canvas == null) {
			return;
		}

		const width = universe.width();
		const height = universe.height();

		canvas.height = (CELL_SIZE_PX + 1) * height + 1;
		canvas.width = (CELL_SIZE_PX + 1) * width + 1;

		const renderLoop = () => {
			if (!isPausedRef.current) {
				universe.tick();

				drawGrid();
				drawCells();
			}

			requestAnimationFrame(renderLoop);
		};

		requestAnimationFrame(renderLoop);
	}, [drawCells, drawGrid, universe]);

	const handleCanvasClick: MouseEventHandler<HTMLCanvasElement> = useCallback(
		(event) => {
			const canvas = canvasRef.current;

			if (canvas == null) {
				return;
			}

			const width = universe.width();
			const height = universe.height();

			const boundingRect = canvas.getBoundingClientRect();

			const scaleX = canvas.width / boundingRect.width;
			const scaleY = canvas.height / boundingRect.height;

			const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
			const canvasTop = (event.clientY - boundingRect.top) * scaleY;

			const row = Math.min(
				Math.floor(canvasTop / (CELL_SIZE_PX + 1)),
				height - 1,
			);
			const col = Math.min(
				Math.floor(canvasLeft / (CELL_SIZE_PX + 1)),
				width - 1,
			);

			universe.toggle_cell(row, col);

			drawGrid();
			drawCells();
		},
		[drawCells, drawGrid, universe],
	);

	return (
		<>
			<canvas ref={canvasRef} onClick={handleCanvasClick} />
			<button type="button" onClick={togglePlayPause}>
				⏯ {isPaused ? "Play" : "Pause"}
			</button>
		</>
	);
}

export default App;
