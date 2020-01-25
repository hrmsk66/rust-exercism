#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match dna.find(|c: char| !"ACGT".contains(c)) {
            Some(val) => Err(val),
            None => Ok(DNA(String::from(dna)))
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA(self.0.chars().map(|c| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => unreachable!(),
        }).collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna.find(|c: char| !"ACGU".contains(c)) {
            Some(val) => Err(val),
            None => Ok(RNA(String::from(rna)))
        }
    }
}