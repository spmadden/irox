IROX-STATS
===========

*Various mathematical and statistics utilities*

### No-STD support:

* By default, makes use of `alloc` in many places.

### Features:

* std: adds interactions with the standard library & operating system.
* miniz: adds deflate compression/decompression support
* emath: adds conversion to/from emath primitives

### Modules:

| Module                            | `[no_std]`?          |                                                                                                                                                                                                               |
|-----------------------------------|----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`abg`](./src/abg.rs)             | ![no_std]            | Alpha-Beta (&#0945;-&#0946;/g-h) and Alpha-Beta-Gamma (&#0945;-&#0946;-&#0947;) and Kalman filters                                                                                                            |
| [`decay`](./src/decay.rs)         | ![no_std]            | Exponential Half-lifes and Decays                                                                                                                                                                             |
| [`filter`](./src/filter.rs)       | ![no_std]            | Discrete Convolution Filters for Digital Signals Processing                                                                                                                                                   |
| [`fitting`](./src/fitting.rs)     | ![no_std]            | Curve Fitting Functions (regressions)                                                                                                                                                                         |
| [`gaussian`](./src/gaussian.rs)   | ![no_std]            | Gaussian Distribution Functions                                                                                                                                                                               |
| [`points`](./src/points.rs)       | ![no_std]            | Geometric Points (Point2D/Double2D/Vec2D/Quad2D)                                                                                                                                                              |
| [`rects`](./src/rects.rs)         | ![no_std]            | Geometric Shapes (Rect2D)                                                                                                                                                                                     |
| [`sampling`](./src/sampling.rs)   | ![no_std]            | Time Series Sample Data (time, value)                                                                                                                                                                         |
| [`streaming`](./src/streaming.rs) | ![no_std]<br/>![std] | Fast Streaming Time Window Aggregation Functions (Mean, Min, Max, Unweighted Sum-of-Squares, (Un)Biased Variance, (Un)Biased Standard Deviation, 4-Number Summary) <br/> `std` adds One-Second-Window Binning |
| [`streams`](./src/streams.rs)     | ![no_std]            | Sample Codec Chained-Streams (Delta, VByte, Compression, etc)                                                                                                                                                 |
| [`tsdf`](./src/tsdf.rs)           | ![std]<br/>![miniz]  | Time-Series Data File format (TSDF) based loosely on SPDP                                                                                                                                                     |
| [`windows`](./src/windows.rs)     | ![std]               | Time-Series Data Window Filters (Savitzky-Golay, Variable-Length Time-Window Binning/Downsampling, Rolling Time Window/Horizons, Linear Regression for Rate-of-Change within Windows)                         |

[no_std]: https://img.shields.io/badge/no__std-yes-green

[std]: https://img.shields.io/badge/feature-std-lightgrey

[miniz]: https://img.shields.io/badge/feature-miniz-lightgrey
