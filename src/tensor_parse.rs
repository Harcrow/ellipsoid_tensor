/* module to read in a single tensor and return the complex matrix*/
use std::fs;
//use std::error::Error;
//use std::io::BufRead;
use nalgebra::Complex;
use nalgebra::Matrix3;

pub fn get_file(path: String) -> Vec<f32> {
    //takes the file passed to the function
    let file = fs::read_to_string(path).expect("Could not read file, ding dong");

    //creates a mutable string slice vector and makes an iterator,  that splits on ',' and new lines -- then puts the iterator into a collection
    let mut val: Vec<&str> = file.split(|c| c == ',' || c == '\n').collect();

    //truncates to 18 values (2x9 matrix values, due to real and im)
    val.truncate(18);

    //creates the vector that will store the prased values of float-32
    let mut float_val: Vec<f32> = Vec::new();

    //iterates over the string vector, pushes to the float vector, trims the white space, parses the string slice to another type.  Unwrap provides the embedded T (generic return type) if there is one.
    for i in &val {
        //println!("{:?}", i.trim());
        float_val.push(i.trim().parse::<f32>().unwrap());
    }
    float_val
}

pub fn complex_matrix(float_val: Vec<f32>) -> Matrix3<Complex<f32>> {
    let mat = Matrix3::<Complex<f32>>::new(
        Complex::<f32>::new(float_val[0], float_val[1]),
        Complex::<f32>::new(float_val[2], float_val[3]),
        Complex::<f32>::new(float_val[4], float_val[5]),
        Complex::<f32>::new(float_val[6], float_val[7]),
        Complex::<f32>::new(float_val[8], float_val[9]),
        Complex::<f32>::new(float_val[10], float_val[11]),
        Complex::<f32>::new(float_val[12], float_val[13]),
        Complex::<f32>::new(float_val[14], float_val[15]),
        Complex::<f32>::new(float_val[16], float_val[17]),
    );

    mat
}
