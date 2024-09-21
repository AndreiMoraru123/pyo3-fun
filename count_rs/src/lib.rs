use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PySequence, PySet};
use rand::random;

#[pyfunction]
#[pyo3(signature = (items, epsilon=0.5, delta=0.001))]
fn count_approx_rs(
    py: Python<'_>,
    items: &Bound<PySequence>,
    epsilon: f64,
    delta: f64,
) -> PyResult<u64> {
    let mut p = 1.0;
    let tracked_items = PySet::empty_bound(py)?;
    let max_tracked =
        ((12.0 / epsilon.powi(2)) * (8.0 * items.len()? as f64 / delta).log2()).round() as usize;
    for item in items.iter()? {
        let item = item?;
        tracked_items.discard(item.clone())?;
        if random::<f64>() < p {
            tracked_items.add(item)?;
        }
        if tracked_items.len() == max_tracked {
            let temp_tracked_items = PySet::empty_bound(py)?;
            for subitem in tracked_items.iter() {
                if random::<f64>() < 0.5 {
                    let _ = temp_tracked_items.add(subitem);
                }
                p /= 2.0;
                if tracked_items.len() == 0 {
                    return Err(PyRuntimeError::new_err("unlucky"));
                }
            }
        }
    }

    Ok((tracked_items.len() as f64 / p).round() as u64)
}

#[pymodule]
fn count_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_approx_rs, m)?)?;
    Ok(())
}
