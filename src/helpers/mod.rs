use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Lines};

pub struct DayData(String);

impl DayData {
    pub fn from_file_path(path: &str) -> DayData {
        DayData(path.to_string())
    }

    pub fn to_string(&self) -> String {
        let f = read_to_string(&self.0).expect("input not found");
        f
    }

    pub fn lines(&self) -> DayDataLineIterator {
        DayDataLineIterator::new(self)
    }
}

pub struct DayDataLineIterator<'a>(&'a DayData, Lines<BufReader<File>>);

impl<'a> DayDataLineIterator<'a> {
    fn new(data: &'a DayData) -> DayDataLineIterator {
        let f = File::open(&data.0).expect("input not found");
        let f = BufReader::new(f);
        DayDataLineIterator(data, f.lines())
    }
}

impl<'a> Clone for DayDataLineIterator<'a> {
    fn clone(&self) -> Self {
        DayDataLineIterator::new(self.0)
    }
}

impl<'a> Iterator for DayDataLineIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|s| s.unwrap())
    }
}
