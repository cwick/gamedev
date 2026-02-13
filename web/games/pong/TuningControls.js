export const PARAM = Object.freeze({
  BALL_X: 0,
  BALL_Y: 1,
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
];
