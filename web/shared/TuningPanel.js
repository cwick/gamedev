import { html, useState, useEffect, useCallback } from "preact";

const TuningControl = ({ spec, currentValue, onValueChange }) => {
  const isReadOnly = spec.readOnly;

  const handleRangeChange = (e) => {
    const parsed = Number(e.target.value);
    if (Number.isFinite(parsed)) {
      onValueChange(parsed);
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
  title,
  controls,
  engine_get_tuning_param,
  engine_set_tuning_param,
  engine_reset_tuning_defaults,
}) => {
  const fetchControlValues = useCallback(() => {
    const values = {};
    controls.forEach((spec) => {
      values[spec.paramId] = engine_get_tuning_param(spec.paramId);
    });
    return values;
  }, [controls, engine_get_tuning_param]);

  const [controlValues, setControlValues] = useState(fetchControlValues);

  useEffect(() => {
    const interval = setInterval(() => {
      setControlValues(fetchControlValues());
    }, 100);

    return () => clearInterval(interval);
  }, [fetchControlValues]);

  return html`
    <div class="debug">
      <div class="debug__section">
        <h3 class="debug__title">${title}</h3>
      </div>

      <div class="debug__section">
        ${controls.map(
          (spec) => html`
            <${TuningControl}
              key=${spec.paramId}
              spec=${spec}
              currentValue=${controlValues[spec.paramId]}
              onValueChange=${(value) =>
                engine_set_tuning_param(spec.paramId, value)}
            />
          `,
        )}
      </div>

      ${controls.some((spec) => !spec.readOnly) &&
      html`
        <div class="debug__section">
          <button
            onClick=${engine_reset_tuning_defaults}
            type="button"
            class="debug__button"
          >
            Reset Defaults
          </button>
        </div>
      `}
    </div>
  `;
};

export default TuningPanel;
