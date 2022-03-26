#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let (n, k) = parse_line!(usize, i32);
    let a = parse_vec!(i32);
    let b = parse_vec!(i32);
    let mut dp = vec![false; n];
    let mut ep = vec![false; n];
    dp[0] = true;
    ep[0] = true;
    for i in 1..n {
        if dp[i - 1] {
            if (a[i - 1] - a[i]).abs() <= k {
                dp[i] = true;
            }
            if (a[i - 1] - b[i]).abs() <= k {
                ep[i] = true;
            }
        }
        if ep[i - 1] {
            if (b[i - 1] - a[i]).abs() <= k {
                dp[i] = true;
            }
            if (b[i - 1] - b[i]).abs() <= k {
                ep[i] = true;
            }
        }
    }
    if dp[n - 1] || ep[n - 1] {
        println!("Yes")
    } else {
        println!("No")
    }
}
