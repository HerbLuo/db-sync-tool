export type Id = string;

const CHARSET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
  .split("");

const base62Encode = (int: number) => {
  if (int === 0) {
    return CHARSET[0];
  }

  let res = "";
  while (int > 0) {
    res = CHARSET[int % 62] + res;
    int = Math.floor(int / 62);
  }
  return res;
};

// 用于生成uuid
const SS4 = () => base62Encode(Math.floor((1 + Math.random()) * 14776335)).substring(1);

const generate16LengthId: () => Id =
  () => [...Array(4)].map(SS4).join("");
const generate32LengthId: () => Id =
  () => [...Array(8)].map(SS4).join("");
const generate64LengthId: () => Id =
  () => [...Array(16)].map(SS4).join("");
const generate255LengthId: () => Id =
  () => [...Array(64)].map(SS4).join("").slice(1);

export {
  generate16LengthId,
  generate32LengthId,
  generate64LengthId,
  generate255LengthId
};
