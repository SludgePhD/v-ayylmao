use std::fmt;

/// A FourCC code identifying a pixel format.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelFormat(u32);

impl PixelFormat {
    /// Planar YUV 4:2:0 standard pixel format.
    ///
    /// All samples are 8 bits in size. The plane containing Y samples comes first, followed by a
    /// plane storing packed U and V samples (with U samples in the first byte and V samples in the
    /// second byte).
    ///
    /// This format is widely supported by hardware codecs (and often the *only* supported format),
    /// so it should be supported by all software, and may be used as the default format.
    pub const NV12: Self = f(b"NV12");

    pub const fn from_bytes(fourcc: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(fourcc))
    }

    pub const fn from_u32_le(fourcc: u32) -> Self {
        Self(fourcc)
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    pub const fn to_u32_le(self) -> u32 {
        self.0
    }
}

const fn f(fourcc: &[u8; 4]) -> PixelFormat {
    PixelFormat::from_bytes(*fourcc)
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = self.0.to_le_bytes();
        let [a, b, c, d] = [a as char, b as char, c as char, d as char];
        write!(f, "{}{}{}{}", a, b, c, d)
    }
}

impl fmt::Debug for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
