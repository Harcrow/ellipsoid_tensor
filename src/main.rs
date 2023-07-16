/* ultimate goal:

Well here's something that would be cool
(but a not small project), digest some tables of retrieved objects
(with polarizability values) and do a best fit of an ellipsoid model
to it and spit out the estimated permeability and semi axes.

test

SVD(conj_transpose(alpha) x alpha) <-- apparently what we do?

1. Measure
2. retrieve full polarizability tensor
3. derive rotation matrix
4. run a regression model plus threshold -- this extracts trends from the LUT to provide a score to each object in a scan
*/

use std::env;
use std::fs::File;
use std::io::{Error, Write};

mod ellipsoid_polarizability;
mod tensor_parse;
mod find_ellipsoid;

use ellipsoid_polarizability as ell;
use find_ellipsoid as find_ell;
fn main() -> Result<(), Error> {
    let ELLIPSE_ITER = 275;
    let ELLIPSE_RESOLUTION = 0.05;
    let TOLERANCE = 3.0;

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut ell_vec = vec![0.0; ELLIPSE_ITER];
    let perm_vec = vec![0, 50, 1000, 10000, 50000, 200000];

    let mut tensor_a1 = Vec::new();
    let mut tensor_a2 = Vec::new();
    let mut tensor_a3 = Vec::new();

    let mut i_index = Vec::new();
    let mut j_index = Vec::new();
    let mut k_index = Vec::new();
    let mut l_index = Vec::new();
    let mut volume = Vec::new();

    let mut index = 0.3;
    let mut count = 0;

    //let mut file = File::create("ellipse.csv")?;

    //Generates the vector that will be used to generate 1000^4 ellipse tensor values
    while ell_vec[ELLIPSE_ITER - 1] == 0.0 {
        ell_vec[count] = index + ELLIPSE_RESOLUTION;
        index += ELLIPSE_RESOLUTION;
        count += 1;
    }
    //   println!("ell_vec {:?}", ell_vec);
    //    let _epsilon = 0.001;

    let vec = tensor_parse::get_file(path.to_string());
    let mat = tensor_parse::complex_matrix(vec);

    let svd = mat.svd(true, true);

    for i in &ell_vec {
        for j in &ell_vec {
            for k in &ell_vec {
                for l in &perm_vec {
                    let ellipse = ell::ellipse(*i, *j, *k, *l as f32);
                    tensor_a1.push(ellipse.0);
                    tensor_a2.push(ellipse.1);
                    tensor_a3.push(ellipse.2);
                    volume.push(ellipse.3);
                    i_index.push(i);
                    j_index.push(j);
                    k_index.push(k);
                    l_index.push(l);

                    //              write!(file, "{:?}, {:?}, {:?},  ", ellipse.0, ellipse.1, ellipse.2)?;
                    //println!("Pusing tensor updates");
                }
            }
        }
    }
    /*                    let ellipse = ell::ellipse(7.0, 3.0, 1.5, 50.0);
                        tensor_a1.push(ellipse.0);
                        tensor_a2.push(ellipse.1);
                        tensor_a3.push(ellipse.2);
                        volume.push(ellipse.3);
                        i_index.push(7.0);
                        j_index.push(3.0);
                        k_index.push(1.5);
                        l_index.push(50.0);
    */

    //    println!("Right-Singular Vector {:.3?}", svd.v_t);
    //    println!("Left-Singular Vector {:.3?}", svd.u);

    let singular_values_1 = svd.singular_values[(0, 0)];
    let singular_values_2 = svd.singular_values[(1, 0)];
    let singular_values_3 = svd.singular_values[(2, 0)];

    let beta_1 = singular_values_1 / (singular_values_1 + singular_values_2 + singular_values_3);

    //    let singular_values_1 = ;
    //    let singular_values_2 = ;
    //    let singular_values_3 = ;

    Ok(())
}
