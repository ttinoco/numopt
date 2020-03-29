use optrs::matrix::CooMat;

fn main() {

    // 6 2 1 0 0
    // 3 1 0 7 0
    // 4 6 0 0 1

    let a = CooMat::new(
        (3, 5),
        vec![0,2,0,0,1,2,1,1,2,0,2],
        vec![0,1,2,0,0,4,1,3,0,1,4],
        vec![5.,6.,1.,1.,3.,-2.,1.,7.,4.,2.,3.],
    );

    println!("a = {:?}", a);

    let mut b = a.to_csr();

    println!("b = {:?}", b);

    b.sum_duplicates();

    println!("b = {:?}", b);
}