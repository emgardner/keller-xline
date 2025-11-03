use core::time::Duration;

#[cfg(feature = "std")]
type Bytes = std::vec::Vec<u8>;

#[cfg(all(not(feature = "std"), feature = "embedded"))]
type Bytes = heapless::Vec<u8, 250>;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Coefficients {
    /// 53 — Threshold value of the square root function (>0 if sqrt is used).
    ThresholdSquareRoot = 53,
    /// 64 — Offset of pressure sensor P1, default 0.0 bar.
    PressureOffsetP1 = 64,
    /// 65 — Gain factor of pressure sensor P1, default 1.0.
    GainFactorP1 = 65,
    /// 66 — Offset of pressure sensor P2, default 0.0 bar.
    PressureOffsetP2 = 66,
    /// 67 — Gain factor of pressure sensor P2, default 1.0.
    GainFactorP2 = 67,
    /// 68 — Offset of analogue output (v5.20-XX.XX / v5.24-XX.XX), bar.
    OffsetAnalogOutput = 68,
    /// 69 — Gain factor of analogue output (v5.20-XX.XX / v5.24-XX.XX).
    GainFactorAnalogOutput = 69,
    /// 70 — Offset of CH0 (v5.20-XX.XX / v5.24-XX.XX), default 0.0.
    OffsetCh0 = 70,
    /// 71 — Gain factor of CH0 (v5.20-XX.XX / v5.24-XX.XX), default 1.0.
    GainFactorCh0 = 71,
    /// 72 — Offset of temperature sensor T (v5.21-XX.XX / v5.24-XX.XX), °C.
    /// NOTE: On v5.20-XX.XX this ID is “Upper threshold for switching output 1”.
    TemperatureOffsetTorUpperThresSw1 = 72,
    /// 73 — Lower threshold for switching output 1 (v5.20-XX.XX only).
    LowerThresSw1 = 73,
    /// 74 — Offset of temperature sensor TOB1 (v5.21-XX.XX / v5.24-XX.XX), °C.
    TemperatureOffsetTOB1 = 74,
    /// 76 — Offset of temperature sensor TOB2 (v5.21-XX.XX / v5.24-XX.XX), °C.
    TemperatureOffsetTOB2 = 76,
    /// 78 — Upper threshold for switching output 2 (v5.20-XX.XX only).
    UpperThresSw2 = 78,
    /// 79 — Lower threshold for switching output 2 (v5.20-XX.XX only).
    LowerThresSw2 = 79,
    /// 100 — Free coefficient for customer use.
    Free100 = 100,
    Free101 = 101,
    Free102 = 102,
    Free103 = 103,
    Free104 = 104,
    Free105 = 105,
    Free106 = 106,
    Free107 = 107,
    Free108 = 108,
    Free109 = 109,
    Free110 = 110,
    Free111 = 111,
    /// 121 — Gain Conductivity Range 1 (v5.21-XX.XX only).
    GainCondRange1 = 121,
    /// 122 — Gain Conductivity Range 2 (v5.21-XX.XX only).
    GainCondRange2 = 122,
    /// 123 — Gain Conductivity Range 3 (v5.21-XX.XX only).
    GainCondRange3 = 123,
    /// 124 — Gain Conductivity Range 4 (v5.21-XX.XX only).
    GainCondRange4 = 124,
    /// 126 — Conductivity Temperature Coefficient (default 0.022 → water)
    /// (v5.21-XX.XX only).
    ConductivityTempCoeff = 126,
    /// 127 — Conductivity Cell Constant (default 1.00) (v5.21-XX.XX only).
    ConductivityCellConstant = 127,
    /// 140…156 — Coefficients for CH0 straight-line curve fitting of P1
    /// (v5.24-XX.XX only).
    Ch0CurveP1_140 = 140,
    Ch0CurveP1_141 = 141,
    Ch0CurveP1_142 = 142,
    Ch0CurveP1_143 = 143,
    Ch0CurveP1_144 = 144,
    Ch0CurveP1_145 = 145,
    Ch0CurveP1_146 = 146,
    Ch0CurveP1_147 = 147,
    Ch0CurveP1_148 = 148,
    Ch0CurveP1_149 = 149,
    Ch0CurveP1_150 = 150,
    Ch0CurveP1_151 = 151,
    Ch0CurveP1_152 = 152,
    Ch0CurveP1_153 = 153,
    Ch0CurveP1_154 = 154,
    Ch0CurveP1_155 = 155,
    Ch0CurveP1_156 = 156,
}

