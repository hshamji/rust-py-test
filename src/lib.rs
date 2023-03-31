// #[macro_use]
// extern crate cpython;
//
// use cpython::{Python, PyResult};
//
// fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
//     let mut total = 0u64;
//
//     // There is an improved version later on this post
//     for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
//         if c1 == c2 {
//             total += 1;
//         }
//     }
//
//     Ok(total)
// }
//
// py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m | {
//     m.add(py, "__doc__", "This module is implemented in Rust")?;
//     m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str)))?;
//     Ok(())
// });


// use cpython::py_module_initializer;
//
// py_module_initializer! {
//     docstring, |py, module| {
//         module.add(py, "__doc__", "This module is implemented in Rust.")?;
//
//         Ok(())
//     }
// }


#![allow(unused)]
fn main() {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    /// A Python module implemented in Rust. The name of this function must match
    /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
    /// import the module.
    #[pymodule]
    #[pyo3(name = "myrustlib")]
    fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
        Ok(())
    }
}
