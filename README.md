# mandelbrot
simple Mandelbrot set fractal visualiser

## building


### native binary
use plain `cargo build`

### wasm target
you need [Emscripten sdk](https://emscripten.org/docs/getting_started/downloads.html#installation-instructions)
and a emscripten rust target 
```
rustup target add wasm32-unknown-emscripten
```

then to build run
```
cargo rustc --target=wasm32-unknown-emscripten --release -- -C link-args="-s ALLOW_MEMORY_GROWTH=1"
```

### benchmarks
I was using [sp-wasm](https://github.com/golemfactory/sp-wasm) to run WASM binary.
Benchmarks are run on iMac, Intel Core i7, 4 GHz

```
$ bench '~/git/sp-wasm/target/release/wasm-sandbox -I x -O x -o out.png \
>  -w target/wasm32-unknown-emscripten/release/mandelbrot.wasm \
>  -j target/wasm32-unknown-emscripten/release/mandelbrot.js \
>  0.2 0.35 0.6 0.45 1000 1000' '~/git/mandelbrot/target/release/mandelbrot 0.2 0.35 0.6 0.45 1000 1000' 
benchmarking bench/~/git/sp-wasm/target/release/wasm-sandbox -I x -O x -o out.png \
  -w target/wasm32-unknown-emscripten/release/mandelbrot.wasm \
  -j target/wasm32-unknown-emscripten/release/mandelbrot.js \
  0.2 0.35 0.6 0.45 1000 1000
time                 1.261 s    (1.218 s .. 1.285 s)
                     1.000 R²   (1.000 R² .. 1.000 R²)
mean                 1.276 s    (1.268 s .. 1.281 s)
std dev              8.042 ms   (1.371 ms .. 10.49 ms)
variance introduced by outliers: 19% (moderately inflated)

benchmarking bench/~/git/mandelbrot/target/release/mandelbrot 0.2 0.35 0.6 0.45 1000 1000
time                 337.2 ms   (NaN s .. 338.6 ms)
                     1.000 R²   (1.000 R² .. 1.000 R²)
mean                 336.7 ms   (336.2 ms .. 336.9 ms)
std dev              361.0 μs   (41.75 μs .. 461.3 μs)
variance introduced by outliers: 19% (moderately inflated)
```

As we can see WASM execution takes ~4x longer.
