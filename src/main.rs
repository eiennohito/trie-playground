
struct BitPatternSearcher1<'a> {
    pattern: &'a [u16; 1],
}

impl <'a> BitPatternSearcher1<'a> {
    fn search(&self, target: &[u16]) -> Option<usize> {
        for offset in 0..target.len() {
            let ptr = unsafe { target.as_ptr().offset(offset as isize)}  as *const u32;
            let value = unsafe { ptr.read_unaligned() };
            let mut mask = self.pattern[0] as u32;
            for shift in 0..16 {
                if value & mask == 0 {
                    return Some(offset * 16 + shift)
                }
                mask <<= 1;
            }
        }
        None
    }
}

#[derive(Default)]
struct TrieNode {
    children: fxhash::FxHashMap<u8, TrieNode>,
    weight: u64,
    value: Option<u32>
}

impl TrieNode {
    pub fn child_indices(&self, out: &mut Vec<u8>) {
        out.extend(self.children.keys());
        out.sort()
    }

    pub fn add(&mut self, data: &[u8], value: u32, weight: u64) -> usize {
        self.weight = self.weight.saturating_add(weight);
        let mut result = 0;
        if data.len() > 0 {
            let (k, rem) = data.split_first().unwrap();
            let entry = self.children.entry(*k);
            result += entry.or_insert_with(|| {
                result += 1;
                TrieNode::default()
            }).add(rem, value, weight)
        } else {
            self.value = Some(value)
        }
        result
    }
}

#[derive(Default)]
struct HashTrie {
    root: TrieNode,
    num_entries: usize
}

impl HashTrie {
    pub fn add(&mut self, data: &[u8], value: u32, weight: u64) {
        self.num_entries += self.root.add(data, value, weight);
    }
}

fn main() {
    println!("Hello, world!");
}
