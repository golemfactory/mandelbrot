use std::path::{Path};
use std::fs;

use failure::{Error};
use gwasm_api::{Blob, TaskResult, TaskInput};
use structopt::StructOpt;


pub trait MapReduce<SplitArgs, ExecuteInput: TaskInput, ExecuteOutput> {

    fn split(args: &SplitArgs) -> Vec<ExecuteInput>;
    fn execute(params: ExecuteInput) -> ExecuteOutput;
    fn merge(args: &SplitArgs, subtasks_result: &TaskResult<ExecuteInput, ExecuteOutput>);
}



pub fn save_params<SplitOutputType : TaskInput>(output_dir: &Path, split_params: &Vec<SplitOutputType>) -> Result<(), Error> {

    let json_params: Vec<serde_json::Value> = split_params.iter().map(TaskInput::pack_task).collect();

    fs::create_dir_all(output_dir)?;

    let mut subtask_num: u32 = 0;
    for subtask_params in json_params {
        let output_file = output_dir.join(format!("params-{}.json", subtask_num));
        fs::write(output_file, serde_json::to_string_pretty(&subtask_params)?)?;

        subtask_num += 1;
    }
    Ok(())
}

pub fn save_params2<SplitOutputType : TaskInput>(output_dir: &Path, split_params: &Vec<SplitOutputType>) -> Result<(), Error> {
    let json_params: Vec<serde_json::Value> = split_params.iter().map(TaskInput::pack_task).collect();

    fs::create_dir_all(output_dir)?;

    let output_file = output_dir.join("params.json");
    fs::write(output_file, serde_json::to_string_pretty(&json_params)?)?;

    Ok(())
}

pub fn split_step<SplitArgs: StructOpt, ExecuteInput: TaskInput, ExecuteOutput, MapReduceType: MapReduce<SplitArgs, ExecuteInput, ExecuteOutput>>() -> Vec<ExecuteInput> {
    let opt = SplitArgs::from_args();

    // Split step.
    let split_params = MapReduceType::split(&opt);
    save_params2(Path::new("results/split/"), &split_params).unwrap();

    return split_params;
}