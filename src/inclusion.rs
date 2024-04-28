use clap::Parser;

#[derive(Debug, Parser)]
struct Inclusion {
    #[arg(long)]
    hex: String
}

impl Inclusion {

    fn to_binary(c: char) -> &'static str {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' | 'a' => "1010",
            'B' | 'b' => "1011",
            'C' | 'c' => "1100",
            'D' | 'd' => "1101",
            'E' | 'e' => "1110",
            'F' | 'f' => "1111",
            _ => "",
        }
    }

    fn binary(&self) -> String {
        self.hex[2..].chars().map(Self::to_binary).collect()
    }
    fn hex_to_binary(&self) -> String {
        let binary_string = if let Ok(hex_value) = usize::from_str_radix(self.hex.trim_start_matches("0x"), 16) {
            let binary_string = format!("{:0b}", hex_value);
            binary_string
        } else {
            "".into()
        };
        binary_string
    }

}

fn main() {
    let inclusion = Inclusion::parse();
    println!("{:?}", inclusion.binary());
}

#[test]
fn test_inclusion() {
    let inclusion = Inclusion { hex: "0xFFFFFFFFFFFE".into() };
    assert_eq!(inclusion.hex_to_binary(), "111111111111111111111111111111111111111111111110");

    let inclusion = Inclusion { hex: "0xffff8".into() };
    assert_eq!(inclusion.hex_to_binary(), "11111111111111111000");

    let inclusion = Inclusion { hex: "0x1FD3C00800000000".into() };
    assert_eq!(inclusion.hex_to_binary(), "1111111010011110000000000100000000000000000000000000000000000");
}