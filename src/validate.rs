//! Helpers pour valider les descriptions (nombre de mots).
//!
//! # Examples
//!
//! ```
//! use fill_prompt::validate::{validate_short, ValidationError};
//! let s = "Phrase courte exemple";
//! assert!(validate_short(s).is_ok());
//! ```
use thiserror::Error;

/// Erreurs de validation des descriptions.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("trop long: {0} mots (max {1})")]
    TooManyWords(usize, usize),
}

/// Compte les mots (séparateur whitespace) de façon simple.
fn word_count(s: &str) -> usize {
    s.split_whitespace().filter(|w| !w.is_empty()).count()
}

/// Valide la phrase courte (<= 30 mots).
pub fn validate_short(s: &str) -> Result<(), ValidationError> {
    let c = word_count(s);
    if c <= 30 {
        Ok(())
    } else {
        Err(ValidationError::TooManyWords(c, 30))
    }
}

/// Valide le paragraphe de contexte (<= 40 mots).
pub fn validate_context(s: &str) -> Result<(), ValidationError> {
    let c = word_count(s);
    if c <= 40 {
        Ok(())
    } else {
        Err(ValidationError::TooManyWords(c, 40))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_ok() {
        let s = "Une phrase très courte.";
        assert_eq!(validate_short(s), Ok(()));
    }

    #[test]
    fn short_too_long() {
        let s = (0..31)
            .map(|i| format!("w{}", i))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(matches!(
            validate_short(&s),
            Err(ValidationError::TooManyWords(_, 30))
        ));
    }

    #[test]
    fn context_ok() {
        let s = "Un paragraphe concis en quelques mots seulement.";
        assert_eq!(validate_context(s), Ok(()));
    }

    #[test]
    fn context_too_long() {
        let s = (0..41)
            .map(|i| format!("x{}", i))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(matches!(
            validate_context(&s),
            Err(ValidationError::TooManyWords(_, 40))
        ));
    }
}
