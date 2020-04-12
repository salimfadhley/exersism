use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

static DNA_BASES: [char; 4] = ['G', 'C', 'T', 'A'];
static RNA_BASES: [char; 4] = ['C', 'G', 'A', 'U'];

pub fn find_invalid_indexes(valid_bases: [char; 4], bases: &str) -> Vec<usize> {
    bases
        .chars()
        .enumerate()
        .filter(|(_a, b)| !valid_bases.contains(b))
        .map(|(q, _r)| q)
        .collect()
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match find_invalid_indexes(DNA_BASES, dna) {
            a if a.is_empty() => Ok(DNA {
                dna: String::from(dna),
            }),
            a => Err(*a.get(0).unwrap()),
        }
    }

    pub fn into_rna(self) -> RNA {
        let m1: HashMap<&char, &char> = DNA_BASES.iter().zip(RNA_BASES.iter()).collect();
        let rna: String = self
            .dna
            .chars()
            .map(|x| m1.get(&x).unwrap())
            .map(|z| **z)
            .collect();
        RNA { rna }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match find_invalid_indexes(RNA_BASES, rna) {
            a if a.is_empty() => Ok(RNA {
                rna: String::from(rna),
            }),
            a => Err(*a.get(0).unwrap()),
        }
    }
}
