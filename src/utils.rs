use std::path::{Path, PathBuf};
use std::fs;
use std::env;

use failure::{Error, Fail};
use gwasm_api::{Blob, TaskResult, TaskInput};
use std::iter::FromIterator;
use structopt::StructOpt;


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
    #[fail(display = "Expected -- separator.")]
    NoSeparator,
    #[fail(display = "No such command {}.", command)]
    NoCommand {
        command: String,
    },
    #[fail(display = "Invalid params format: {}.", message)]
    InvalidParamsFormat {
        message: String
    },
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

pub fn load_params<MapReduceType: MapReduce>(params_path: &Path) -> Result<MapReduceType::ExecuteInput, Error> {
    let content = fs::read_to_string(params_path)?;
    let json = serde_json::from_str(&content)?;

    load_params_json::<MapReduceType>(json)
}

pub fn load_params_json<MapReduceType: MapReduce>(json: serde_json::Value) -> Result<MapReduceType::ExecuteInput, Error> {
    if !json.is_array() {
        Err(ApiError::InvalidParamsFormat{ message: String::from("Top level array not found") })?
    }

    MapReduceType::ExecuteInput::from_json(json)
}

pub fn dispatch_and_run_command<MapReduceType: MapReduce>() -> Result<(), Error> {
    let mut args: Vec<String> = env::args().collect();
    // TODO: check param len
    let command = args[1].clone();

    // Remove program name and command.
    args.drain(0..2);

    if command == "split" {
        split_step::<MapReduceType>(&args)
    }
    else if command == "execute" {
        execute_step::<MapReduceType>(&args)
    }
    else if command == "merge" {
        merge_step::<MapReduceType>(&args)
    }
    else {
        Err(ApiError::NoCommand{ command })?
    }
}


pub fn split_step<MapReduceType: MapReduce>(args: &Vec<String>) -> Result<(), Error> {

    // TODO: check param len
    let work_dir = PathBuf::from(&args[0]);
    let split_params = MapReduceType::split(&Vec::from_iter(args[1..].iter().cloned()));

    let split_out_path = work_dir.join("tasks.json");
    save_params_vec(&split_out_path, &split_params)
}

pub fn execute_step<MapReduceType: MapReduce>(args: &Vec<String>) -> Result<(), Error>  {

    let params_path = PathBuf::from(args[0].clone());
    let output_desc_path = PathBuf::from(args[1].clone());

    let input_params = load_params::<MapReduceType>(&params_path)?;
    let output_desc = MapReduceType::execute(input_params);

    save_params(&output_desc_path, &output_desc)
}

pub fn merge_step<MapReduceType: MapReduce>(args: &Vec<String>) -> Result<(), Error>  {

    let tasks_params = PathBuf::from(args[0].clone());
    let tasks_outputs = PathBuf::from(args[1].clone());

//    if args[2] != "--" {
//        return Err(ApiError::NoSeparator)?;
//    }


    Ok(())
}

pub fn parse_args<ArgsType: StructOpt>(args: &Vec<String>) -> ArgsType {
    // Note: StructOpt wants to eat first argument as program name.
    // But we don't have program name at this point, so we must add fake argument.
    let mut args_copy = args.clone();
    args_copy.insert(0, String::from("--"));

    return ArgsType::from_iter(args_copy.into_iter());
}


