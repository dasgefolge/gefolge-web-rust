#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use pyo3::prelude::*;

pub mod login;

pub(crate) fn import<'p>(py: Python<'p>, module: &str) -> PyResult<&'p PyModule> {
    let sys = py.import("sys")?;
    sys.getattr("path")?.call_method1("append", ("/opt/py",))?;
    py.import(module)
}
