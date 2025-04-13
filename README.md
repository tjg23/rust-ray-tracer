# rust-ray-tracer
A simple, barebones ray tracing / path tracing engine written in Rust.
Development largely followed the first two books in the [Ray Tracing in One Weekend](https://raytracing.github.io)
series, translating its C++ implementation into Rust code.

## Basics
- Includes basic spheres, triangles, and quadrilaterals.
- Includes complex triangle mesh models
- Objects have a material and texture, including texture mapping.
- Outputs to `.ppm` files

## Features
- Supports **anti-aliasing**, **diffuse materials**, **metals**, **dielectrics**, and **volumes**
- Uses a **Bounding Volume Hierarchy** (BVH), consisting of **axis-aligned bounding boxes**, for performance
- Allows **importing models** from `.obj` files and using **instances** for efficiency
