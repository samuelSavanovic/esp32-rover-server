#[repr(C, packed)]
pub struct Command {
    pub kind: u8,
    pub left_pwm: i16,
    pub right_pwm: i16,
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = core::mem::size_of::<Command>();
        let mut out = vec![0u8; size];

        unsafe {
            std::ptr::copy_nonoverlapping(
                self as *const _ as *const u8,
                out.as_mut_ptr(),
                size,
            );
        }

        out
    }
    pub fn new(left: i16, right: i16) -> Self {
        Self { kind: super::TYPE_COMMAND, left_pwm: left, right_pwm: right }
    }
}


