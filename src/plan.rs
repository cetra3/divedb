use anyhow::Result;
use dive_deco::{BuhlmannConfig, BuhlmannModel, DecoModel, DecoStageType, Depth, Gas, Time};

use crate::schema::{DiveMetric, DivePlanInput, DiveSchedule, DiveStage, GasOutput, StageType};

pub fn plan_dive(dive_plan: DivePlanInput) -> Result<DiveSchedule> {
    let mut gasses = dive_plan
        .deco_gasses
        .into_iter()
        .map(Gas::from)
        .collect::<Vec<_>>();

    let config = BuhlmannConfig::new()
        .with_gradient_factors(dive_plan.gf_low as u8, dive_plan.gf_high as u8);
    let mut model = BuhlmannModel::new(config);

    // 18m/min
    let descent_rate = 18.;

    let drop_to_bottom_time = dive_plan.depth / descent_rate;

    let back_gas = Gas::from(dive_plan.back_gas);

    gasses.push(back_gas);

    model.record_travel(
        Depth::from_meters(dive_plan.depth),
        Time::from_minutes(drop_to_bottom_time),
        &back_gas,
    );
    model.record(
        Depth::from_meters(dive_plan.depth),
        Time::from_minutes(dive_plan.time),
        &back_gas,
    );

    // calculate deco runtime providing available gasses
    let deco_runtime = model.deco(gasses).map_err(|err| anyhow::anyhow!("{err}"))?;

    let mut stages = Vec::new();

    let mut metrics = Vec::new();

    let mut cur_time = 0;

    metrics.push(DiveMetric {
        time: 0,
        depth: 0.,
        pressure: None,
        temperature: None,
        o2: None,
        he: None,
    });

    stages.push(DiveStage {
        stage_type: StageType::DESCEND,
        time: drop_to_bottom_time as i32,
        depth: dive_plan.depth,
        gas: Some(back_gas.into()),
    });

    cur_time += drop_to_bottom_time as i32 * 60;

    metrics.push(DiveMetric {
        time: cur_time,
        depth: dive_plan.depth,
        pressure: None,
        temperature: None,
        o2: None,
        he: None,
    });

    stages.push(DiveStage {
        stage_type: StageType::STAY,
        time: dive_plan.time,
        depth: dive_plan.depth,
        gas: None,
    });

    for _ in 0..dive_plan.time {
        cur_time += 60;

        metrics.push(DiveMetric {
            time: cur_time,
            depth: dive_plan.depth,
            pressure: None,
            temperature: None,
            o2: None,
            he: None,
        });
    }

    for stage in deco_runtime.deco_stages {
        let time = stage.duration.as_minutes() as i32;

        cur_time += stage.duration.as_seconds() as i32;

        let (o2, he) = if stage.stage_type == DecoStageType::GasSwitch {

            let gas = GasOutput::from(stage.gas);


            (Some(gas.o2), Some(gas.he))

        } else {
            (None, None)
        };

        metrics.push(DiveMetric {
            time: cur_time,
            depth: stage.end_depth.as_meters() as f32,
            pressure: None,
            temperature: None,
            o2,
            he,
        });

        if time == 0
            && (stage.stage_type == DecoStageType::Ascent
                || stage.stage_type == DecoStageType::DecoStop)
        {
            continue;
        }

        stages.push(DiveStage {
            stage_type: stage.stage_type.into(),
            time,
            depth: stage.end_depth.as_meters() as f32,
            gas: Some(stage.gas.into()),
        });
    }

    metrics.push(DiveMetric {
        time: cur_time,
        depth: 0.,
        pressure: None,
        temperature: None,
        o2: None,
        he: None,
    });

    let small_chart = crate::chart::render_dive(
        metrics.clone(),
        Some(BuhlmannModel::new(config)),
        Some(469.),
        Some(288.),
    )?;

    let big_chart = crate::chart::render_dive(
        metrics,
        Some(BuhlmannModel::new(config)),
        Some(958.),
        Some(400.),
    )?;

    Ok(DiveSchedule {
        runtime: deco_runtime.tts.as_minutes() as i32 + drop_to_bottom_time as i32 + dive_plan.time,
        tts: deco_runtime.tts.as_minutes() as i32,
        stages,
        small_chart,
        big_chart,
    })
}
