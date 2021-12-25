# ray

This is a simple ray tracer based on [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) and parts of [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html).

## Setup

This assumes that you're compiling in a UNIX enviroment and has g++ 9.3.0 or higher available.

To compile and run
```bash
make folders
make
./ray > img.ppm
```

At the moment, the only available image format is PPM.

In theory it should work with clang and on Mac OS, but it wasn't tested.

## Creating a scene

Modify the functions on `src/scene.cpp` and compile it.

By default, the program will use all your logical cores except one, to change that or the parallelism strategy, modify the functions on `src/utils/parallel.cpp`

## TODO:

1. Add textures
2. Add blocks
3. Add different Light Sources
4. Enable different parallel strategies (e.g.: hierarchical accuracy instead of time)