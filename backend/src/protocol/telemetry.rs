use anyhow::{Result, bail};

#[repr(C, packed)]
pub struct Telemetry {
    pub distance_mm: u32,
}

impl Telemetry {
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        // 1 for required KIND byte
        let expected = 1 + core::mem::size_of::<Telemetry>();
        if b.len() != expected {
            bail!("expected {} bytes, got {}", expected, b.len());
        }

        let kind = b[0];
        if kind != super::TYPE_TELEMETRY {
            bail!(
                "unexpected kind: got {}, expected {}",
                kind,
                super::TYPE_TELEMETRY
            );
        }

        let payload = &b[1..];
        let mut t = Telemetry { distance_mm: 0 };

        unsafe {
            std::ptr::copy_nonoverlapping(payload.as_ptr(), &mut t as *mut _ as *mut u8, expected);
        }

        Ok(t)
    }
}
