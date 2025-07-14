import { greet } from "wasm";
import "./App.css";

function App() {
	return <button onClick={greet}>Greet</button>;
}

export default App;
