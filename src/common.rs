use derive_more::Display;

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Alignment {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

/// Common size classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Size {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}
