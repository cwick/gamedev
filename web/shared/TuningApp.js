import { html } from "preact";
import {
  engine_get_tuning_param,
  engine_reset_tuning_defaults,
  engine_set_tuning_param,
} from "../dist/gamedev_wasm_hello.js";
import TuningPanel from "./TuningPanel.js";

const TuningApp = ({ controls }) => {
  return html`<${TuningPanel}
    controls=${controls}
    engine_get_tuning_param=${engine_get_tuning_param}
    engine_set_tuning_param=${engine_set_tuning_param}
    engine_reset_tuning_defaults=${engine_reset_tuning_defaults}
  />`;
};

export default TuningApp;
