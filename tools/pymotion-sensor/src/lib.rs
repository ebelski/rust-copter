use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use motion_sensor::{Reading, Triplet};

/// Accelerometer readings, units in Gs
#[pyclass]
pub struct Acc {
    /// X coordinate
    #[pyo3(get)]
    pub x: f32,
    /// Y coordinate
    #[pyo3(get)]
    pub y: f32,
    /// Z coordinate
    #[pyo3(get)]
    pub z: f32,
}

impl From<Triplet<f32>> for Acc {
    fn from(t: Triplet<f32>) -> Self {
        Acc {
            x: t.x,
            y: t.y,
            z: t.z,
        }
    }
}

/// Gyroscope readings, units in deg/sec
#[pyclass]
pub struct Gyro {
    /// X coordinate
    #[pyo3(get)]
    pub x: f32,
    /// Y coordinate
    #[pyo3(get)]
    pub y: f32,
    /// Z coordinate
    #[pyo3(get)]
    pub z: f32,
}

impl From<Triplet<f32>> for Gyro {
    fn from(t: Triplet<f32>) -> Self {
        Gyro {
            x: t.x,
            y: t.y,
            z: t.z,
        }
    }
}

/// Magnetometer readings, units in uT
#[pyclass]
pub struct Mag {
    /// X coordinate
    #[pyo3(get)]
    pub x: f32,
    /// Y coordinate
    #[pyo3(get)]
    pub y: f32,
    /// Z coordinate
    #[pyo3(get)]
    pub z: f32,
}

impl From<Triplet<f32>> for Mag {
    fn from(t: Triplet<f32>) -> Self {
        Mag {
            x: t.x,
            y: t.y,
            z: t.z,
        }
    }
}

/// Converts a raw buffer of one or more readings into a collection of readings
///
/// Throws an error if we don't know how to convert a reading. Assumes that all bytes
/// in the buffer represents one or more readings. If there are extra bytes, the
/// implementation will try to convert them and fail.
#[pyfunction]
pub fn convert_readings(py: Python, mut buffer: &[u8]) -> PyResult<Vec<PyObject>> {
    let mut objects = Vec::new();
    while !buffer.is_empty() {
        buffer = match postcard::take_from_bytes(buffer) {
            Ok((reading, buffer)) => {
                match reading {
                    Reading::Accelerometer(acc) => {
                        let acc = Acc::from(acc);
                        objects.push(acc.into_py(py));
                    }
                    Reading::Gyroscope(gyro) => {
                        let gyro = Gyro::from(gyro);
                        objects.push(gyro.into_py(py));
                    }
                    Reading::Magnetometer(mag) => {
                        let mag = Mag::from(mag);
                        objects.push(mag.into_py(py));
                    }
                }
                buffer
            }
            Err(err) => {
                return Err(PyErr::new::<ValueError, _>(format!(
                    "error converting readings: {:?} (found {} readings in buffer)",
                    err,
                    objects.len(),
                )));
            }
        }
    }
    Ok(objects)
}

/// Python interface to the Rust motion-sensor crate types
#[pymodule]
pub fn motion_sensor(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Acc>()?;
    m.add_class::<Gyro>()?;
    m.add_class::<Mag>()?;

    m.add("READING_SIZE", std::mem::size_of::<Reading>())?;
    
    m.add_wrapped(wrap_pyfunction!(convert_readings))?;

    Ok(())
}
