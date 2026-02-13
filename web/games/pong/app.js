import { html } from "preact";
import TuningApp from "../../shared/TuningApp.js";
import { controls } from "./TuningControls.js";

const App = () => {
  return html`<${TuningApp} title="Pong Tuning" controls=${controls} />`;
};

export default App;
