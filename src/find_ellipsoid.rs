pub fn find_ell (TOLERANCE: f32, vec: Vec<f32>, singular_value: f32) -> Vec<usize>  {

    let mut index = Vec::new();
    
    for (count, v) in vec.iter().enumerate() {
         if v <= &(singular_value + TOLERANCE) && v >= &(singular_value - TOLERANCE) {
              index.push(count);
               println!("COUNT: {:?}, V: {:?}", count, v);
          }
    }
    index
}
