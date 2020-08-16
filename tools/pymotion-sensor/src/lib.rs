use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
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

fn reading_to_pyobj(py: Python, reading: Reading) -> PyObject {
    match reading {
        Reading::Accelerometer(acc) => Acc::from(acc).into_py(py),
        Reading::Gyroscope(gyro) => Gyro::from(gyro).into_py(py),
        Reading::Magnetometer(mag) => Mag::from(mag).into_py(py),
    }
}

/// Converts a raw buffer of one or more readings into a collection of readings
///
/// Throws an error if we don't know how to convert a reading. Assumes that all bytes
/// in the buffer represents one or more readings. If there are extra bytes, the
/// implementation will try to convert them and fail.
///
/// The readings are COBS encoded. You should find the first zero byte in the buffer,
/// create a `bytearray` up to and including that zero byte, then pass the buffer into
/// this function. If there is other data after the zero byte, you may discard it after
/// acquiring the readings.
///
/// The function decodes in place. If this function returns readings, the bytes up to the
/// zero byte may be modified.
#[pyfunction]
pub fn convert_readings(py: Python, buffer: &PyByteArray) -> PyResult<Vec<PyObject>> {
    // Safety: short-lived operation that does not execute any Python code.
    let buffer = unsafe { buffer.as_bytes_mut() };
    let readings: Vec<Reading> = match postcard::from_bytes_cobs(buffer) {
        Err(err) => {
            return Err(PyErr::new::<ValueError, _>(format!(
                "error converting readings: {:?}",
                err,
            )));
        }
        Ok(readings) => readings,
    };
    Ok(readings
        .into_iter()
        .map(|reading| reading_to_pyobj(py, reading))
        .collect())
}

/// Python interface to the Rust motion-sensor crate types
///
/// See the `convert_readings` function documentation for more information on turning
/// raw byte arrays into motion sensor readings.
#[pymodule]
pub fn motion_sensor(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Acc>()?;
    m.add_class::<Gyro>()?;
    m.add_class::<Mag>()?;

    m.add_wrapped(wrap_pyfunction!(convert_readings))?;

    Ok(())
}
