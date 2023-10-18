fn main() {

    // initialize vector

    //let mut v = vec![1, 2, 3, 4, 5, 6]
    //let mut v = vec![2, -100, 1000, 21, 123, 2, 2, 3, 3, 3, 4, 5]
    //let mut v = vec![1, 1, 1, 1, 1];
    let mut v = vec![1, 5, 2, 70];

    // median
    
    let mut median: f64;
    let length = v.len();
        // sort
    v.sort();
        // get midpoint(s) and calculate median
    if length % 2 == 0 {
        median = (v[length / 2] + v[(length / 2) - 1]) as f64;
        median = median / 2.0;
    } else {
        median = v[length / 2] as f64;
    }
        // print median
    println!("median is {}", median);



    // mode
        // init hashmap
        // iterate over vector like in listing 8-25 (official rust book)
        // create modes vec
        // iterate over value pairs in hashmap, store max value(s) in the vec
        // print modes
}
