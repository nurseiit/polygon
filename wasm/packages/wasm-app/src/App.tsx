import { greet } from "@workspace/wasm-library";
import "./App.css";

function App() {
	return <button onClick={greet}>Greet</button>;
}

export default App;
