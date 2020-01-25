#[derive(Debug, PartialEq)]
pub struct DNA { nucs: Vec<char> }

#[derive(Debug, PartialEq)]
pub struct RNA { nucs: Vec<char> }

fn map (nuc: char) -> char {
    match nuc {
        'A' => 'U',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => unreachable!(),
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut d = DNA { nucs: Vec::new() };

        for (i, v) in dna.chars().enumerate() {
            match v {
                'A'|'C'|'G'|'T' => d.nucs.push(v),
                _ => return Err(i),
            }
        }
        Ok(d)
    }

    pub fn into_rna(self) -> RNA {
        let mut nucs: Vec<char> = Vec::new();

        for v in self.nucs {
            nucs.push(map(v));
        }
        RNA { nucs }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut r = RNA { nucs: Vec::new() };

        for (i, v) in rna.chars().enumerate() {
            match v {
                'A'|'C'|'G'|'U' => r.nucs.push(v),
                _ => return Err(i),
            }
        }
        Ok(r)
    }
}
