use ordered_float::NotNan;
use pyo3::prelude::*;

use super::price_queue;

#[pymodule]
fn cyberpunk_display(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PriceQueueRust>()?;
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
            pq: price_queue::PriceQueue::new(),
        }
    }

    pub fn push(&mut self, p: f64) -> PyResult<()> {
        self.pq.push(NotNan::new(p).unwrap());
        Ok(())
    }

    pub fn to_plot(&self) -> PyResult<String> {
        Ok(self.pq.to_plot())
    }

    pub fn to_rgb565(&self) -> PyResult<Vec<Vec<u16>>> {
        Ok(self.pq.to_rgb565())
    }
}
