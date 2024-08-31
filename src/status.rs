use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    Paralyse,
    Burn,
    Poison,
    Toxic,
    Sleep,
    Freeze,
    Flinch,
    Confusion,
    Drowsy,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Paralyse => String::from("PAR"),
                Status::Burn => String::from("BRN"),
                Status::Poison => String::from("PSN"),
                Status::Toxic => String::from("TOX"),
                Status::Sleep => String::from("SLP"),
                Status::Freeze => String::from("FRZ"),
                Status::Flinch => String::from("flinched"),
                Status::Confusion => String::from("confused"),
                Status::Drowsy => String::from("drowsy"),
            }
        )
    }
}
