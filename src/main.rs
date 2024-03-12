use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_hello/hello.py"));
    let say_hello = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("say_hello")?
            .into();
        app.call0(py)
    });

    println!("From Rust: {}", say_hello?);
    Ok(())
}
