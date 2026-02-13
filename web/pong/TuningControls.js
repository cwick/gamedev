export const PARAM = Object.freeze({
  PADDLE_WIDTH: 0,
  PADDLE_HEIGHT: 1,
  PADDLE_SPEED: 2,
  BALL_SPEED: 3,
  AI_REACTION_SPEED: 4,
});

export const controls = [
  {
    label: "Paddle Width",
    paramId: PARAM.PADDLE_WIDTH,
    min: 5,
    max: 40,
    step: 1,
    decimals: 1,
  },
  {
    label: "Paddle Height",
    paramId: PARAM.PADDLE_HEIGHT,
    min: 40,
    max: 200,
    step: 1,
    decimals: 1,
  },
  {
    label: "Paddle Speed",
    paramId: PARAM.PADDLE_SPEED,
    min: 100,
    max: 1000,
    step: 10,
    decimals: 1,
  },
  {
    label: "Ball Speed",
    paramId: PARAM.BALL_SPEED,
    min: 100,
    max: 1000,
    step: 10,
    decimals: 1,
  },
  {
    label: "AI Reaction Speed",
    paramId: PARAM.AI_REACTION_SPEED,
    min: 50,
    max: 800,
    step: 10,
    decimals: 1,
  },
];
