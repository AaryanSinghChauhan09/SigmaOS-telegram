//! Options model.

#[derive(Debug, Clone, PartialEq)]
pub enum Mode { Default, Chat(String, String, String), Send(String, String, String), Search(String), Secret(String) }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert!(matches!(Options::default().mode, Mode::Default)); } }
