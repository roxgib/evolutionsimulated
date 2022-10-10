use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ColourGene {
    Orange = 0,
    Red = 1,
    LBlue = 2,
    DBlue = 3,
    Black = 4,
    Yellow = 5,
    Purple = 6,
}

impl ColourGene {
    pub fn new_random() -> ColourGene {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x: u8 = rng.gen();
        match x % 7 {
            0 => ColourGene::Orange,
            1 => ColourGene::Red,
            2 => ColourGene::LBlue,
            3 => ColourGene::DBlue,
            4 => ColourGene::Black,
            5 => ColourGene::Yellow,
            6 => ColourGene::Purple,
            _ => unreachable!(),
        }
    }

    pub fn colour(first: ColourGene, second: ColourGene) -> ColourGene {
        if (first as u8) < (second as u8) {
            first
        } else {
            second
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Speed {
    Slow(bool),
    Medium(bool),
    Fast,
}

impl Speed {
    pub fn new_random() -> Speed {
        let x: u8 = rand::random();
        let y: bool = rand::random();
        match x % 3 {
            0 => Speed::Slow(y),
            1 => Speed::Medium(y),
            2 => Speed::Fast,
            _ => Speed::Slow(y),
        }
    }

    pub fn speed(first: Speed, second: Speed) -> Speed {
        match (first, second) {
            (Speed::Fast, Speed::Fast) => Speed::Fast,
            (Speed::Fast, _) => first,
            (_, Speed::Fast) => second,
            (Speed::Slow(_), Speed::Slow(_)) => first,
            (Speed::Medium(_), Speed::Medium(_)) => first,
            (Speed::Slow(a), Speed::Medium(b)) => {
                if a == b {
                    first
                } else if a {
                    first
                } else {
                    second
                }
            },
            (Speed::Medium(a), Speed::Slow(b)) => {
                if a == b {
                    second
                } else if a {
                    first
                } else {
                    second
                }
            }
        }
    }
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Speed::Fast => write!(f, "Fast"),
            Speed::Medium(a) => if *a { write!(f, "Med(d)") } else { write!(f, "Med(r)") },
            Speed::Slow(a) => if *a { write!(f, "Slow(d)") } else { write!(f, "Slow(r)") },
        }
    }
}

impl Display for ColourGene {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColourGene::Orange => write!(f, "Orange"),
            ColourGene::Red => write!(f, "Red"),
            ColourGene::LBlue => write!(f, "LBlue"),
            ColourGene::DBlue => write!(f, "DBlue"),
            ColourGene::Black => write!(f, "Black"),
            ColourGene::Yellow => write!(f, "Yellow"),
            ColourGene::Purple => write!(f, "Purple"),
        }
    }
}