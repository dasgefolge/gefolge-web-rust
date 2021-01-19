use pyo3::prelude::*;

pub mod login;

pub(crate) fn import<'p>(py: Python<'p>, module: &str) -> PyResult<&'p PyModule> {
    let sys = py.import("sys")?;
    sys.get("path")?.call_method1("append", ("/opt/py",))?;
    py.import(module)
}
