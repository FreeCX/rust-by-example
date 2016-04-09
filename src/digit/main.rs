pub trait StrDigit {
    fn dec_to_bin(&self) -> String;
    fn bin_to_dec(&self) -> String;
}

impl StrDigit for str {
    fn dec_to_bin(&self) -> String {
        let mut result = String::new();
        let mut number: usize = match self.parse() {
            Ok(value) => value,
            Err(why) => panic!("{}", why),
        };
        while number != 0 {
            result.push(match number % 2 {
                0 => '0',
                1 => '1',
                _ => continue,
            });
            number /= 2;
        }
        // reverse string
        result.chars()
              .rev()
              .collect()
    }
    fn bin_to_dec(&self) -> String {
        let mut result: usize = 0;
        let mut counter = self.len() as u32;
        for one in self.chars() {
            result += match one {
                '1' => 2usize.pow(counter - 1),
                _ => 0,
            };
            counter -= 1;
        }
        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dec_bin() {
        // ( bin, dec )
        let vector = vec![("101010", "42"),
                          ("1100101", "101"),
                          ("1000101011100", "4444"),
                          ("10000000000", "1024")];
        for (bin, dec) in vector {
            let test01 = dec.dec_to_bin();
            let test02 = test01.bin_to_dec();
            assert_eq!(test01, bin);
            assert_eq!(test02, dec);
        }
    }
}

fn main() {
    let n = "42";
    let test01 = n.dec_to_bin();
    let test02 = test01.bin_to_dec();
    println!("{}_10 -> {}_2 -> {}_10", n, test01, test02);
}
