pub struct BitSet32 {
    val: u32
}

impl BitSet32 {
    pub fn new(v: u32) -> BitSet32 {
	BitSet32 { val: v }
    }
    
    #[inline]
    pub fn get(&self, i: usize) -> bool {
	(self.val & (1 << i)) != 0
    }

    #[inline]
    pub fn get_many(&self, start: usize, end: usize) -> u32 {
        let mut mask = 0;
        for i in (start..end) {
            mask |= 1 << i;
        }
        (self.val & mask) >> start
    }

    #[inline]
    pub fn and(&self, other: BitSet32) -> BitSet32 {
	BitSet32 { val: self.val & other.val }
    }

    #[inline]
    pub fn or(&self, other: BitSet32) -> BitSet32 {
	BitSet32 { val: self.val | other.val }
    }

    #[inline]
    pub fn xor(&self, other: BitSet32) -> BitSet32 {
	BitSet32 { val: self.val ^ other.val }
    }
}
