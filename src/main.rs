// use std::arch::aarch64::float32x2_t;
 use std::io; //io is a module
// use rand::Rng; //rng is a trait
// use std::cmp::Ordering;
// use std::mem;

fn main() {

    let mut b: f32 = 10.0;
    let mut m: f32 = 41.0;
    const epochs: i32 = 10000;
    //const trainStep: f32 = 0.01;
    //y = mx+b (minimize this loss)

    const size: usize = 9;
    const sizeF: f32 = size as f32;

    let _data: [(f32, f32); size] = [(1.0,1.0),(2.0,2.0),(3.0,3.0),(4.0,4.0),(5.0,5.0),(6.0,6.0),(7.0,7.0),(8.0,8.0),(9.0,9.0)];
    let input: [f32; size] = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];

   // let mut loss: Vec<f32> = vec![];

    for u in 0..epochs{
    let mut output: Vec<f32> = vec![];
    //output matrix
    for i in input{
        let out = i*m + b;
        output.push(out);
        //println!("{out}");
    }

    //println!("done");

    //total loss (1/size)E(y - (mx+b))^2
    let mut tloss = 0.0;
    for j in 0..size{
        tloss += (_data[j].1 - output[j])*(_data[j].1 - output[j]);
        //println!("{tloss}");
    }
    tloss = tloss / sizeF;
    //println!("tloss {tloss}");

    //loss function of m
    //d/dm = (2/size)E(y - (mx + b))(-x)
    let mut mloss: f32 = 0.0;
    for g in 0..size{
        mloss += (2.0 / sizeF)*(_data[g].1 - output[g])*(-1.0 * input[g]);
    }
    //println!("mloss -- {mloss}");
    
    m -= 0.01*mloss;

    //loss function of b
    //d/db = (2/size)(y - (mx + b))
    let mut bloss: f32 = 0.0;
    for h in 0..size{
        bloss += (2.0 / sizeF)*(_data[h].1 - output[h])*(-1.0);
    }
    //println!("bloss -- {bloss}");

    b -= 0.01*bloss;
    

    
}

    println!("Slope -- {m}");
    println!("Y-intercept -- {b}");



}
