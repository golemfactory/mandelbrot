pub mod utils;
pub mod png_utils;
pub mod mandelbrot;

use structopt::*;

use gwasm_api::{Blob, TaskResult, TaskInput};

use crate::utils::MapReduce;
use crate::mandelbrot::{MandelbrotParams, Mandelbrot, ExecuteParams};



fn main() {

    let split_params = utils::split_step::<Mandelbrot>();

    // Execute step for all subtasks.

    // Temporary
    let opt = MandelbrotParams::from_args();

    let mut results = Vec::new();
    for subtask_params in split_params.into_iter() {
        results.push((subtask_params.clone(), Mandelbrot::execute(subtask_params)));
    }

    // Merge step.
    Mandelbrot::merge(&opt, &results);
}
