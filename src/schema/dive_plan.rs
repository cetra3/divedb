use async_graphql::*;
use dive_deco::{DecoStageType, Gas};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct DivePlanInput {
    pub time: i32,
    pub depth: f32,
    pub gf_low: usize,
    pub gf_high: usize,
    pub back_gas: GasInput,
    pub deco_gasses: Vec<GasInput>,
}

#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
pub struct GasInput {
    pub litres: f32,
    pub o2: f32,
    pub he: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct GasOutput {
    pub o2: f32,
    pub he: f32,
}

impl From<Gas> for GasOutput {
    fn from(value: Gas) -> Self {
        let pp = value.gas_pressures_compound(1.);

        Self {
            o2: pp.o2 as f32 * 100.,
            he: pp.he as f32 * 100.,
        }
    }
}

impl From<GasInput> for Gas {
    fn from(val: GasInput) -> Self {
        Gas::new((val.o2 / 100.) as f64, (val.he / 100.) as f64)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct DiveSchedule {
    pub runtime: i32,
    pub tts: i32,
    pub stages: Vec<DiveStage>,
    pub small_chart: String,
    pub big_chart: String
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct DiveStage {
    pub stage_type: StageType,
    pub time: i32,
    pub depth: f32,
    pub gas: Option<GasOutput>,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
pub enum StageType {
    ASCEND,
    DESCEND,
    STAY,
    GAS_CHANGE,
}


impl From<DecoStageType> for StageType {
    fn from(value: DecoStageType) -> Self {
        match value {
            DecoStageType::Ascent => Self::ASCEND,
            DecoStageType::DecoStop => Self::STAY,
            DecoStageType::GasSwitch => Self::GAS_CHANGE,
        }
    }
}
