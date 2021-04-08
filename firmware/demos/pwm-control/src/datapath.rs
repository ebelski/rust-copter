//! IMU datapath over UART

use teensy4_bsp::hal;

type Sink = hal::uart::Tx<hal::iomuxc::consts::U2>;

/// Datapath writer
pub struct Datapath {
    peripheral: hal::dma::Peripheral<Sink, u8, hal::dma::Circular<u8>>,
    circular: Option<hal::dma::Circular<u8>>,
}

/// Required buffer alignment type for DMA transfers
#[repr(align(1024))]
struct Align1024(hal::dma::Buffer<[u8; 1024]>);

/// Transfer buffer
static BUFFER: Align1024 = Align1024(hal::dma::Buffer::new([0; 1024]));

/// Possible datapath errors
#[derive(Debug)]
pub enum Error {
    /// You've already created a datapath, and that datapath
    /// owns the static buffer
    AlreadyCreated,
    /// We can't find the buffer; unexpected
    NoBuffer,
    /// Error starting the transfer
    Transfer(hal::dma::Error<void::Void>),
}

impl Datapath {
    pub fn new(sink: Sink, mut channel: hal::dma::Channel) -> Result<Self, Error> {
        let circular = hal::dma::Circular::new(&BUFFER.0).map_err(|_| Error::AlreadyCreated)?;

        channel.set_interrupt_on_completion(false);
        channel.set_interrupt_on_half(false);
        let peripheral = hal::dma::transfer_u8(sink, channel);

        Ok(Datapath {
            peripheral,
            circular: Some(circular),
        })
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<(), Error> {
        self.poll()?;

        if let Some(mut circular) = self.circular.take() {
            circular.insert(buffer.iter().copied());
            match self.peripheral.start_transfer(circular) {
                Ok(()) => Ok(()),
                Err((circular, err)) => {
                    self.circular = Some(circular);
                    Err(Error::Transfer(err))
                }
            }
        } else if let Some(mut circular) = self.peripheral.write_half() {
            circular.insert(buffer.iter().copied());
            Ok(())
        } else {
            Err(Error::NoBuffer)
        }
    }

    pub fn poll(&mut self) -> Result<(), Error> {
        if self.peripheral.is_transfer_complete() {
            self.circular = self.peripheral.transfer_complete();
            if self.circular.is_none() {
                Err(Error::NoBuffer)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
