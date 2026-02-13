export const PARAM = Object.freeze({
  BALL_X: 0,
  BALL_Y: 1,
  BALL_VX: 2,
  BALL_VY: 3,
});

export const controls = [
  {
    label: "Ball X",
    paramId: PARAM.BALL_X,
    decimals: 1,
    readOnly: true,
  },
  {
    label: "Ball Y",
    paramId: PARAM.BALL_Y,
    decimals: 1,
    readOnly: true,
  },
  {
    label: "Ball Velocity X",
    paramId: PARAM.BALL_VX,
    decimals: 1,
    readOnly: true,
  },
  {
    label: "Ball Velocity Y",
    paramId: PARAM.BALL_VY,
    decimals: 1,
    readOnly: true,
  },
];
