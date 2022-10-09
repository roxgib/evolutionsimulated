use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EyeColour {
    Green,
    Blue,
    Purple,
}

impl EyeColour {
    pub fn new_random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x: u8 = rng.gen();
        match x % 3 {
            0 => EyeColour::Green,
            1 => EyeColour::Blue,
            2 => EyeColour::Purple,
            _ => EyeColour::Green,
        }
    }

    pub fn colour(first: EyeColour, second: EyeColour) -> EyeColour {
        if (first as u8) < (second as u8) {
            first
        } else {
            second
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SkinColour {
    Green,
    Red,
    Yellow,
}

impl SkinColour {
    pub fn new_random() -> SkinColour {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x: u8 = rng.gen();
        match x % 4 {
            0 => SkinColour::Green,
            1 => SkinColour::Yellow,
            2 => SkinColour::Red,
            _ => SkinColour::Red,
        }
    }

    pub fn colour(first: SkinColour, second: SkinColour) -> SkinColour {
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
            Speed::Medium(a) => if *a { write!(f, "Medium (d)") } else { write!(f, "Medium(r)") },
            Speed::Slow(a) => if *a { write!(f, "Slow (d)") } else { write!(f, "Slow(r)") },
        }
    }
}