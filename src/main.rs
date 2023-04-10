use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn print_help(){
    println!("Usage: hexdump [-b -c -bc -h] [FILENAME]");
    println!("\t-b\tBinary mode");
    println!("\t-c\tColor mode");
    println!("\t-h\tHelp");
    println!("\tFILENAME\tFile to be hexdumped");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut binary_mode = false;
    let mut color_mode = false;
    let mut quiet = false;
    let mut filename = "";
    for arg in args.iter() {
        if arg == "-b" {
            binary_mode = true;
        } else if arg == "-c" {
            color_mode = true;
        } else if arg == "-q" {
            quiet = true;
        } else if arg == "-h" {
            print_help();
            return;
        } else {
            filename = arg;
        }
    }

    if !std::path::Path::new(filename).exists() {
        println!("File not found: {}", filename);
        return;
    }
    let mut error_message = "Failed to open file:".to_string();
    error_message.push_str(filename);
    let file = File::open(filename).expect(error_message.as_str());
    let mut reader = BufReader::new(file);

    let mut buf = [0; 16];
    let mut address = 0;
    let mut printed_star = false;
    let mut c;
    if quiet {
        println!("HEXDUMP: {}", filename);
        if !binary_mode {
            println!("ADDRESS             DATA                     DATA                ASCII");
        }
    }
    loop {
        error_message = "Failed to read from file:".to_string();
        error_message.push_str(filename);
        let n = reader.read(&mut buf).expect(error_message.as_str());
        if n == 0 {
            break;
        }

        let all_zero = buf.iter().all(|&val| val == 0); 
        if all_zero {
            if !printed_star {
                println!("*");
                address += 16;
                printed_star = true;
            } else {
                address += 16;
            }

            continue;
        } else {
            printed_star = false;
        }
        if !binary_mode {
            if color_mode {
                print!("\x1B[38;5;51m{:08x}\x1B[0m  ", address);
            } else {
                print!("{:08x}  ", address);
            }
        }

        for i in 0..16 {
            if i < n {
                let val = buf[i];
                if binary_mode {
                    if color_mode {
                        if val < 128 && val != 0 {
                            print!("{:08b} ", val);
                        } else {
                            print!("\x1B[38;5;240m{:08b}\x1B[0m ", val);
                        }
                    } else {
                        print!("{:08b} ", val);
                    }
                } else if color_mode {
                    c = val as char;
                    if val == 0x00 {
                        print!("\x1B[38;5;240m{:02x}\x1B[0m ", val);
                    } else if c.is_ascii_alphanumeric() {
                        print!("\x1B[32m{:02x}\x1B[0m ", val);
                    } else {
                        print!("{:02x} ", val);
                    }
                } else {
                    print!("{:02x} ", val);
                }
            } else {
                print!("   ");
            }
            if i == 7 {
                print!(" ");
            }
        }        

        if !binary_mode {
            print!(" ");

            for i in 0..n {
                let c = buf[i] as char;
                if c.is_ascii_alphanumeric() {
                    print!("\x1B[32m{}\x1B[0m", c);
                } else {
                    print!("\x1B[38;5;240m.\x1B[0m");
                }
            }
        }

        println!();
        address += 16;
    }
}
