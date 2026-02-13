import { html } from "preact";
import {
  engine_get_tuning_param,
  engine_reset_tuning_defaults,
  engine_set_tuning_param,
  engine_tuning_schema_version,
} from "../dist/gamedev_wasm_hello.js";
import TuningPanel from "./TuningPanel.js";
import { controls } from "./TuningControls.js";

const App = () => {
  return html`<${TuningPanel}
    title="Pong Tuning"
    schemaVersion=${engine_tuning_schema_version()}
    controls=${controls}
    engine_get_tuning_param=${engine_get_tuning_param}
    engine_set_tuning_param=${engine_set_tuning_param}
    engine_reset_tuning_defaults=${engine_reset_tuning_defaults}
  />`;
};

export default App;
