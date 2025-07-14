import { useCallback, useState, type ChangeEventHandler } from "react";
import { greet } from "@workspace/wasm-library";

import "./App.css";

function App() {
	const [value, setValue] = useState("");

	const handleGreet = useCallback(() => greet(value), [value]);

	const handleChange: ChangeEventHandler<HTMLInputElement> = useCallback(
		({ currentTarget }) => setValue(currentTarget.value),
		[],
	);
	return (
		<>
			<input value={value} onChange={handleChange} />
			<button onClick={handleGreet}>Greet {value}</button>
		</>
	);
}

export default App;
