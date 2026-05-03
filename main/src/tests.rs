use super::*;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[test]
fn default() {
    let args = Args::try_parse_from(["passgen"]).unwrap();

    assert!(!args.show);
    assert!(args.clipboard);
    assert_eq!(args.length, 20);
    assert_eq!(args.amount, 1);
    assert!(args.lowercase);
    assert!(args.uppercase);
    assert!(args.numbers);
    assert!(args.symbols);
}

#[test]
fn flag() {
    let args = Args::try_parse_from(["passgen", "--show"]).unwrap();

    assert!(args.show);
}

#[test]
fn no_clipboard_flag_disables_clipboard() {
    let args = Args::try_parse_from(["passgen", "--no-clipboard", "--show"]).unwrap();

    assert!(!args.clipboard);
    assert!(args.show);
}

#[test]
fn length() {
    let args = Args::try_parse_from(["passgen", "--length", "64", "--amount", "5"]).unwrap();

    assert_eq!(args.length, 64);
    assert_eq!(args.amount, 5);
}

#[test]
fn charset() {
    let args = Args::try_parse_from(["passgen"]).unwrap();
    let charset = build_charset(&args);

    assert!(charset.contains(&b'a'));
    assert!(charset.contains(&b'Z'));
    assert!(charset.contains(&b'7'));
    assert!(charset.contains(&b'!'));
}

#[test]
fn charset_disabled() {
    let args = Args::try_parse_from(["passgen", "--no-numbers", "--no-symbols"]).unwrap();

    let charset = build_charset(&args);

    assert!(charset.contains(&b'a'));
    assert!(charset.contains(&b'Z'));
    assert!(!charset.contains(&b'7'));
    assert!(!charset.contains(&b'!'));
}

#[test]
fn bad_charset() {
    let args = Args::try_parse_from([
        "passgen",
        "--no-lowercase",
        "--no-uppercase",
        "--no-numbers",
        "--no-symbols",
    ])
    .unwrap();

    let charset = build_charset(&args);
    let result = validate_args(&args, &charset);

    assert!(result.is_err());
}

#[test]
fn no_output_issue() {
    let args = Args::try_parse_from(["passgen", "--no-clipboard"]).unwrap();
    let charset = build_charset(&args);
    let result = validate_args(&args, &charset);
    assert!(result.is_err());
}

#[test]
fn length_clamp() {
    assert_eq!(clamp_length(0), 2);
    assert_eq!(clamp_length(1), 2);
    assert_eq!(clamp_length(16), 16);
    assert_eq!(clamp_length(999_999), MAX_LENGTH);

    assert_eq!(clamp_amount(0), 1);
    assert_eq!(clamp_amount(1), 1);
    assert_eq!(clamp_amount(10), 10);
    assert_eq!(clamp_amount(999_999), MAX_AMOUNT);
}

#[test]
fn correctness() {
    let args = Args::try_parse_from([
        "passgen",
        "--no-uppercase",
        "--no-numbers",
        "--no-symbols",
        "--show",
    ])
    .unwrap();
    let charset = build_charset(&args);
    let mut rng = StdRng::seed_from_u64(123);
    let password = generate_password(32, &charset, &mut rng);

    assert_eq!(password.len(), 32);
    for byte in password.as_bytes() {
        assert!(LOWERCASE.contains(byte));
    }
}
