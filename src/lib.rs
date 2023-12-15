use pyo3::prelude::*;
use wasmedge_sdk::plugin::PluginManager;

#[pyfunction]
fn run() {
    println!("Hello, world!");

    let plugin_path = "libwasmedgePluginWasiNN.so";

    let _ = PluginManager::load(Some(std::path::Path::new(plugin_path)));

    let names = PluginManager::names();
    let count = PluginManager::count();

    println!("Plugin count: {}", count);
    println!("Plugin names: {:?}", names);
}

/// A Python module implemented in Rust.
#[pymodule]
fn load_plugin(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
