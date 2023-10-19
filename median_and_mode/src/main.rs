use std::collections::HashMap;

fn main() {

    // initialize vector

    //let mut v = vec![1, 2, 3, 4, 5, 6]
    //let mut v = vec![2, -100, 1000, 21, 123, 2, 2, 3, 3, 3, 4, 5]
    //let mut v = vec![1, 1, 1, 1, 1];
    let mut v = vec![1, 5, 2, 70];



    // median
    
    let mut median: f64;

        // sort
    v.sort();

        // get midpoint(s) and calculate median
    let length = v.len();
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
    let mut int_map = HashMap::new();

        // iterate over vector like in listing 8-25 (official rust book)
    for num in v {
        let count = int_map.entry(num).or_insert(0);
        *count += 1;
    }

        // create modes vec
    let mut modes: Vec<i32> = Vec::new();

        // iterate over value pairs in hashmap, store max value(s) in the vec
    let mut max: i32 = 1;
    for (uniq_num, value) in int_map {
        if value == max { modes.push(uniq_num); };
        if value > max {
            max = value;
            modes.clear();
            modes.push(uniq_num);
        };
    }

        // print modes
    println!("mode(s): {:?}", modes);
}
