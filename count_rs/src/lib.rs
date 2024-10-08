use nohash_hasher::IntSet;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PySequence, PySet};
use rand::random;
use rand::{rngs::SmallRng, Rng, SeedableRng};

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

#[pyfunction]
#[pyo3(signature = (items, epsilon=0.5, delta=0.001))]
fn count_approx_rs_opt(
    py: Python<'_>,
    items: &Bound<PySequence>,
    epsilon: f64,
    delta: f64,
) -> PyResult<u64> {
    let mut p = 1.0;
    let mut tracked_items = IntSet::default();
    let mut rng = SmallRng::from_entropy();
    let mut random = || rng.gen::<f64>();
    let max_tracked =
        ((12.0 / epsilon.powi(2)) * (8.0 * items.len()? as f64 / delta).log2()).round() as usize;
    for item in items.iter()? {
        let hash = item?.hash()?;
        tracked_items.remove(&hash);
        if random() < p {
            tracked_items.insert(hash);
        }
        if tracked_items.len() == max_tracked {
            tracked_items.retain(|_| random() < 0.5);
            p /= 2.0;
            if tracked_items.len() == 0 {
                return Err(PyRuntimeError::new_err("unlucky"));
            }
        }
    }

    Ok((tracked_items.len() as f64 / p).round() as u64)
}

#[pymodule]
fn count_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_approx_rs, m)?)?;
    m.add_function(wrap_pyfunction!(count_approx_rs_opt, m)?)?;
    Ok(())
}
