pub fn main() {
    assert_eq!(matches(111111), true);
    assert_eq!(matches(223450), false);
    assert_eq!(matches(123789), false);

    let start: i32 = 234208;
    let end: i32 = 765869;

    // Exercise 1
    let mut total: i32 = 0;
    for i in start..end {
        if matches(i) {
            total += 1;
        }
    }
    println!("Solution exercise 1: {}", total);


    assert_eq!(matches_extra(112233), true);
    assert_eq!(matches_extra(123444), false);
    
    assert_eq!(matches_extra(223333), true);

    // Exercise 2
    let mut total: i32 = 0;
    for i in start..end {
        if matches_extra(i) {
            total += 1;
        }
    }
    println!("Solution exercise 2: {}", total);
}

fn matches(mut num: i32) -> bool {
    let mut cur: i32 = 10;
    let mut repeat: bool = false;

    for _ in 0..6 {
        
        if (num % 10) > cur {
            return false;
        } else if (num % 10) == cur {
            repeat = true;
        }

        cur = num % 10;
        num /= 10;
    }

    return repeat;
}

fn matches_extra(mut num: i32) -> bool {
    let mut cur: i32 = 10;
    let mut repeat: bool = false;
    let mut repeat_len: i8 = 0;

    for _ in 0..6 {
        if (num % 10) > cur {
            return false;
        } else if (num % 10) == cur {
            repeat_len += 1;
        } else {
            if repeat_len == 2 {
                repeat = true;
            }
            repeat_len = 1;
            cur = num % 10;
        }

        num /= 10;
    }

    return repeat || repeat_len == 2;
}