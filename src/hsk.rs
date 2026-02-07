use file_to_array_macro::include_lines;

pub struct HSKLists<'a> {
    hsk1: [&'a str; 499],
    hsk2: [&'a str; 772],
    hsk3: [&'a str; 973],
    hsk4: [&'a str; 1000],
    hsk5: [&'a str; 1071],
    hsk6: [&'a str; 1132],
    hsk7: [&'a str; 5435],
}

pub type Hsk = u8;

pub const HSK: HSKLists<'static> = HSKLists {
    hsk1: include_lines!("../../data/HSK1.txt"),
    hsk2: include_lines!("../../data/HSK2.txt"),
    hsk3: include_lines!("../../data/HSK3.txt"),
    hsk4: include_lines!("../../data/HSK4.txt"),
    hsk5: include_lines!("../../data/HSK5.txt"),
    hsk6: include_lines!("../../data/HSK6.txt"),
    hsk7: include_lines!("../../data/HSK7.txt"),
};

impl HSKLists<'_> {
    pub fn level(&self, query: &str) -> Option<Hsk> {
        if self.hsk1.contains(&query) {
            Some(1)
        } else if self.hsk2.contains(&query) {
            Some(2)
        } else if self.hsk3.contains(&query) {
            Some(3)
        } else if self.hsk4.contains(&query) {
            Some(4)
        } else if self.hsk5.contains(&query) {
            Some(5)
        } else if self.hsk6.contains(&query) {
            Some(6)
        } else if self.hsk7.contains(&query) {
            Some(7)
        } else {
            None
        }

        // let hsks: [&[&str]; 6] = [
        //     &self.hsk1, &self.hsk2, &self.hsk3, &self.hsk4, &self.hsk5, &self.hsk6,
        // ];
        // hsks.iter()
        //     .enumerate()
        //     .find(|(_, els)| els.contains(&query))
        //     .map(|(level, _)| (level as Hsk) + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::HSK;

    #[test]
    fn hsk_setup() {
        assert_eq!(HSK.hsk1.contains(&"我"), true);
        assert_eq!(HSK.hsk2.contains(&"我"), false);
        assert_eq!(HSK.hsk3.contains(&"我"), false);
        assert_eq!(HSK.hsk4.contains(&"我"), false);
        assert_eq!(HSK.hsk5.contains(&"我"), false);
        assert_eq!(HSK.hsk6.contains(&"我"), false);

        assert_eq!(HSK.hsk1.contains(&"就"), false);
        assert_eq!(HSK.hsk2.contains(&"就"), true);
        assert_eq!(HSK.hsk3.contains(&"就"), false);
        assert_eq!(HSK.hsk4.contains(&"就"), false);
        assert_eq!(HSK.hsk5.contains(&"就"), false);
        assert_eq!(HSK.hsk6.contains(&"就"), false);
    }

    #[test]
    fn can_query_hsk() {
       assert_eq!(HSK.level("我"), Some(1));
       assert_eq!(HSK.level("就"), Some(2));
       assert_eq!(HSK.level("还"), Some(2));
       assert_eq!(HSK.level("嘿"), Some(6));
       assert_eq!(HSK.level("我就"), None);    
    }
}
