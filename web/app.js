import { html, useEffect, useState } from "preact";

let debugUpdateFn = null;

export function setDebugState(newState) {
  if (debugUpdateFn) {
    debugUpdateFn(newState);
  }
}

const DebugDisplay = ({ gameState }) => {
  return html`
    <div id="output">
      <p><strong>Game Running</strong></p>
      <p>
        Ball position: (${gameState.ball_x.toFixed(1)},
        ${gameState.ball_y.toFixed(1)})
      </p>
      <p>
        Paddle 1 Y: ${gameState.paddle1_y.toFixed(1)} | Paddle 2 Y:
        ${gameState.paddle2_y.toFixed(1)}
      </p>
      <p>
        Score: P1=${gameState.p1_score} | P2=${gameState.p2_score} |
        Phase=${gameState.game_phase} | Len=${gameState.stateLen}
      </p>
      <p>Controls: W/ArrowUp (up) | S/ArrowDown (down)</p>
    </div>
  `;
};

const App = () => {
  const [gameState, setGameState] = useState({
    ball_x: 0,
    ball_y: 0,
    paddle1_y: 0,
    paddle2_y: 0,
    p1_score: 0,
    p2_score: 0,
    game_phase: 0,
    stateLen: 0,
  });

  // Store the setter function for external updates
  useEffect(() => {
    debugUpdateFn = setGameState;
    return () => {
      debugUpdateFn = null;
    };
  }, []);

  return html`
    <div class="container">
      <h1>🦀 Rust WebAssembly Hello World</h1>

      <div class="info">
        <strong>Status:</strong>
        <span id="status">Loading WebAssembly module...</span>
      </div>

      <${DebugDisplay} gameState=${gameState} />

      <canvas
        id="gameCanvas"
        width="800"
        height="600"
        style="border: 1px solid #666; margin-top: 20px; display: block; max-width: 100%; background: #000;"
      ></canvas>
    </div>
  `;
};

export default App;
