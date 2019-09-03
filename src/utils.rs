use std::path::{Path, PathBuf};
use std::fs;
use std::env;

use failure::{Error, Fail};
use gwasm_api::{Blob, TaskResult, TaskInput};
use structopt::StructOpt;
use std::iter::FromIterator;


pub trait MapReduce {

    type ExecuteInput: TaskInput;
    type ExecuteOutput: TaskInput;

    fn split(args: &Vec<String>) -> Vec<Self::ExecuteInput>;
    fn execute(params: Self::ExecuteInput) -> Self::ExecuteOutput;
    fn merge(args: &Vec<String>, subtasks_result: &TaskResult<Self::ExecuteInput, Self::ExecuteOutput>);
}

#[derive(Debug, Fail)]
pub enum ApiError {
    #[fail(display = "Can't find parent")]
    NoParent,
}


pub fn save_params_vec<SplitOutputType : TaskInput>(output_file: &Path, split_params: &Vec<SplitOutputType>) -> Result<(), Error> {
    let json_params: Vec<serde_json::Value> = split_params.iter().map(TaskInput::pack_task).collect();
    save_json(output_file, &serde_json::json!(json_params))
}

pub fn save_params<SplitOutputType : TaskInput>(output_file: &Path, split_params: &SplitOutputType) -> Result<(), Error> {
    let json: serde_json::Value = split_params.pack_task();
    save_json(output_file, &json)
}

pub fn save_json(output_file: &Path, json: &serde_json::Value) -> Result<(), Error> {

    let work_dir = output_file.parent().ok_or(ApiError::NoParent)?;
    fs::create_dir_all(work_dir)?;

    fs::write(output_file, serde_json::to_string_pretty(&json)?)?;
    Ok(())
}

pub fn load_params<MapReduceType: MapReduce>(params_path: &Path) -> MapReduceType::ExecuteOutput {
    unimplemented!()
}

pub fn dispatch_and_run_command<MapReduceType: MapReduce>() {
    let mut args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    args.drain(0..1);

    if command == "split" {
        split_step::<MapReduceType>(&args);
    }
    else if command == "execute" {
        execute_step::<MapReduceType>(&args);
    }
    else if command == "merge" {
        merge_step::<MapReduceType>(&args);
    }
    else {
        panic!("Command not specified.")
    }
}


pub fn split_step<MapReduceType: MapReduce>(args: &Vec<String>){

    let work_dir = PathBuf::from(&args[0]);
    let split_params = MapReduceType::split(&Vec::from_iter(args[1..].iter().cloned()));

    let split_out_path = work_dir.join("tasks.json");
    save_params_vec(&split_out_path, &split_params).unwrap();
}

pub fn execute_step<MapReduceType: MapReduce>(args: &Vec<String>) {

    let params_path = PathBuf::from(args[0].clone());
    let output_desc_path = PathBuf::from(args[1].clone());

    let output = load_params::<MapReduceType>(&output_desc_path);

    save_params(&output_desc_path, &output);
}

pub fn merge_step<MapReduceType: MapReduce>(args: &Vec<String>) {

}

