use super::{write::Writer, Encoder};
use crate::{config::Config, utils::Sealed};

/// An Encoder that writes bytes into a given writer `W`.
///
/// This struct should rarely be used.
/// In most cases, prefer any of the `encode` functions.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to write integers to the writer.
///
/// ```
/// # use bincode::enc::{write::SliceWriter, EncoderImpl, Encode};
/// let slice: &mut [u8] = &mut [0, 0, 0, 0];
/// let config = bincode::config::legacy().with_big_endian();
///
/// let mut encoder = EncoderImpl::new(SliceWriter::new(slice), config);
/// // this u32 can be any Encodable
/// 5u32.encode(&mut encoder).unwrap();
/// assert_eq!(encoder.into_writer().bytes_written(), 4);
/// assert_eq!(slice, [0, 0, 0, 5]);
/// ```
pub struct EncoderImpl<W: Writer, C: Config> {
    writer: W,
    config: C,
}

impl<W: Writer, C: Config> EncoderImpl<W, C> {
    /// Create a new Encoder
    pub fn new(writer: W, config: C) -> EncoderImpl<W, C> {
        EncoderImpl { writer, config }
    }

    /// Return the underlying writer
    #[inline]
    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<W: Writer, C: Config> Encoder for EncoderImpl<W, C> {
    type W = W;

    type C = C;

    #[inline]
    fn writer(&mut self) -> &mut Self::W {
        &mut self.writer
    }

    #[inline]
    fn config(&self) -> &Self::C {
        &self.config
    }
}

impl<W: Writer, C: Config> Sealed for EncoderImpl<W, C> {}
#[cfg(test)]
mod tests_rug_367 {
    use super::*;
    use crate::enc::encoder::EncoderImpl;
    use crate::enc::write::SizeWriter;
    use crate::config::Configuration;
    use crate::config::{BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        // Prepare sample data for arguments
        let mut p0 = {
            let mut v18 = SizeWriter {
                bytes_written: 0,
            };
            v18
        };

        let mut p1 = {
            let mut v9: Configuration<BigEndian, Fixint, Limit<100>> = Configuration::<BigEndian, Fixint, Limit<100>>::default();
            v9
        };

        EncoderImpl::<SizeWriter, Configuration<BigEndian, Fixint, Limit<100>>>::new(p0, p1);
    }
}