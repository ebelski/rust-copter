//! Sensor datapath implementations
//!
//! The datapath describes how data reaches the host. It's a one-way
//! channel.

#![allow(dead_code)] // Demo may only use one of these datapaths

pub mod uart;
pub mod usb;
