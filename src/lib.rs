pub mod base;

use crate::base::{
    Channels, Coefficients, ConfigurationCommands, ProtocolError, XLineFrame, XLineIO,
    XLineResponseFrame, ZeroCommands, crc16_hi_lo,
};
use core::time::Duration;
#[cfg(feature = "std")]
type Bytes = std::vec::Vec<u8>;
#[cfg(all(not(feature = "std"), feature = "embedded"))]
/// 250 is the maximum number of bytes so that will be allocated for each frame
const BYTES_CAP: usize = 250;
type Bytes = heapless::Vec<u8, BYTES_CAP>;

pub const TRANSPARENT_ADDRESS: u8 = 250;

pub struct KellerXLine<T: XLineIO> {
    transport: T,
    timeout: Duration,
    address: u8,
}

pub type XLineResult<T, E> = Result<T, ProtocolError<E>>;

impl<T: XLineIO> KellerXLine<T> {
    pub fn new(transport: T, timeout: Duration, address: u8) -> XLineResult<Self, T::Error> {
        Ok(Self {
            transport,
            timeout,
            address,
        })
    }

    async fn send_frame(&mut self, frame: &XLineFrame) -> XLineResult<(), T::Error> {
        self.transport.clear_rx().await?;
        #[cfg(feature = "std")]
        let mut out: Bytes = {
            let mut v = Bytes::with_capacity(2 + frame.data.len() + 2);
            v.push(frame.address);
            v.push(frame.function_code as u8);
            v.extend_from_slice(frame.data);
            v
        };
        #[cfg(all(not(feature = "std"), feature = "embedded"))]
        let mut out: Bytes = {
            let mut v = Bytes::new();
            let _ = v.push(self.address);
            let _ = v.push(frame.function_code as u8);
            let _ = v.extend_from_slice(frame.data.as_slice());
            v
        };
        let (hi, lo) = crc16_hi_lo(&out);
        let _ = out.push(hi);
        let _ = out.push(lo);
        self.transport.write_all(&out, self.timeout).await?;
        Ok(())
    }

    async fn read_response(
        &mut self,
        expected_len: usize,
    ) -> XLineResult<XLineResponseFrame, T::Error> {
        #[cfg(feature = "std")]
        let mut raw = vec![0u8; expected_len];
        #[cfg(all(not(feature = "std"), feature = "embedded"))]
        let mut raw = [0u8; BYTES_CAP];

        #[cfg(feature = "std")]
        self.transport.read_exact(&mut raw, self.timeout).await?;
        #[cfg(all(not(feature = "std"), feature = "embedded"))]
        self.transport
            .read_exact(&mut raw[..expected_len], self.timeout)
            .await?;

        #[cfg(feature = "std")]
        let parsed =
            XLineResponseFrame::from_buffer(&raw).map_err(parse_err_to_proto::<T::Error>)?;
        #[cfg(all(not(feature = "std"), feature = "embedded"))]
        let parsed = XLineResponseFrame::from_buffer(&raw[..expected_len])
            .map_err(ProtocolError::FrameError)?;
        Ok(parsed)
    }

    async fn transaction(
        &mut self,
        req: XLineFrame,
        expected_reply_len: usize,
    ) -> XLineResult<XLineResponseFrame, T::Error> {
        self.send_frame(&req).await?;
        let resp = self.read_response(expected_reply_len).await?;
        if resp.function_code != req.function_code as u8 {
            return Err(ProtocolError::NonMatchingFunctionCode);
        }
        if req.address != TRANSPARENT_ADDRESS && resp.address != req.address {
            return Err(ProtocolError::WrongAddress);
        }

        Ok(resp)
    }

    pub async fn read_coefficent(
        &mut self,
        coefficient: Coefficients,
    ) -> XLineResult<f32, T::Error> {
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::ReadCoefficients,
            data: [coefficient as u8].into(),
        };
        let response = self
            .transaction(req, base::FunctionCodes::ReadCoefficients.response_len())
            .await?;
        Ok(response.data_as_f32())
    }

    pub async fn write_coefficent(
        &mut self,
        coefficient: Coefficients,
        value: f32,
    ) -> XLineResult<(), T::Error> {
        let be = value.to_be_bytes();
        let payload = [coefficient as u8, be[0], be[1], be[2], be[3]];
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::WriteCoefficients,
            data: payload.into(),
        };
        self.transaction(req, base::FunctionCodes::WriteCoefficients.response_len())
            .await?;
        Ok(())
    }

    pub async fn read_configuration(
        &mut self,
        variable: ConfigurationCommands,
    ) -> XLineResult<u8, T::Error> {
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::ReadConfigurations,
            data: [variable as u8].into(),
        };
        let response = self
            .transaction(req, base::FunctionCodes::ReadConfigurations.response_len())
            .await?;
        Ok(response.data_as_u8())
    }

    pub async fn write_configuration(
        &mut self,
        variable: ConfigurationCommands,
        value: u8,
    ) -> XLineResult<(), T::Error> {
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::WriteConfiguration,
            data: [variable as u8, value].into(),
        };
        self.transaction(req, base::FunctionCodes::WriteConfiguration.response_len())
            .await?;
        Ok(())
    }

    pub async fn init_and_release(&mut self) -> XLineResult<(), T::Error> {
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::InitializeAndRealese,
            data: [].into(),
        };
        self.transaction(
            req,
            base::FunctionCodes::InitializeAndRealese.response_len(),
        )
        .await?;
        Ok(())
    }

    pub async fn write_address(&mut self, address: u8) -> XLineResult<u8, T::Error> {
        let req = XLineFrame {
            address,
            function_code: base::FunctionCodes::WriteAndReadNewDeviceAddress,
            data: [].into(),
        };
        let response = self
            .transaction(
                req,
                base::FunctionCodes::WriteAndReadNewDeviceAddress.response_len(),
            )
            .await?;
        Ok(response.data_as_u8())
    }

    pub async fn read_serial_number(&mut self) -> XLineResult<u32, T::Error> {
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::ReadSerialNumber,
            data: [].into(),
        };
        let response = self
            .transaction(req, base::FunctionCodes::ReadSerialNumber.response_len())
            .await?;
        Ok(response.payload[0] as u32 * 256
            ^ 3 + response.payload[1] as u32 * 256
            ^ 2 + response.payload[2] as u32 * 256 + response.payload[3] as u32)
    }

    pub async fn read_channel_value(&mut self, channel: Channels) -> XLineResult<f32, T::Error> {
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::ReadChannelValueFloat,
            data: [channel as u8].into(),
        };
        let response = self
            .transaction(
                req,
                base::FunctionCodes::ReadChannelValueFloat.response_len(),
            )
            .await?;
        Ok(response.data_as_f32())
    }

    pub async fn zero(&mut self, channel: ZeroCommands) -> XLineResult<(), T::Error> {
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::ZeroCommand,
            data: [channel as u8].into(),
        };
        self.transaction(req, base::FunctionCodes::ZeroCommand.response_len())
            .await?;
        Ok(())
    }

    pub async fn zero_with_value(
        &mut self,
        channel: ZeroCommands,
        value: f32,
    ) -> XLineResult<(), T::Error> {
        let be = value.to_be_bytes();
        let payload = [channel as u8, be[0], be[1], be[2], be[3]];
        let req = XLineFrame {
            address: self.address,
            function_code: base::FunctionCodes::ZeroCommand,
            data: payload.into(),
        };
        self.transaction(req, base::FunctionCodes::ZeroCommand.response_len())
            .await?;
        Ok(())
    }
}
