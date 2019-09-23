pub mod mandelbrot;
pub mod png_utils;
pub mod utils;

use crate::mandelbrot::Mandelbrot;
use gwasm_api::dispatcher;

fn main() {
    // This function will parse command line arguments and dispatch it
    // to one of split, execute and merge functions.
    dispatcher::run(&Mandelbrot::split, &Mandelbrot::execute, &Mandelbrot::merge).unwrap();
}
