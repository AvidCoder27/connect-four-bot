use std::fmt::{Debug, Display};

use colored::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Yellow,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gameover {
    Win(crate::color::Color),
    Tie,
    None,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Yellow => write!(f, "{}", "Yellow".yellow()),
            Color::Red => write!(f, "{}", "Red".red()),
        }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Yellow => write!(f, "{}", "Y".yellow()),
            Color::Red => write!(f, "{}", "R".red()),
        }
    }
}

impl Color {
    pub const fn opposite(&self) -> Color {
        match self {
            Color::Yellow => Color::Red,
            Color::Red => Color::Yellow,
        }
    }
}
