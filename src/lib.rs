use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;

pub mod v1;
use crate::v1::exp::RegExp;
use crate::v1::part::RegexPart;
use crate::v1::utils::{digit, exactly, any_of};




#[pymodule]
fn rgxx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RegexPart>()?;
    m.add_class::<RegExp>()?;
    m.add_function(wrap_pyfunction!(digit, m)?)?;
    m.add_function(wrap_pyfunction!(exactly, m)?)?;
    m.add_function(wrap_pyfunction!(any_of, m)?)?;
    Ok(())
}
