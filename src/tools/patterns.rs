pub fn parse_pattern(pattern: &str) -> Vec<Option<u8>> {
    let mut aob: Vec<Option<u8>> = Vec::new();

    for part in pattern.split_whitespace() {
        if part == "??" {
            aob.push(None);
        } else {
            aob.push(Some(u8::from_str_radix(part, 16).unwrap()));
        }
    }
    
    aob
}

// suports wildcards
pub fn pattern_scanner(aob: &[u8], pattern: &[Option<u8>]) -> Vec<usize> {
    let mut internal_match: bool = false;
    let mut positions: Vec<usize> = Vec::new();

    for i in 0..aob.len().saturating_sub(pattern.len()) {
        internal_match = true;

        for j in 0..pattern.len() {
            if pattern[j] == None{
                continue;
            }
            else if let Some(byte) = pattern[j] {
                if byte == aob[i+j] {
                    continue;
                }
                else{
                    internal_match = false;
                    break;
                }
            }
        }

        if internal_match {
            positions.push(i)
        }
    }

    positions
}