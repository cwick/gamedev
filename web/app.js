import { html, useEffect, useState } from "preact";

let debugUpdateFn = null;

export function setDebugState(newState) {
  if (debugUpdateFn) {
    debugUpdateFn(newState);
  }
}

const DebugRow = ({ label, value, isBoolean = false }) => {
  const className = isBoolean ? `debug-bool ${value ? "true" : "false"}` : "";
  const displayValue = isBoolean ? (value ? "YES" : "NO") : value;

  return html`
    <div class="debug-row">
      <span class="debug-label">${label}</span>
      <span class="debug-value ${className}">${displayValue}</span>
    </div>
  `;
};

const DebugPanel = ({ gameState }) => {
  if (!gameState) {
    return html`
      <div class="debug-panel">
        <div style="padding: 20px; text-align: center;">
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
    <div class="debug-panel">
      <div class="debug-section">
        <h3>⚫ Ball</h3>
        <${DebugRow} label="X" value=${gameState.ball_x.toFixed(1)} />
        <${DebugRow} label="Y" value=${gameState.ball_y.toFixed(1)} />
      </div>

      <div class="debug-section">
        <h3>🎾 Paddles</h3>
        <${DebugRow} label="P1 Y" value=${gameState.paddle1_y.toFixed(1)} />
        <${DebugRow} label="P2 Y" value=${gameState.paddle2_y.toFixed(1)} />
      </div>

      <div class="debug-section">
        <h3>📊 Score</h3>
        <${DebugRow} label="P1" value=${gameState.p1_score} />
        <${DebugRow} label="P2" value=${gameState.p2_score} />
      </div>

      <div class="debug-section">
        <h3>🎮 Game State</h3>
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
    <div>
      <div class="container">
        <h1>🦀 Rust WebAssembly Hello World</h1>

        <div class="info">
          <strong>Status:</strong>
          <span id="status">Loading WebAssembly module...</span>
        </div>

        <canvas
          id="gameCanvas"
          width="800"
          height="600"
          style="border: 1px solid #666; margin-top: 20px; display: block; max-width: 100%; background: #000;"
        ></canvas>
      </div>

      <${DebugPanel} gameState=${gameState} />
    </div>
  `;
};

export default App;
