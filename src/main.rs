pub mod utils;
pub mod png_utils;
pub mod mandelbrot;

use crate::utils::MapReduce;
use crate::mandelbrot::{Mandelbrot};



fn main() {

    // Run MapReduce task defined in Mandelbrot struct.
    // This function will parse command line arguments and dispatch it
    // to one of split, execute and merge functions.
    utils::dispatch_and_run_command::<Mandelbrot>();
}