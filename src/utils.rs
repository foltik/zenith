#[rustfmt::skip]
pub mod bytes {
    pub fn ds(f: &mut Vec<u8>, s: &[u8]) { f.extend_from_slice(s); }
    pub fn dq(f: &mut Vec<u8>, v: u64)   { f.extend_from_slice(&v.to_ne_bytes()); }
    pub fn dd(f: &mut Vec<u8>, v: u32)   { f.extend_from_slice(&v.to_ne_bytes()); }
    pub fn dw(f: &mut Vec<u8>, v: u16)   { f.extend_from_slice(&v.to_ne_bytes()); }
    pub fn db(f: &mut Vec<u8>, v: u8)    { f.extend_from_slice(&[v]); }
}
