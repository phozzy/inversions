fn count_split_inv(first_sorted: Vec<isize>, second_sorted: Vec<isize>) -> (Vec<isize>, usize) {
    let mut output: Vec<isize> = Vec::new();
    let mut first = first_sorted.iter();
    let mut second = second_sorted.iter();
    let mut f;
    let mut s;
    let mut fi = true;
    let mut si = true;
    let mut inversions: usize = 0;
    let mut merges: usize = 0;
    match first.next() {
        Some(val) => f = val,
        None => panic!("Empy array!"),
    };
    match second.next() {
        Some(val) => s = val,
        None => panic!("Empy array!"),
    };
    loop {
        if f < s && fi || !si {
            output.push(*f);
            inversions += merges;
            match first.next() {
                Some(val) => {
                    f = val;
                },
                None => {
                    fi = false;
                },
            };
        } else {
            output.push(*s);
            merges += 1;
            match second.next() {
                Some(val) => {
                    s = val;
                },
                None => {
                    si = false;
                },
            };
        };
        if !(fi || si) {
            break (output, inversions);
        }
    }
}
fn main() {
    println!("Hello, world!");
}
