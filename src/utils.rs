use std::path::{Path, PathBuf};
use std::fs;
use std::env;

use failure::{Error};
use gwasm_api::{Blob, TaskResult, TaskInput};
use structopt::StructOpt;


pub trait MapReduce {

    type ExecuteInput: TaskInput;
    type ExecuteOutput;

    fn split(args: &Vec<String>) -> Vec<Self::ExecuteInput>;
    fn execute(params: Self::ExecuteInput) -> Self::ExecuteOutput;
    fn merge(args: &Vec<String>, subtasks_result: &TaskResult<Self::ExecuteInput, Self::ExecuteOutput>);
}

pub fn save_params<SplitOutputType : TaskInput>(output_dir: &Path, split_params: &Vec<SplitOutputType>) -> Result<(), Error> {
    let json_params: Vec<serde_json::Value> = split_params.iter().map(TaskInput::pack_task).collect();

    fs::create_dir_all(output_dir)?;

    let output_file = output_dir.join("params.json");
    fs::write(output_file, serde_json::to_string_pretty(&json_params)?)?;

    Ok(())
}

pub fn dispatch_and_run_command<MapReduceType: MapReduce>() {
    let mut args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let work_dir = PathBuf::from(args[2].clone());

    args.drain(0..2);

    if command == "split" {
        split_step::<MapReduceType>(&work_dir, &args);
    }
    else if command == "execute" {
        execute_step::<MapReduceType>(&work_dir, &args);
    }
    else if command == "merge" {
        panic!("Not implemented")
    }
    else {
        panic!("Command not specified.")
    }
}


pub fn split_step<MapReduceType: MapReduce>(work_dir: &Path, args: &Vec<String>) -> Vec<MapReduceType::ExecuteInput> {

    let split_params = MapReduceType::split(args);
    save_params(Path::new("results/split/"), &split_params).unwrap();

    return split_params;
}

pub fn execute_step<MapReduceType: MapReduce>(work_dir: &Path, args: &Vec<String>) {

    let params_path = args[0].clone();


}

pub fn merge_step<MapReduceType: MapReduce>(work_dir: &Path, args: &Vec<String>) {

}

