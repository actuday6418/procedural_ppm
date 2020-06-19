use std::env;
use std::fs;
use std::io::Write;

const WIDTH: i32 = 1080;
const HEIGHT: i32 = 1080;

fn parse(args: &[String]) -> (f64, f64, f64) {
    let red_value = args[1].parse().expect("Parsing Error");
    let green_value = args[2].parse().expect("Parsing Error");
    let blue_value = args[3].parse().expect("Parsing Error");
    (red_value, green_value, blue_value)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //Check if we have all the command line args
    if args.len() == 4 || args.len() == 5 {
        let (r, g, b) = parse(&args);
        println!("red:{}, green:{}, blue:{}", r, g, b);
        let path_to_output = if args.len() != 5 {
            "output.ppm"
        } else {
            &*args[4]
        };
        let mut file = fs::File::create(path_to_output).expect("Could not open file");
        let mut content = String::from("P3\n1080 1080\n255\n");
        for i in 1..HEIGHT {
            for j in 1..WIDTH {
                let var_a: i32 = (i as f64 / 1080.0 * (255.0 - r)) as i32;
                let var_b: i32= (j as f64 / 1080.0 * (255.0 - g)) as i32;
                let var_c: i32= ((i + j) as f64 / 2160.0 * (255.0 - b)) as i32;
                content.push_str(&*format!("{} {} {} ", var_a, var_b, var_c));
            }
            content.push_str("\n");
        }
        file.write_all(content.as_bytes()).expect("Write Failed");
    }
}
