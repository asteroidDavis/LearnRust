use std::io;
use float_cmp::approx_eq;

fn main() {
	println!("Input should be formatted as fff.ffU where f is the digits of a number and U is the character representing units");
    println!("Enter input temperature with units in {{C, F, K}}: ");
	let mut input_temperature = String::new();
	io::stdin()
		.read_line(&mut input_temperature)
		.expect("Failed to read line.");
	let len = input_temperature.trim().len();
	let input_units: char = match input_temperature.trim()[len-1..len].parse() {
		Ok(unit) => unit,
		Err(_) => panic!("Unsupported input format. Follow instructions next time."),
	};
	let input_temperature: f64 = match input_temperature.trim()[..len-1].parse() {
		Ok(temp) => temp,
		Err(_) => panic!("Unsupported input format. Follow instructions next time."),
	};
	println!("What units should the output contain {{C, F, K}}: ");
	let mut output_units = String::new();
	io::stdin()
		.read_line(&mut output_units)
		.expect("Failed to read line.");
	let output_units: char = match output_units.trim().parse() {
		Ok(unit) => unit,
		Err(_) => panic!("Unsupported input format. Follow instructions next time."),
	};
	println!("{} {} is the same as {} {}", input_temperature, input_units, convert(input_units, output_units, input_temperature), output_units);
}

fn convert(input_unit: char, output_unit: char, temperature: f64) -> f64 {
	let output_temperature: f64 = match (input_unit, output_unit) {
		('F', 'F') => temperature,
		('C', 'C') => temperature,
		('F', 'C') => convert_fahrenheit_to_celsius(temperature),
		('C', 'F') => convert_celsius_to_fahrenheit(temperature),
		(_, _) => panic!("Conversion from {} to {} not supported!", input_unit.to_string(), output_unit.to_string())
	};
	return output_temperature
}

// TODO: these should instead store a formula once and invert it for the opposite conversion. Unfortunately I don't know how to do that without storing algebraic expressions
fn convert_celsius_to_fahrenheit(temperature: f64) -> f64 {
	return temperature * (9.0/5.0) + 32.0
}
fn convert_fahrenheit_to_celsius(temperature: f64) -> f64 {
	return (temperature - 32.0 )*(5.0/9.0)
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn boiling(){
        assert!(approx_eq!(f64, convert('C', 'F', 100.0), 212.0, ulps=10));
    }

    #[test]
    fn freezing() {
        assert!(approx_eq!(f64, convert('F', 'C', 32.0), 0.0, ulps=10));
    }

    #[test]
    #[should_panic]
    fn kelvin_unsupported() {
        assert!(approx_eq!(f64, convert('K', 'C', 273.0), 0.0, ulps=10));
    }
}