impl From<Coefficients> for u8 {
    #[inline]
    fn from(c: Coefficients) -> Self {
        c as u8
    }
}

impl core::convert::TryFrom<u8> for Coefficients {
    type Error = ();
    #[inline]
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        use Coefficients::*;
        let out = match v {
            53 => ThresholdSquareRoot,
            64 => PressureOffsetP1,
            65 => GainFactorP1,
            66 => PressureOffsetP2,
            67 => GainFactorP2,
            68 => OffsetAnalogOutput,
            69 => GainFactorAnalogOutput,
            70 => OffsetCh0,
            71 => GainFactorCh0,
            72 => TemperatureOffsetTorUpperThresSw1,
            73 => LowerThresSw1,
            74 => TemperatureOffsetTOB1,
            76 => TemperatureOffsetTOB2,
            78 => UpperThresSw2,
            79 => LowerThresSw2,
            100 => Free100,
            101 => Free101,
            102 => Free102,
            103 => Free103,
            104 => Free104,
            105 => Free105,
            106 => Free106,
            107 => Free107,
            108 => Free108,
            109 => Free109,
            110 => Free110,
            111 => Free111,
            121 => GainCondRange1,
            122 => GainCondRange2,
            123 => GainCondRange3,
            124 => GainCondRange4,
            126 => ConductivityTempCoeff,
            127 => ConductivityCellConstant,
            140 => Ch0CurveP1_140,
            141 => Ch0CurveP1_141,
            142 => Ch0CurveP1_142,
            143 => Ch0CurveP1_143,
            144 => Ch0CurveP1_144,
            145 => Ch0CurveP1_145,
            146 => Ch0CurveP1_146,
            147 => Ch0CurveP1_147,
            148 => Ch0CurveP1_148,
            149 => Ch0CurveP1_149,
            150 => Ch0CurveP1_150,
            151 => Ch0CurveP1_151,
            152 => Ch0CurveP1_152,
            153 => Ch0CurveP1_153,
            154 => Ch0CurveP1_154,
            155 => Ch0CurveP1_155,
            156 => Ch0CurveP1_156,
            _ => return Err(()),
        };
        Ok(out)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum FunctionCodes {
    ReadCoefficients = 30,
    WriteCoefficients = 31,
    ReadConfigurations = 32,
    WriteConfiguration = 33,
    InitializeAndRealese = 48,
    WriteAndReadNewDeviceAddress = 66,
    ReadSerialNumber = 67,
    ReadChannelValueFloat = 73,
    ReadChannelValueInteger = 74,
    ZeroCommand = 95,
    // Comamnd 100 Omitted
}

impl FunctionCodes {
    pub fn response_len(&self) -> usize {
        match self {
            Self::ReadCoefficients => 8,
            Self::WriteCoefficients => 5,
            Self::ReadConfigurations => 8,
            Self::WriteConfiguration => 5,
            Self::InitializeAndRealese => 10,
            Self::WriteAndReadNewDeviceAddress => 5,
            Self::ReadSerialNumber => 8,
            Self::ReadChannelValueFloat => 9,
            Self::ReadChannelValueInteger => 9,
            Self::ZeroCommand => 5,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ZeroCommands {
    SetZeroP1 = 0,
    ResetZeroP1 = 1,
    SetZeroP2 = 2,
    ResetZeroP2 = 3,
    SetZeroCH0 = 6,
    ResetZeroCH0 = 7,
    SetZeroT = 8,
    ResetZeroT = 9,
    SetZeroTOB1 = 10,
    ResetZeroTOB1 = 11,
    SetZeroTOB2 = 12,
    ResetZeroTOB2 = 13,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Channels {
    CH0 = 0,
    P1 = 1,
    P2 = 2,
    T = 3,
    TOB1 = 4,
    TOB2 = 5,
    ConTc = 10,
    ConRaw = 11,
}

pub enum ConfigurationCommands {
    CfgPressure = 0,
    CfgTemperature = 1,
    Ch0Config = 2,
    TempIntervalSeconds = 3,
    TempComp = 4,
    Filter = 7,
    DAC = 9,
    Uart = 10,
    FilterFactory = 11,
    Status = 12,
    DeviceAddress = 13,
    Pmode = 14,
    SPS = 15,
    SDI12 = 20,
    ModbusInterframeTime9k6 = 25,
    ModbusInterframeTime115k2 = 26,
    ConOn = 28,
    ConRange = 31,
    ConTempCompMode = 32,
    SDI12Available = 33,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum KellerErrors {
    NonImplementedFunction = 1,
    InvalidAddress = 2,
    IncorrectMessageLength = 3,
    ErrorSavingValue = 4,
    DeviceNotInitialized = 32,
    Other(u8),
}

impl From<u8> for KellerErrors {
    fn from(data: u8) -> KellerErrors {
        match data {
            1 => KellerErrors::NonImplementedFunction,
            2 => KellerErrors::InvalidAddress,
            3 => KellerErrors::IncorrectMessageLength,
            4 => KellerErrors::ErrorSavingValue,
            32 => KellerErrors::DeviceNotInitialized,
            _ => KellerErrors::Other(data),
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait XLineIO {
    type Error;
    async fn write_all(&mut self, buf: &[u8], timeout: Duration) -> Result<(), Self::Error>;
    async fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<(), Self::Error>;
    async fn clear_rx(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum ProtocolError<TE> {
    Transport(TE),
    Timeout,
    EchoMismatch,
    FrameError(XLineFrameError),
    WrongAddress,
    NonMatchingFunctionCode,
}

impl<E> From<E> for ProtocolError<E> {
    fn from(error: E) -> ProtocolError<E> {
        ProtocolError::Transport(error)
    }
}

pub fn crc16_hi_lo(data: &[u8]) -> (u8, u8) {
    let polynom: u16 = 0xA001;
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        crc ^= b as u16;
        for _ in 0..8 {
            let lsb = (crc & 1) != 0;
            crc >>= 1;
            if lsb {
                crc ^= polynom;
            }
        }
    }
    (((crc >> 8) & 0xFF) as u8, (crc & 0xFF) as u8)
}

pub struct XLineFrame {
    pub address: u8,
    pub function_code: FunctionCodes,
    pub data: Bytes,
}

/// Errors that can occur while parsing a response frame from raw bytes.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum XLineFrameError {
    /// Buffer length must be at least 4 (addr, func, ... , CRC_hi, CRC_lo)
    TooShort,
    /// Function code indicates device error (> 127)
    DeviceError(KellerErrors),
    /// CRC mismatch: (expected, got)
    BadCrc { expected: u16, got: u16 },
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XLineResponseFrame {
    pub address: u8,
    pub function_code: u8,
    pub payload: Bytes,
    pub crc: u16,
}

impl XLineResponseFrame {
    pub fn from_buffer(buf: &[u8]) -> Result<Self, XLineFrameError> {
        if buf.len() < 5 {
            return Err(XLineFrameError::TooShort);
        }

        let addr = buf[0];
        let func = buf[1];

        if func > 127 {
            return Err(XLineFrameError::DeviceError(KellerErrors::from(buf[2])));
        }

        let data_len = buf.len() - 2;
        let got_crc = u16::from_be_bytes([buf[data_len], buf[data_len + 1]]);
        let expected_crc = crc16(&buf[..data_len]);

        if got_crc != expected_crc {
            return Err(XLineFrameError::BadCrc {
                expected: expected_crc,
                got: got_crc,
            });
        }
        #[cfg(feature = "std")]
        let payload: Bytes = buf[2..data_len].to_vec();

        #[cfg(all(not(feature = "std"), feature = "embedded"))]
        let payload: Bytes = {
            let mut v: Bytes = Bytes::new();
            let _ = v.extend_from_slice(&buf[2..data_len]);
            v
        };

        Ok(Self {
            address: addr,
            function_code: func,
            payload,
            crc: got_crc,
        })
    }

    pub fn wire_len(&self) -> usize {
        2 + self.payload.len() + 2
    }

    pub fn data_as_u8(&self) -> u8 {
        return self.payload[0];
    }

    pub fn data_as_f32(&self) -> f32 {
        f32_from_be_bytes(&self.payload).unwrap()
    }
}

pub fn f32_from_be_bytes(buf: &[u8]) -> Option<f32> {
    if buf.len() != 4 {
        return None;
    }
    let arr = [buf[0], buf[1], buf[2], buf[3]];
    Some(f32::from_bits(u32::from_be_bytes(arr)))
}

pub fn f32_to_be_bytes(value: f32, out: &mut [u8]) -> Option<()> {
    if out.len() != 4 {
        return None;
    }
    let be = value.to_bits().to_be_bytes();
    out.copy_from_slice(&be);
    Some(())
}

pub fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        crc ^= b as u16;
        for _ in 0..8 {
            let lsb = (crc & 1) != 0;
            crc >>= 1;
            if lsb {
                crc ^= 0xA001;
            }
        }
    }
    crc
}
