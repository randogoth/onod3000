use pyo3::prelude::*;
use super::Onod as O;

#[pyclass]
struct Onod;

#[pymethods]
impl Onod {
    #[staticmethod]
    pub fn run(test: &str, samples: Vec<u8>) -> (f64, f64, f64) {
        O::run(test, &samples)
    }
}

#[pymodule]
fn onod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Onod>()?;
    Ok(())
}