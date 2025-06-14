//! Font character width measurer for SVG badge rendering.
//!
//! This module provides [`CharWidthMeasurer`], a utility for loading and consuming font width tables
//! (from JSON or string), and for calculating the width of strings in a given font. It is equivalent
//! to the JS CharWidthTableConsumer used in shields.io, and is used internally for accurate badge layout.
//!
//! # Typical Usage
//!
//! ```rust
//! use shields::measurer::CharWidthMeasurer;
//! let data = vec![(65, 90, 10.0), (97, 122, 8.0)]; // A-Z width 10, a-z width 8
//! let measurer = CharWidthMeasurer::from_data(data);
//! let width = measurer.width_of("Hello", true);
//! assert!(width > 0.0);
//! ```
//!
//! See [`CharWidthMeasurer`] for details.

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self};

/// Measures character widths for a given font, for use in SVG badge layout.
///
/// This struct loads a font width table (from data, JSON file, or string) and provides methods
/// to look up the width of individual characters or entire strings. Used internally for accurate
/// badge rendering.
///
/// ## Example
/// ```rust
/// use shields::measurer::CharWidthMeasurer;
/// let data = vec![(65, 90, 10.0), (97, 122, 8.0)];
/// let measurer = CharWidthMeasurer::from_data(data);
/// let width = measurer.width_of("Hello", true);
/// assert!(width > 0.0);
/// ```
pub struct CharWidthMeasurer {
    /// Lookup table: char_code -> width
    hash_map: HashMap<u32, f64>,
    /// Width of character 'm'
    pub em_width: f64,
}

impl CharWidthMeasurer {
    /// Returns true if the given character code is a control character (ASCII 0-31 or 127).
    ///
    /// # Arguments
    /// * `char_code` - Unicode code point.
    ///
    /// # Returns
    /// `true` if control character, else `false`.
    pub fn is_control_char(char_code: u32) -> bool {
        char_code <= 31 || char_code == 127
    }

    /// Creates a new measurer from a vector of (lower, upper, width) tuples.
    ///
    /// Each tuple defines a range of character codes and their width.
    ///
    /// # Arguments
    /// * `data` - Vector of (lower, upper, width) tuples.
    ///
    /// # Returns
    /// A new [`CharWidthMeasurer`].
    ///
    /// ## Example
    /// ```
    /// use shields::measurer::CharWidthMeasurer;
    /// let data = vec![(65, 90, 10.0), (97, 122, 8.0)];
    /// let measurer = CharWidthMeasurer::from_data(data);
    /// ```
    pub fn from_data(data: Vec<(u32, u32, f64)>) -> Self {
        // Build lookup table: expand all ranges to char_code -> width
        let mut hash_map = HashMap::new();
        for &(lower, upper, width) in &data {
            for code in lower..=upper {
                hash_map.insert(code, width);
            }
        }
        // emWidth is the width of character 'm'
        let mut consumer = CharWidthMeasurer {
            hash_map,
            em_width: 0.0,
        };
        consumer.em_width = consumer.width_of("m", true);
        consumer
    }

