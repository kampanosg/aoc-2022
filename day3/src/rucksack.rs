#[derive(Debug)]
pub struct Rucksack {
    name: String,
}

impl Rucksack {
    pub fn new(val: &str) -> Rucksack {
        return Rucksack {
            name: String::from(val),
        };
    }

    pub fn size(&self) -> usize {
        self.name.chars().count()
    }

    pub fn compartments(&self) -> (String, String) {
        (self.first_compartment(), self.second_compartment())
    }

    fn first_compartment(&self) -> String {
        if self.size() < 2 {
            return self.name.to_string();
        }
        let half = self.size() / 2;
        self.name[0..half].to_string()
    }

    fn second_compartment(&self) -> String {
        if self.size() < 2 {
            return self.name.to_string();
        }
        let half = self.size() / 2;
        let end = self.size();
        self.name[half..end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Rucksack;

    #[test]
    fn test_size_given_empty_name_then_return_zero() {
        let r = Rucksack::new("");
        assert_eq!(r.size(), 0)
    }

    #[test]
    fn test_size_given_populated_name_then_return_correct_size() {
        let r = Rucksack::new("the_quick_brown_fox");
        assert_eq!(r.size(), 19)
    }

    #[test]
    fn test_first_compartment_given_empty_name_then_return_empty_str() {
        let r = Rucksack::new("");
        assert_eq!(r.first_compartment(), "")
    }

    #[test]
    fn test_first_compartment_given_single_char_name_then_return_single_char() {
        let r = Rucksack::new("g");
        assert_eq!(r.first_compartment(), "g")
    }

    #[test]
    fn test_first_compartment_given_double_char_name_then_return_first_char() {
        let r = Rucksack::new("gk");
        assert_eq!(r.first_compartment(), "g")
    }

    #[test]
    fn test_first_compartment_given_even_len_name_then_return_first_half() {
        let r = Rucksack::new("abcdEFGH");
        assert_eq!(r.first_compartment(), "abcd")
    }

    #[test]
    fn test_first_compartment_given_odd_len_name_then_return_first_half() {
        let r = Rucksack::new("abcDEFG");
        assert_eq!(r.first_compartment(), "abc")
    }

    #[test]
    fn test_first_compartment_given_long_even_len_name_then_return_first_half() {
        let r = Rucksack::new("the_quick_brown_fox_THE_QUICK_BROWN_FOX_");
        assert_eq!(r.first_compartment(), "the_quick_brown_fox_")
    }

    #[test]
    fn test_second_compartment_given_empty_name_then_return_empty_str() {
        let r = Rucksack::new("");
        assert_eq!(r.second_compartment(), "")
    }

    #[test]
    fn test_second_compartment_given_single_char_name_then_return_single_char() {
        let r = Rucksack::new("g");
        assert_eq!(r.second_compartment(), "g")
    }

    #[test]
    fn test_second_compartment_given_double_char_name_then_return_second_char() {
        let r = Rucksack::new("gk");
        assert_eq!(r.second_compartment(), "k")
    }

    #[test]
    fn test_second_compartment_given_even_len_name_then_return_first_half() {
        let r = Rucksack::new("abcdEFGH");
        assert_eq!(r.second_compartment(), "EFGH")
    }

    #[test]
    fn test_second_compartment_given_odd_len_name_then_return_first_half() {
        let r = Rucksack::new("abcDEFG");
        assert_eq!(r.second_compartment(), "DEFG")
    }

    #[test]
    fn test_second_compartment_given_long_even_len_name_then_return_first_half() {
        let r = Rucksack::new("the_quick_brown_fox_THE_QUICK_BROWN_FOX_");
        assert_eq!(r.second_compartment(), "THE_QUICK_BROWN_FOX_")
    }
}
