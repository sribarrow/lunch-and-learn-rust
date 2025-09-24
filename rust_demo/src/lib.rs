// src/lib.rs
use numpy::PyReadonlyArray1;
use pyo3::prelude::*; // Python, PyResult, #[pymodule], Bound<PyModule>, etc.
use pyo3::wrap_pyfunction;
use rayon::prelude::*;

/// Sum of squares, zero-copy from a NumPy array
#[pyfunction]
fn sum_squares_numpy(py: Python<'_>, x: PyReadonlyArray1<f64>) -> PyResult<f64> {
    let slice = x.as_slice()?; // safe zero-copy view
                               // Release the GIL while we crunch numbers
    let sum = py.allow_threads(|| slice.iter().map(|v| v * v).sum());
    Ok(sum)
}

/// Parallel sum of squares for large arrays (uses rayon)
#[pyfunction]
fn sum_squares_numpy_parallel(py: Python<'_>, x: PyReadonlyArray1<f64>) -> PyResult<f64> {
    let slice = x.as_slice()?;
    let sum: f64 = py.allow_threads(|| slice.par_iter().map(|v| v * v).sum());
    Ok(sum)
}

/// Count primes up to n (inclusive) using a simple sieve.
/// Great for showing big speedups vs a Python loop.
#[pyfunction]
fn count_primes(py: Python<'_>, n: usize) -> PyResult<usize> {
    let count = py.allow_threads(|| {
        if n < 2 {
            return 0usize;
        }
        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut p = 2usize;
        while p * p <= n {
            if is_prime[p] {
                let mut m = p * p;
                while m <= n {
                    is_prime[m] = false;
                    m += p;
                }
            }
            p += 1;
        }
        is_prime.into_iter().filter(|&b| b).count()
    });
    Ok(count)
}

#[pyfunction]
fn fib_iter_mod(n: u64) -> PyResult<u64> {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        // wrapping_add makes the intent explicit
        b = tmp.wrapping_add(b);
    }
    Ok(a)
}

#[pymodule]
fn rust_demo(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_squares_numpy, m)?)?;
    m.add_function(wrap_pyfunction!(sum_squares_numpy_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(count_primes, m)?)?;
    m.add_function(wrap_pyfunction!(fib_iter_mod, m)?)?;
    Ok(())
}
