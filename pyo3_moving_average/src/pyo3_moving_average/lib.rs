use pyo3::prelude::*;

fn moving_avg(data: Vec<f64>,window_size: usize) -> PyResult<Vec<f64>> {
    if window_size == 0 || window_size > data.len() { //Checks if user passed 0 as window size and if requested window size is larger than total number of items in input data array

    }
}

fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(moving_avg, m)?)?;
    Ok(())
}