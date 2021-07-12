use ordered_float::NotNan;
use pyo3::prelude::*;
use pyo3::PyIterProtocol;

use super::price_queue;
use super::ws_coin;

#[pymodule]
fn cyberpunk_display(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PriceQueueRust>()?;
    m.add_class::<WsCoinRust>()?;
    Ok(())
}

#[pyclass]
struct PriceQueueRust {
    pq: price_queue::PriceQueue,
}

#[pymethods]
impl PriceQueueRust {
    #[new]
    pub fn new() -> Self {
        PriceQueueRust {
            pq: price_queue::PriceQueue::default(),
        }
    }

    pub fn push(&mut self, p: f64) -> PyResult<()> {
        self.pq.push(NotNan::new(p).unwrap());
        Ok(())
    }

    pub fn to_plot(&self) -> PyResult<String> {
        Ok(self.pq.to_plot())
    }

    pub fn to_rgb565(&self) -> PyResult<Vec<u16>> {
        Ok(self.pq.to_screen().serialize())
    }
}

#[pyclass]
struct WsCoinRust {
    ws_coin: ws_coin::WsCoin,
}

#[pymethods]
impl WsCoinRust {
    #[new]
    pub fn new() -> Self {
        WsCoinRust {
            ws_coin: ws_coin::WsCoin::default(),
        }
    }
}

#[pyproto]
impl PyIterProtocol for WsCoinRust {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<Self>) -> Option<f64> {
        slf.ws_coin.next().map(|p| p.price.into())
    }
}
