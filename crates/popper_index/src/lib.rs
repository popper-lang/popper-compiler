pub trait Idx: Clone + Copy + PartialEq + Eq {
    const MAX_ID: usize;
    const MAX: Self;
    fn new(val: usize) -> Self;
    fn index(self) -> usize;
}

impl Idx for usize {
    const MAX_ID: usize = usize::MAX;
    const MAX: Self = usize::MAX;
    fn new(val: usize) -> Self {
        val
    }

    fn index(self) -> usize {
        self
    }
}

impl Idx for u8 {
    const MAX_ID: usize = u8::MAX as usize;
    const MAX: Self = u8::MAX;

    fn new(val: usize) -> Self {
        assert!(val <= u8::MAX as usize);
        val as u8
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl Idx for u16 {
    const MAX_ID: usize = u16::MAX as usize;
    const MAX: Self = u16::MAX;

    fn new(val: usize) -> Self {
        assert!(val <= u16::MAX as usize);
        val as u16
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl Idx for u32 {
    const MAX_ID: usize = u32::MAX as usize;
    const MAX: Self = u32::MAX;
    fn new(val: usize) -> Self {
        assert!(val <= u32::MAX as usize);
        val as u32
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl Idx for u64 {
    const MAX_ID: usize = u64::MAX as usize;
    const MAX: Self = u64::MAX;
    fn new(val: usize) -> Self {
        assert!(val <= u64::MAX as usize);
        val as u64
    }

    fn index(self) -> usize {
        self as usize
    }
}



