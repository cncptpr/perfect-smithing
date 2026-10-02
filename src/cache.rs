use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
};

use crate::APSPResult;

/// Why a cached result cannot be reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheError {
    /// The cached matrices cover a different range of positions.
    SizeMismatch,
    /// The cached matrices were built from different steps.
    StepsMismatch,
}

/// Pure reuse check for a loaded cache entry.
pub fn validate(result: &APSPResult, size: usize, steps: &[i64]) -> Result<(), CacheError> {
    if result.distance.size != size || result.predecessor.size != size {
        return Err(CacheError::SizeMismatch);
    }
    if result.steps.as_slice() != steps {
        return Err(CacheError::StepsMismatch);
    }
    Ok(())
}

fn from_reader(reader: impl io::Read, size: usize, steps: &[i64]) -> Option<APSPResult> {
    let result: APSPResult = serde_json::from_reader(reader).ok()?;
    validate(&result, size, steps).ok()?;
    Some(result)
}

pub fn load_cache(size: usize, steps: &[i64]) -> Option<APSPResult> {
    let path = format!("cache/result-{size}.json");
    from_reader(BufReader::new(File::open(path).ok()?), size, steps)
}

pub fn store_cache(result: &APSPResult) -> io::Result<()> {
    std::fs::create_dir_all("cache/")?;
    let path = format!("cache/result-{}.json", result.distance.size);
    let writer = BufWriter::new(File::create(path)?);
    serde_json::to_writer(writer, result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::floyd_warshall;

    const SIZE: usize = 5;
    const STEPS: [i64; 2] = [-1, 1];

    fn fixture() -> APSPResult {
        floyd_warshall(SIZE as i64, STEPS.to_vec())
    }

    #[test]
    fn accepts_matching_size_and_steps() {
        assert_eq!(validate(&fixture(), SIZE, &STEPS), Ok(()));
    }

    #[test]
    fn rejects_size_mismatch() {
        assert_eq!(
            validate(&fixture(), SIZE + 1, &STEPS),
            Err(CacheError::SizeMismatch)
        );
    }

    #[test]
    fn rejects_steps_mismatch() {
        assert_eq!(
            validate(&fixture(), SIZE, &[1, 2]),
            Err(CacheError::StepsMismatch)
        );
    }

    #[test]
    fn rejects_cache_written_before_steps_were_stored() {
        // The old format simply lacks the field, so parsing already fails.
        let mut legacy = serde_json::to_value(fixture()).unwrap();
        legacy.as_object_mut().unwrap().remove("steps");

        assert!(serde_json::from_value::<APSPResult>(legacy).is_err());
    }

    #[test]
    fn round_trips_through_json() {
        let original = fixture();
        let parsed: APSPResult =
            serde_json::from_value(serde_json::to_value(&original).unwrap()).unwrap();

        assert_eq!(validate(&parsed, SIZE, &STEPS), Ok(()));
        assert_eq!(parsed.steps, STEPS);
    }
}
