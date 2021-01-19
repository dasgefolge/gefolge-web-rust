use {
    pyo3::prelude::*,
    serenity::model::prelude::*,
};

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub struct Mensch {
    pub snowflake: UserId,
}

impl Mensch {
    fn inner<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        Ok(crate::import(py, "gefolge_web.login")?.get("Mensch")?.call1((self.snowflake.0,))?)
    }

    pub fn by_api_key(api_key: &str) -> PyResult<Option<Mensch>> {
        Ok(
            Python::with_gil(|py| {
                PyResult::Ok(if let Some(mensch) = crate::import(py, "gefolge_web.login")?.get("Mensch")?.call_method1("by_api_key", (api_key,))?.extract::<Option<&PyAny>>()? {
                    Some(mensch.getattr("snowflake")?.extract()?)
                } else {
                    None
                })
            })?.map(|snowflake| Mensch { snowflake: UserId(snowflake) })
        )
    }

    pub fn nickname(&self) -> PyResult<Option<String>> {
        Python::with_gil(|py| Ok(self.inner(py)?.getattr("nickname")?.extract()?))
    }

    pub fn username(&self) -> PyResult<String> {
        Python::with_gil(|py| Ok(self.inner(py)?.getattr("username")?.extract()?))
    }

    pub fn discrim(&self) -> PyResult<u16> {
        Python::with_gil(|py| Ok(self.inner(py)?.getattr("profile_data")?.get_item("discriminator")?.extract()?))
    }
}
