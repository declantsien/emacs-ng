use clap::builder::PossibleValue;
// use emacs_sys::lisp::LispObject;
use clap::ValueEnum;

/// Represents the color preferences for program output
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DumpMethod {
    Dump,
    Bootstrap,
}

impl DumpMethod {
    /// Report all `possible_values`
    pub fn possible_values() -> impl Iterator<Item = PossibleValue> {
        Self::value_variants()
            .iter()
            .filter_map(ValueEnum::to_possible_value)
    }
}

impl std::fmt::Display for DumpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for DumpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

impl ValueEnum for DumpMethod {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dump, Self::Bootstrap]
    }

    #[cfg(have_unexec)]
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Dump => PossibleValue::new("dump"),
            Self::Bootstrap => PossibleValue::new("bootstrap"),
        })
    }

    #[cfg(have_pdumper)]
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Dump => PossibleValue::new("pdump"),
            Self::Bootstrap => PossibleValue::new("pbootstrap"),
        })
    }
}

// impl into/from for lispobject
