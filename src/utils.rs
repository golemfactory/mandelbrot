use std::path::{Path, PathBuf};
use std::fs;
use std::env;

use failure::{Error};
use gwasm_api::{Blob, TaskResult, TaskInput};
use structopt::StructOpt;
use std::iter::FromIterator;


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

    save_params(&work_dir, &split_params).unwrap();
}

pub fn execute_step<MapReduceType: MapReduce>(args: &Vec<String>) {

    let params_path = args[0].clone();


}

pub fn merge_step<MapReduceType: MapReduce>(args: &Vec<String>) {

}

