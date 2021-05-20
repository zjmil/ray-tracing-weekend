# ray-tracing-weekend

This is an implementation of [ray-tracing-weekend](https://raytracing.github.io) but in Rust.
This was implemented to get more familiar with Rust and also learn a little about ray tracing.


### Building & Running

The application produces a ppm image to stdout. To build and run, one can run:
`cargo run --release > image.ppm`

NOTE: There currently is not a way to configure which image is generated
without changing the code; this is a TODO.


### Examples

![Balls](/images/balls.png)

