use pyo3::{pyclass, pymethods};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use rgx::{Part, RegExp};

#[pyclass(name = "RegExp")]
#[derive(Clone)]
pub struct PyRegExp {
    rx: RegExp,
}
impl PyRegExp {
    pub fn new(parts: Vec<PyPart>) -> Result<Self, RgxError> {
        let patterns: Result<Vec<Part>, RgxError> = parts.into_iter()
            .map(|part| Ok(part.inner))
            .collect();
        let patterns = patterns?;
        Ok(Self {
            rx: RegExp::new(patterns)?,
        })
    }
    pub fn compile(&self) -> String {
        self.rx.pattern.clone()
    }
    pub fn __repr__(&self) -> String {
        format!("RegExp({})", self.rx.pattern)
    }
}



#[pyclass(name = "Part")]
#[derive(Clone)]
pub struct PyPart {
    pub inner: Part,
}

#[pymethods]
impl PyPart {
    #[new]
    pub fn new(pattern: Option<&str>) -> Self {
        Self {
            inner: Part::new(pattern),
        }
    }

    pub fn times(&mut self, count: usize) -> Self {
        let part = self.inner.times(count).to_owned();
        Self {
            inner: part,
        }
    }

    pub fn grouped_as(&mut self, name: &str) -> Self {
        let part = self.inner.group_as(name).to_owned();
        Self {
            inner: part,
        }
    }

    pub fn and(&mut self, other: &PyPart) -> Self {
        let part = other.inner.clone();
        let inner = self.inner.add(&part).to_owned();
        Self { inner, }
    }

    pub fn digit(&mut self) -> Self {
        Self {
            inner: self.inner.digit().to_owned(),
        }
    }

    #[staticmethod]
    pub fn any_of(parts: Vec<PyPart>) -> Result<Self, PyErr> {
        let patterns: Result<Vec<Part>, PyErr> = parts.into_iter()
            .map(|part| Ok(part.inner))
            .collect();
        let patterns = patterns?;
        let part = Part::new(None).any_of(patterns).map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?.to_owned();
        Ok(Self {
            inner: part,
        })
    }

    pub fn exactly(&self, pattern: &str) -> Self {
        let part = Part::new(None).exactly(pattern).to_owned();
        Self {
            inner: part,
        }
    }

    #[staticmethod]
    pub fn any_character() -> Self {
        let part = Part::new(None).any_character().to_owned();
        Self {
            inner: part,
        }
    }

    pub fn infinity(&self) -> Self {
        let part = Part::new(None).infinity().to_owned();
        Self {
            inner: part,
        }
    }

    pub fn __str__(&self) -> String {
        self.inner.pattern.clone()
    }

    pub fn __repr__(&self) -> String {
        format!("PyPart({})", self.inner.pattern)
    }
}

use pyo3::prelude::*;
use pyo3::types::PyModule;
use rgx::error::RgxError;

#[pymodule]
fn rgxx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPart>()?;
    m.add_class::<PyRegExp>()?;
    Ok(())
}

