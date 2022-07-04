It is a on-going crate that provides basic matrix manipunations and ports and python's [scipy-sklearn's matrix decomposition](https://scikit-learn.org/stable/modules/decomposition.html) to pure Rust 

## Progress

Currently on top of the crate [rulinalg](https://github.com/AtheMathmo/rulinalg)
it provides some linear algebra computations:
- basics
  - Matrix Inversion/Transformations and other manipunations
  - Linear Solver
- Decompositions
  - LUP 
  - QR 
  - SVD 
  - Upper
  - Eigenvalue
  - RandomizedPCA(**not yet**)
  - SparsedPCA(**not yet**)
	

## References

- you can look around the [scikit-learn's decomposition](https://scikit-learn.org/stable/modules/decomposition.html#decompositions) site and see around the python version of it,

- the basic framework of this crate is inspired by the this crate [rulinalg](https://github.com/AtheMathmo/rulinalg), it will be a plus to know about it.
