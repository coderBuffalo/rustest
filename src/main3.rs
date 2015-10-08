use std::num::ParseIntError;
use std::convert;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct AppError(String);
macro_rules! to_apperr {
    ($e:ident, $m:expr) => {
        impl convert::From<$e> for AppError {
            fn from(error: $e) -> AppError {
                AppError(format!($m, error.description().to_string()))
            }
        }
    }
}


impl Error for AppError {
    fn description(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// impl convert::From<num::ParseIntError> for AppError {
//     fn from(error: num::ParseIntError) -> AppError {
//         AppError(format!("Unable to parse values: {}", error.description().to_string()))
//     }
// }
to_apperr!(ParseIntError, "Unable to parse values: {}");

fn main() {
    match read_values() {
        Ok(values) => println!("{}", avg(&values)),
        Err(e) => println!("{}", e.description()),
    }
}

fn avg(values: &[u32]) -> u32 {
    let (sum, count) = values.iter().fold((0,0), |(s,c), n| (s+n, c+1));

    (sum as f32 / count as f32) as u32
}

fn read_values() -> Result<Vec<u32>, AppError> {
    let mut values  = Vec::new();
    for arg in std::env::args().skip(1) {
        values.push(try!(arg.parse()));

        // values.push(match arg.parse::<u32>() {
        //     Ok(n) => n,
        //     Err(e) => return AppError(e.description().to_string()),
        // });
    }

    Ok(values)
}