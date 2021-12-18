
pub struct BitReader {
    len: usize,
    pos: usize,
    bytes: Vec<u8>,
}

impl BitReader {
	#[inline(always)]
	#[allow(unused)]
    pub fn new() -> Self {
        BitReader { len: 0, pos: 0, bytes: Vec::new() }
    }

	#[inline(always)]
	#[allow(unused)]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        BitReader { len: bytes.len() * 8, pos: 0, bytes }
    }

    pub fn from_bits(bools: Vec<bool>) -> Self {
        let mut bytes = Vec::new();
        for b in bools.chunks(8) {
            let mut byte = 0;
            for (i, bit) in b.iter().enumerate() {
                byte |= (*bit as u8) << (7 - i);
            }
            bytes.push(byte);
        }
        BitReader { len: bools.len(), pos: 0, bytes }
    }

	#[inline]
	#[allow(unused)]
    pub fn append_bytes(&mut self, bytes: Vec<u8>) {
        self.len += bytes.len() * 8;
        self.bytes.extend(bytes);
    }

	#[allow(unused)]
    pub fn append(&mut self, bit: bool) {
        self.len += 1;
        if self.len % 8 == 0 {
            self.bytes.push(bit as u8);
        }
        else {
            let byte = self.bytes.last_mut().unwrap();
            if bit { *byte |= 1 << (self.len % 8 - 1); }
        }
    }

    pub fn read_bits(&mut self, count: usize) -> Option<usize> {
        if count == 0 { return Some(0); }
        if self.pos + count > self.len { return None; }

        let mut result = 0usize;
        for _ in 0..count {
            let bit = self.bytes[self.pos / 8] & (1 << (7 - (self.pos % 8))) != 0;
            result = (result << 1) | (bit as usize);
            self.pos += 1;
        }
        Some(result)
    }

    pub fn left(&self) -> usize {
        self.len - self.pos
    }
}

impl Iterator for BitReader {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        }
        let byte = self.bytes[self.pos / 8];
        let bit = byte & (1 << (7 - (self.pos % 8)));
        self.pos += 1;
        Some(bit != 0)
    }
}