# Octarr

Stands for octree based 3D array.

Octarr is a dynamic cubic octree capable of storing class data, accessed via indexers.
It is a rust port of https://github.com/swift502/Octarr and inspired by https://github.com/marknefedov/cubic-octree.

> #### Adapted from swift502/Octarr
>
> "Basically a generic near endless 3D array, which can be accessed like regular arrays and grows and shrinks depending on how much space needs to be allocated. Location index can be positive or negative in any direction. Octarr is centered around the zero coordinate (0, 0, 0), and grows and shrinks from and to this zero coordinate.
> 
> Unlike multidimensional or jagged arrays, octarr is memory friendly. You can write a data block at the [i128::MAX, i128::MAX, i128::MAX] position and not run out of memory. Octree node lookups  have logarithmic complexity. Octarr is internally using the i128 data type to allow for unconstrained data location."

## Usage

```toml
[dependencies]
octarr = "0.1"
```