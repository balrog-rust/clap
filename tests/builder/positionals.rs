use clap::{arg, App, Arg, ErrorKind};

#[test]
fn only_pos_follow() {
    let r = App::new("onlypos")
        .args(&[arg!(f: -f [flag] "some opt"), arg!([arg] "some arg")])
        .try_get_matches_from(vec!["", "--", "-f"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert!(!m.is_present("f"));
    assert_eq!(m.value_of("arg").unwrap(), "-f");
}

#[test]
fn issue_946() {
    let r = App::new("compiletest")
        .setting(clap::AppSettings::AllowHyphenValues)
        .arg(arg!(--exact    "filters match exactly"))
        .arg(
            clap::Arg::new("filter")
                .index(1)
                .takes_value(true)
                .help("filters to apply to output"),
        )
        .try_get_matches_from(vec!["compiletest", "--exact"]);
    assert!(r.is_ok(), "{:#?}", r);
    let matches = r.unwrap();

    assert!(matches.is_present("exact"));
    assert!(matches.value_of("filter").is_none());
}

#[test]
fn positional() {
    let r = App::new("positional")
        .args(&[arg!(-f --flag "some flag"), Arg::new("positional").index(1)])
        .try_get_matches_from(vec!["", "-f", "test"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");

    let m = App::new("positional")
        .args(&[arg!(-f --flag "some flag"), Arg::new("positional").index(1)])
        .try_get_matches_from(vec!["", "test", "--flag"])
        .unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");
}

#[test]
fn lots_o_vals() {
    let r = App::new("opts")
        .arg(arg!(<opt>... "some pos"))
        .try_get_matches_from(vec![
            "", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some",
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("opt"));
    assert_eq!(m.values_of("opt").unwrap().count(), 297); // i.e. more than u8
}

#[test]
fn positional_multiple() {
    let r = App::new("positional_multiple")
        .args(&[
            arg!(-f --flag "some flag"),
            Arg::new("positional")
                .index(1)
                .takes_value(true)
                .multiple_values(true),
        ])
        .try_get_matches_from(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(
        &*m.values_of("positional").unwrap().collect::<Vec<_>>(),
        &["test1", "test2", "test3"]
    );
}

#[test]
fn positional_multiple_3() {
    let r = App::new("positional_multiple")
        .args(&[
            arg!(-f  --flag "some flag"),
            Arg::new("positional")
                .index(1)
                .takes_value(true)
                .multiple_values(true),
        ])
        .try_get_matches_from(vec!["", "test1", "test2", "test3", "--flag"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(
        &*m.values_of("positional").unwrap().collect::<Vec<_>>(),
        &["test1", "test2", "test3"]
    );
}

#[test]
fn positional_multiple_2() {
    let result = App::new("positional_multiple")
        .args(&[arg!(-f --flag "some flag"), Arg::new("positional").index(1)])
        .try_get_matches_from(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::UnknownArgument);
}

#[test]
fn positional_possible_values() {
    let r = App::new("positional_possible_values")
        .args(&[
            arg!(-f --flag "some flag"),
            Arg::new("positional").index(1).possible_value("test123"),
        ])
        .try_get_matches_from(vec!["", "-f", "test123"]);
    assert!(r.is_ok(), "{:#?}", r);
    let m = r.unwrap();
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(
        &*m.values_of("positional").unwrap().collect::<Vec<_>>(),
        &["test123"]
    );
}

#[test]
fn create_positional() {
    let _ = App::new("test")
        .arg(Arg::new("test").index(1).help("testing testing"))
        .try_get_matches_from(vec![""])
        .unwrap();
}

#[test]
fn positional_hyphen_does_not_panic() {
    let _ = App::new("test")
        .arg(Arg::new("dummy"))
        .try_get_matches_from(vec!["test", "-"])
        .unwrap();
}

#[test]
fn single_positional_usage_string() {
    let mut app = App::new("test").arg(arg!([FILE] "some file"));
    assert_eq!(app.render_usage(), "USAGE:\n    test [FILE]");
}

#[test]
fn single_positional_multiple_usage_string() {
    let mut app = App::new("test").arg(arg!([FILE]... "some file"));
    assert_eq!(app.render_usage(), "USAGE:\n    test [FILE]...");
}

#[test]
fn multiple_positional_usage_string() {
    let mut app = App::new("test")
        .arg(arg!([FILE] "some file"))
        .arg(arg!([FILES]... "some file"));
    assert_eq!(app.render_usage(), "USAGE:\n    test [ARGS]");
}

#[test]
fn multiple_positional_one_required_usage_string() {
    let mut app = App::new("test")
        .arg(arg!(<FILE> "some file"))
        .arg(arg!([FILES]... "some file"));
    assert_eq!(app.render_usage(), "USAGE:\n    test <FILE> [FILES]...");
}

#[test]
fn single_positional_required_usage_string() {
    let mut app = App::new("test").arg(arg!(<FILE> "some file"));
    assert_eq!(app.render_usage(), "USAGE:\n    test <FILE>");
}

// This tests a programmer error and will only succeed with debug_assertions
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Found non-required positional argument \
with a lower index than a required positional argument"]
fn missing_required() {
    let _ = App::new("test")
        .arg(arg!([FILE1] "some file"))
        .arg(arg!(<FILE2> "some file"))
        .try_get_matches_from(vec![""]);
}

#[test]
fn missing_required_2() {
    let r = App::new("test")
        .arg(arg!(<FILE1> "some file"))
        .arg(arg!(<FILE2> "some file"))
        .try_get_matches_from(vec!["test", "file"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn last_positional() {
    let r = App::new("test")
        .arg(arg!(<TARGET> "some target"))
        .arg(arg!([CORPUS] "some corpus"))
        .arg(arg!([ARGS]... "some file").last(true))
        .try_get_matches_from(vec!["test", "tgt", "--", "arg"]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert_eq!(m.values_of("ARGS").unwrap().collect::<Vec<_>>(), &["arg"]);
}

#[test]
fn last_positional_no_double_dash() {
    let r = App::new("test")
        .arg(arg!(<TARGET> "some target"))
        .arg(arg!([CORPUS] "some corpus"))
        .arg(arg!([ARGS]... "some file").last(true))
        .try_get_matches_from(vec!["test", "tgt", "crp", "arg"]);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn last_positional_second_to_last_mult() {
    let r = App::new("test")
        .arg(arg!(<TARGET> "some target"))
        .arg(arg!([CORPUS]... "some corpus"))
        .arg(arg!([ARGS]... "some file").last(true))
        .try_get_matches_from(vec!["test", "tgt", "crp1", "crp2", "--", "arg"]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument 'arg' is a positional argument and can't have short or long name versions"]
fn positional_arg_with_long() {
    use clap::{App, Arg};

    let _ = App::new("test")
        .arg(Arg::new("arg").index(1).long("arg"))
        .try_get_matches();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument 'arg' is a positional argument and can't have short or long name versions"]
fn positional_arg_with_short() {
    use clap::{App, Arg};

    let _ = App::new("test")
        .arg(Arg::new("arg").index(1).short('a'))
        .try_get_matches();
}

#[test]
fn ignore_hyphen_values_on_last() {
    let app = clap::App::new("foo")
        .arg(
            clap::Arg::new("cmd")
                .multiple_values(true)
                .last(true)
                .allow_hyphen_values(true),
        )
        .arg(
            clap::Arg::new("name")
                .long("name")
                .short('n')
                .takes_value(true)
                .required(false),
        );

    let matches = app.try_get_matches_from(["test", "-n", "foo"]).unwrap();
    assert_eq!(matches.value_of("name"), Some("foo"));
}
