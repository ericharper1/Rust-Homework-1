#[test]
fn test_modexp() {
    assert_eq!(modexp(2, 20, 17), 16);
    assert_eq!(modexp(0, 0, 0), 0);
    assert_eq!(modexp(2, 4, 1), 0);
    assert_eq!(modexp(2, 0, 1), 0);
    assert_eq!(modexp(2, 31, 3), 2);
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }

    let mut z: u64 = modexp(x, y / 2, m);
    z = (z * z) % m;

    if (y % 2) == 1 {
        z = (z * x) % m;
    }

    z
}

fn error() -> ! {
    eprintln!("modexp: usage: modexp <x>, <y>, <m>");
    std::process::exit(1);
}

fn range_error() -> ! {
    eprintln!("The exponential value provided is out of range");
    std::process::exit(1);
}

fn main() {
    // this is used for skipping prog name in cmd-line args
    let mut argv: [u64; 3] = [0; 3];
    let mut i = 0;
    for arg in std::env::args().skip(1) {
        argv[i] = arg.parse().unwrap_or_else(|_| error());
        i += 1;
    }

    if argv[2] == 0 {
        error();
    }

    if argv[0].pow(argv[1] as u32) > u64::from(u32::max_value()) {
        range_error();
    }

    let res = modexp(argv[0], argv[1], argv[2]);

    println!(
        "The result of ({}**{}) mod {} is: {}",
        argv[0], argv[1], argv[2], res
    );
}
