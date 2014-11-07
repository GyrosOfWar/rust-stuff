pub struct BitSet32 {
    val: u32
}

impl BitSet32 {
	pub fn new(v: u32) -> BitSet32 {
		BitSet32 {val: v}
	}

	pub fn get(&self, i: uint) -> bool {
		(self.val & (1 << i)) != 0
	}
 
	pub fn get_many(&self, start: uint, end: uint) -> u32 {
        let mut mask = 0;
        for i in range(start, end) {
        	mask |= 1 << i;
        }
        (self.val & mask) >> start
	}

	pub fn and(&self, other: BitSet32) -> BitSet32 {
		BitSet32 {val: self.val & other.val}
	}

	pub fn or(&self, other: BitSet32) -> BitSet32 {
		BitSet32 {val: self.val | other.val}
	}

	pub fn xor(&self, other: BitSet32) -> BitSet32 {
		BitSet32 {val: self.val ^ other.val}
	}
}
