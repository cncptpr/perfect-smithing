/// The CSS class colouring one of the eight hits.
pub fn class_for(value: i64) -> &'static str {
    match value {
        -15 => "hit-n15",
        -6 => "hit-n6",
        -5 => "hit-n5",
        -3 => "hit-n3",
        2 => "hit-2",
        7 => "hit-7",
        13 => "hit-13",
        16 => "hit-16",
        _ => "hit-unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve::DEFAULT_STEPS;

    #[test]
    fn every_hit_gets_its_own_class() {
        let classes: Vec<_> = DEFAULT_STEPS.into_iter().map(class_for).collect();

        assert_eq!(
            classes,
            [
                "hit-n15", "hit-n6", "hit-n5", "hit-n3", "hit-2", "hit-7", "hit-13", "hit-16",
            ]
        );
    }

    #[test]
    fn values_outside_the_hits_get_no_class() {
        assert_eq!(class_for(0), "hit-unknown");
        assert_eq!(class_for(99), "hit-unknown");
    }
}
