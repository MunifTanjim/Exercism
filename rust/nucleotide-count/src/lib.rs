use std::collections::HashMap;

macro_rules! with_nucleotide {
    ($nucleotide: ident, $operation: expr) => {
        match $nucleotide as u8 {
            b'A' | b'C' | b'G' | b'T' => $operation,
            c => return Err(char::from(c)),
        }
    };
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    with_nucleotide!(
        nucleotide,
        dna.bytes().try_fold(0_usize, |c, b| {
            Ok(c + with_nucleotide!(b, (b == nucleotide as u8) as usize))
        })
    )
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    Ok(HashMap::from_iter(vec![
        ('A', count('A', dna)?),
        ('C', count('C', dna)?),
        ('G', count('G', dna)?),
        ('T', count('T', dna)?),
    ]))
}
