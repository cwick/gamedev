import { html, useState } from "preact";

const TuningControl = ({
  spec,
  engine_get_tuning_param,
  engine_set_tuning_param,
}) => {
  const [, setTrigger] = useState(0);

  const currentValue = engine_get_tuning_param(spec.paramId);
  const isReadOnly = spec.readOnly;

  const handleRangeChange = (e) => {
    const parsed = Number(e.target.value);
    if (Number.isFinite(parsed)) {
      engine_set_tuning_param(spec.paramId, parsed);
      setTrigger((t) => t + 1);
    }
  };

  const handleNumberChange = (e) => {
    const parsed = Number(e.target.value);
    if (Number.isFinite(parsed)) {
      const status = engine_set_tuning_param(spec.paramId, parsed);
      setTrigger((t) => t + 1);
    }
  };

  const displayValue = Number.isFinite(currentValue)
    ? currentValue.toFixed(spec.decimals ?? 2)
    : "N/A";

  return html`
    <div class="debug__control">
      <div class="debug__row">
        <span class="debug__label">${spec.label}</span>
        <span class="debug__value">${displayValue}</span>
      </div>
      ${!isReadOnly &&
      html`
        <input
          type="range"
          min=${spec.min}
          max=${spec.max}
          step=${spec.step}
          value=${currentValue}
          onInput=${handleRangeChange}
          class="debug__slider"
        />
      `}
    </div>
  `;
};

const TuningPanel = ({
  controls,
  engine_get_tuning_param,
  engine_set_tuning_param,
  engine_reset_tuning_defaults,
}) => {
  const [refreshTrigger, setRefreshTrigger] = useState(false);

  const handleReset = () => {
    engine_reset_tuning_defaults();
    setRefreshTrigger((prev) => !prev);
  };

  return html`
    <div class="debug">
      <div class="debug__section">
        <h3 class="debug__title">Arkanoid Tuning</h3>
      </div>

      <div class="debug__section">
        ${controls.map(
          (spec) => html`
            <${TuningControl}
              key=${spec.paramId}
              spec=${spec}
              engine_get_tuning_param=${engine_get_tuning_param}
              engine_set_tuning_param=${engine_set_tuning_param}
            />
          `,
        )}
      </div>

      <div class="debug__section">
        <button onClick=${handleReset} type="button" class="debug__button">
          Reset Defaults
        </button>
      </div>
    </div>
  `;
};

export default TuningPanel;
