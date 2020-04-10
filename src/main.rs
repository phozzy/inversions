fn sort_and_count(unsorted: Vec<isize>) -> (Vec<isize>, usize) {
    let length = unsorted.len();
    if length == 1 {
        (unsorted, 0)
    } else {
        let (first_unsorted, second_unsorted) = unsorted.split_at(length / 2);
        let (first_sorted, left_inversions) = sort_and_count(first_unsorted.to_vec());
        let (second_sorted, right_inversions) = sort_and_count(second_unsorted.to_vec());
        let (sorted, split_inversions) = count_split_inv(first_sorted, second_sorted);
        let total_inversions = left_inversions + right_inversions + split_inversions;
        (sorted, total_inversions)
    }
}
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
    let unsorted = vec![1, 3, 5, 2, 4, 6];
    let (_sorted, inversions) = sort_and_count(unsorted);
    println!("number of inversions is {}", inversions);
}
