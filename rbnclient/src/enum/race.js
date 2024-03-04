// 赛道数据，包含赛道名称，赛道id，赛道类型
export let stageList = [
  {
    stage: "hy",
    stage_id: 1,
    stage_type: "tarmac",
  },
  {
    stage: "hy2",
    stage_id: 2,
    stage_type: "tarmac",
  },
];
// 车辆数据，包含车辆名称，车辆id
export let carList = [
  {
    car: "car1",
    car_id: 1,
  },
  {
    car: "car2",
    car_id: 2,
  },
];
// 车辆损坏数据枚举，包含车辆损坏名称，车辆损坏id
export let damageList = [
  {
    label: "real",
    value: 1,
  },
  {
    label: "sim",
    value: 2,
  },
];

// 湿滑路面数据枚举，包含湿滑路面名称，湿滑路面id
export let wetnessList = [
  {
    label: "wet",
    value: 1,
  },
  {
    label: "dry",
    value: 2,
  },
];

// 天气数据枚举，包含天气名称，天气id
export let weatherList = [
  {
    label: "sunny",
    value: 1,
  },
  {
    label: "rain",
    value: 2,
  },
];

// 天气类型
export let skytypeList = [
  {
    skytype: "sa",
    skytype_id: 1,
  },
  {
    skytype: "sb",
    skytype_id: 2,
  },
]

export const updateStageList = (list) => {
  stageList = list;
}

export const getStageList = () => {
  return stageList;
}