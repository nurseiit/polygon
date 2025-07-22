import { useEffect, useRef } from "react";
import { Universe } from "../../wasm-library";

function App() {
	const canvasRef = useRef<HTMLPreElement>(null);

	useEffect(() => {
		const universe = Universe.new();

		const renderLoop = () => {
			if (canvasRef.current == null) {
				return;
			}

			canvasRef.current.textContent = universe.render();
			universe.tick();

			requestAnimationFrame(renderLoop);
		};

		requestAnimationFrame(renderLoop);
	}, []);

	return (
		<>
			<pre ref={canvasRef} />
		</>
	);
}

export default App;
