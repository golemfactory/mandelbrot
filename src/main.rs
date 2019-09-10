pub mod utils;
pub mod png_utils;
pub mod mandelbrot;

use gwasm_api::dispatcher;
use crate::mandelbrot::{Mandelbrot};



fn main() {

    // Run MapReduce task defined in Mandelbrot struct.
    // This function will parse command line arguments and dispatch it
    // to one of split, execute and merge functions.
    dispatcher::run(&Mandelbrot::split, &Mandelbrot::execute, &Mandelbrot::merge).unwrap();
}