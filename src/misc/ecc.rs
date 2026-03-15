
// ECC symbol/codeword in u8 bins
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ECCWord {
    pub sym: Vec<u8>,     // raw u8 symbols
    pub ok:  u8,          // 1 if decodable, 0 otherwise (u8 boolean)
}
