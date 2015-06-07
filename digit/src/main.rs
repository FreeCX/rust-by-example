trait StrDigit {
    fn dec_to_bin( &self ) -> String;
    fn bin_to_dec( &self ) -> String;
}

impl StrDigit for str {
    fn dec_to_bin( &self ) -> String {
        let mut result = String::new();
        let mut number: usize = match self.parse() {
            Ok( value ) => value,
            Err( why ) => panic!( "{}", why )
        };
        while number != 0 {
            result.push( match number % 2 {
                0 => '0',
                1 => '1',
                _ => continue
            } );
            number /= 2;
        }
        unsafe {
            let vec = result.as_mut_vec();
            vec.reverse()
        }
        result
    }
    fn bin_to_dec( &self ) -> String {
        let mut result: usize = 0;
        let mut counter = self.len() as u32;
        for one in self.chars() {
            result += match one {
                '1' => 2usize.pow( counter - 1 ),
                _ => 0
            };
            counter -= 1;
        }
        result.to_string()
    }
}

fn main() {
    let test01 = "11001";
    let test02 = "42";
    println!( "{}_2 -> {}_10", test01, test01.bin_to_dec() );
    println!( "{}_10 -> {}_2", test02, test02.dec_to_bin() );
}
