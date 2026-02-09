import { html, useEffect, useState } from "preact";

let debugUpdateFn = null;

export function setDebugState(newState) {
  if (debugUpdateFn) {
    debugUpdateFn(newState);
  }
}

const DebugRow = ({ label, value, isBoolean = false }) => {
  const modifiers = isBoolean
    ? `debug__value--bool debug__value--${value ? "true" : "false"}`
    : "";
  const displayValue = isBoolean ? (value ? "YES" : "NO") : value;

  return html`
    <div class="debug__row">
      <span class="debug__label">${label}</span>
      <span class="debug__value ${modifiers}">${displayValue}</span>
    </div>
  `;
};

const DebugPanel = ({ gameState }) => {
  if (!gameState) {
    return html`
      <div class="debug">
        <div class="debug__empty">
          <p>Waiting for game to initialize...</p>
        </div>
      </div>
    `;
  }

  const getGamePhaseLabel = (phase) => {
    if (phase === 0) return "Playing";
    if (phase === 1) return "Game Over";
    return "Unknown";
  };

  return html`
    <div class="debug">
      <div class="debug__section">
        <h3 class="debug__title">Ball</h3>
        <${DebugRow} label="X" value=${gameState.ball_x.toFixed(1)} />
        <${DebugRow} label="Y" value=${gameState.ball_y.toFixed(1)} />
      </div>

      <div class="debug__section">
        <h3 class="debug__title">Paddles</h3>
        <${DebugRow} label="P1 Y" value=${gameState.paddle1_y.toFixed(1)} />
        <${DebugRow} label="P2 Y" value=${gameState.paddle2_y.toFixed(1)} />
      </div>

      <div class="debug__section">
        <h3 class="debug__title">Score</h3>
        <${DebugRow} label="P1" value=${gameState.p1_score} />
        <${DebugRow} label="P2" value=${gameState.p2_score} />
      </div>

      <div class="debug__section">
        <h3 class="debug__title">Game State</h3>
        <${DebugRow}
          label="Phase"
          value=${getGamePhaseLabel(gameState.game_phase)}
        />
      </div>
    </div>
  `;
};

const App = () => {
  const [gameState, setGameState] = useState(null);

  // Store the setter function for external updates
  useEffect(() => {
    debugUpdateFn = setGameState;
    return () => {
      debugUpdateFn = null;
    };
  }, []);

  return html`
    <div class="app">
      <div class="app__card">
        <h1 class="app__title">🦀 Rust WebAssembly Hello World</h1>

        <div class="app__status">
          <strong>Status:</strong>
          <span id="status" class="app__status-value"
            >Loading WebAssembly module...</span
          >
        </div>

        <canvas
          id="gameCanvas"
          class="app__canvas"
          width="800"
          height="600"
        ></canvas>
      </div>

      <${DebugPanel} gameState=${gameState} />
    </div>
  `;
};

export default App;
