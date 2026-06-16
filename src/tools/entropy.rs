pub fn entropy_blocks(data: &[u8], block_size: usize) -> Vec<f64> {
    data.chunks(block_size)
        .map(|block| {
            let mut counts = [0usize; 256];

            for &byte in block {
                counts[byte as usize] += 1;
            }

            let len = block.len() as f64;

            counts
                .iter()
                .filter(|&&c| c > 0)
                .map(|&c| {
                    let p = c as f64 / len;
                    -p * p.log2()
                })
                .sum()
        })
        .collect()
}