    /// Loads a measurer from a JSON file (synchronously).
    ///
    /// # Arguments
    /// * `path` - Path to the JSON file.
    ///
    /// # Returns
    /// `Ok(CharWidthMeasurer)` if successful, or an `io::Error`.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_sync(path: &str) -> io::Result<Self> {
        let json_str = fs::read_to_string(path)?;
        let value: Value = serde_json::from_str(&json_str)?;
        let arr = value
            .as_array()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "JSON is not an array"))?;
        let mut data = Vec::with_capacity(arr.len());
        for item in arr {
            let triple = item.as_array().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Subitem is not an array")
            })?;
            let lower = triple[0].as_u64().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "lower is not an integer")
            })? as u32;
            let upper = triple[1].as_u64().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "upper is not an integer")
            })? as u32;
            let width = triple[2].as_f64().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "width is not a float")
            })?;
            data.push((lower, upper, width));
        }
        Ok(CharWidthMeasurer::from_data(data))
    }

    /// Loads a measurer from a JSON string.
    ///
    /// # Arguments
    /// * `data` - JSON string.
    ///
    /// # Returns
    /// `Ok(CharWidthMeasurer)` if successful, or an `io::Error`.
    ///
    /// # Errors
    /// Returns an error if the string cannot be parsed.
    pub fn load_from_str(data: &str) -> io::Result<Self> {
        let value: Value = serde_json::from_str(data)?;
        let arr = value
            .as_array()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "JSON is not an array"))?;
        let mut data = Vec::with_capacity(arr.len());
        for item in arr {
            let triple = item.as_array().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Subitem is not an array")
            })?;
            let lower = triple[0].as_u64().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "lower is not an integer")
            })? as u32;
            let upper = triple[1].as_u64().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "upper is not an integer")
            })? as u32;
            let width = triple[2].as_f64().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "width is not a float")
            })?;
            data.push((lower, upper, width));
        }
        Ok(CharWidthMeasurer::from_data(data))
    }

    /// Looks up the width of a single character code.
    ///
    /// Control characters have width 0. Returns `None` if not found.
    ///
    /// # Arguments
    /// * `char_code` - Unicode code point.
    ///
    /// # Returns
    /// Some(width) if found, or None.
    ///
    /// ## Example
    /// ```
    /// use shields::measurer::CharWidthMeasurer;
    /// let data = vec![(65, 90, 10.0)];
    /// let measurer = CharWidthMeasurer::from_data(data);
    /// assert_eq!(measurer.width_of_char_code(65), Some(10.0));
    /// ```
    pub fn width_of_char_code(&self, char_code: u32) -> Option<f64> {
        if Self::is_control_char(char_code) {
            return Some(0.0);
        }
        // Directly use the hash table to look up character width
        // The lookup table has already expanded all ranges to char_code -> width during initialization
        self.hash_map.get(&char_code).copied()
    }

    /// Calculates the width of a string.
    ///
    /// If `guess` is true, uses `em_width` for unknown characters; otherwise panics.
    ///
    /// # Arguments
    /// * `text` - The string to measure.
    /// * `guess` - Whether to guess width for unknown characters.
    ///
    /// # Returns
    /// Total width of the string.
    ///
    /// # Panics
    /// If `guess` is false and an unknown character is encountered.
    ///
    /// ## Example
    /// ```
    /// use shields::measurer::CharWidthMeasurer;
    /// let data = vec![(65, 90, 10.0)];
    /// let measurer = CharWidthMeasurer::from_data(data);
    /// let width = measurer.width_of("ABC", true);
    /// assert_eq!(width, 30.0);
    /// ```
    pub fn width_of(&self, text: &str, guess: bool) -> f64 {
        let mut total = 0.0;
        for ch in text.chars() {
            let code = ch as u32;
            match self.width_of_char_code(code) {
                Some(width) => total += width,
                None => {
                    if guess {
                        total += self.em_width;
                    } else {
                        panic!("No width available for character code {}", text);
                    }
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_chars() {
        assert!(CharWidthMeasurer::is_control_char(0));
        assert!(CharWidthMeasurer::is_control_char(31));
        assert!(CharWidthMeasurer::is_control_char(127));
        assert!(!CharWidthMeasurer::is_control_char(32));
        assert!(!CharWidthMeasurer::is_control_char(128));
    }

    #[test]
    fn test_from_data() {
        let data = vec![(65, 90, 10.0), (97, 122, 8.0)]; // A-Z width 10, a-z width 8
        let measurer = CharWidthMeasurer::from_data(data);

        assert_eq!(measurer.width_of_char_code(65), Some(10.0)); // 'A'
        assert_eq!(measurer.width_of_char_code(90), Some(10.0)); // 'Z'
        assert_eq!(measurer.width_of_char_code(97), Some(8.0)); // 'a'
        assert_eq!(measurer.width_of_char_code(122), Some(8.0)); // 'z'
        assert_eq!(measurer.width_of_char_code(64), None); // '@'
    }

    #[test]
    fn test_width_of() {
        let data = vec![
            (65, 90, 10.0),   // A-Z width 10
            (97, 122, 8.0),   // a-z width 8
            (109, 109, 16.0), // Set width of 'm' to 16 for testing
        ];
        let measurer = CharWidthMeasurer::from_data(data);

        // Check if em_width is set correctly
        assert_eq!(measurer.em_width, 16.0);

        // Test string width calculation
        assert_eq!(measurer.width_of("ABC", true), 30.0);
        assert_eq!(measurer.width_of("abc", true), 24.0);
        assert_eq!(measurer.width_of("Am", true), 26.0);

        // Test guess mode for unknown characters
    }

    #[test]
    #[should_panic(expected = "No width available for character code")]
    fn test_width_of_no_guess() {
        let data = vec![(65, 90, 10.0)];
        let measurer = CharWidthMeasurer::from_data(data);
        measurer.width_of("A測", false); // Should panic for unknown character '測'
    }
}
