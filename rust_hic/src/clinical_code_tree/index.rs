use serde::{Serialize, Deserialize};

/// Index used to sort the code categories.
/// 
/// The index contains one or two string values.
/// The first value, which is required, is the
/// lexicographical start point for the category.
/// For example, the top level ICD-10 code category
/// begins with index A00. The second value, which 
/// is present only for proper categories, defines
/// the end-point of the category. For example, 
/// the category ending B99 includes all categories
/// and codes whose name (without the dot) truncated
/// to three characters is lexicographically less than
/// or equal to B99.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Index {
    Leaf(String),
    Category(String, String)
}

impl Index {
    pub fn make_leaf(start: &str) -> Self {
       Self::Leaf(String::from(start))
    }

    pub fn make_category(start: &str, end: &str) -> Self {
        Self::Category(String::from(start), String::from(end))
     }
}

/// Tests for the index
/// 
/// The main thing to check is that the index compares correctly
/// with itself so that the sorting function in the code tree works
/// correctly.
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn single_element_index_compares_for_equality() {
        let i1 = Index::make_leaf("ABC");
        let i2 = Index::make_leaf("ABC");
        assert_eq!(i1, i2);
    }

    #[test]
    fn two_element_index_compares_for_equality() {
        let i1 = Index::make_category("ABC", "xyz");
        let i2 = Index::make_category("ABC", "xyz");
        assert_eq!(i1, i2);
    }

    #[test]
    fn single_element_index_compare_inequality() {
        let i1 = Index::make_leaf("A00");
        let i2 = Index::make_leaf("A01");
        assert!(i1 < i2);

        let i1 = Index::make_leaf("i212");
        let i2 = Index::make_leaf("i222");
        assert!(i1 < i2);
    }

    /// As far as sorting goes, priority should be given to the first
    /// index (the start of the range). 
    #[test]
    fn two_element_index_compare_inequality_first_element() {
        let i1 = Index::make_category("A00", "Z00");
        let i2 = Index::make_category("A01", "X00");
        assert!(i1 < i2);
    }

    #[test]
    fn two_element_index_compare_inequality_second_element() {
        let i1 = Index::make_category("I00", "I01");
        let i2 = Index::make_category("I00", "I02");
        assert!(i2 > i1);
    }

}