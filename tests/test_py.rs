use crate::workdir::Workdir;

#[test]
fn py_map() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("inc")
        .arg("int(number) + 1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_builtins() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("sum")
        .arg("sum([int(number), 2, 23])")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "sum"],
        svec!["a", "13", "38"],
        svec!["b", "24", "49"],
        svec!["c", "72", "97"],
        svec!["d", "7", "32"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_math() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("div")
        .arg("math.floor(int(number) / 2)")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "div"],
        svec!["a", "13", "6"],
        svec!["b", "24", "12"],
        svec!["c", "72", "36"],
        svec!["d", "7", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_userhelper() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "6"],
            svec!["d", "7"],
            svec!["e", "fib of 8"],
        ],
    );

    wrk.create_from_string(
        "user_helper.py",
        r#"
def fibonacci(input):
    try:
      float(input)
    except ValueError:
      return "incorrect input - not a number"
    sinput = str(input)
    if not float(sinput).is_integer():
        return "incorrect input - not a whole number"

    n = int(sinput)
    if n < 0:
        return "incorrect input - negative number"
    elif n == 0:
        return 0
    elif n == 1 or n == 2:
        return 1
    else:
        return fibonacci(n-1) + fibonacci(n-2)


def celsius_to_fahrenheit(celsius):
    try:
        float(celsius)
    except ValueError:
        return "incorrect input - not a float"
    fahrenheit = (float(celsius) * 9/5) + 32
    return f'{fahrenheit:.1f}'
"#,
    );

    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("--helper")
        .arg("user_helper.py")
        .arg("fib")
        .arg("qsv_uh.fibonacci(number)")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "fib"],
        svec!["a", "1", "1"],
        svec!["b", "2", "1"],
        svec!["c", "6", "8"],
        svec!["d", "7", "13"],
        svec!["e", "fib of 8", "incorrect input - not a number"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("--helper")
        .arg("user_helper.py")
        .arg("fahrenheit")
        .arg("qsv_uh.celsius_to_fahrenheit(number)")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "fahrenheit"],
        svec!["a", "1", "33.8"],
        svec!["b", "2", "35.6"],
        svec!["c", "6", "42.8"],
        svec!["d", "7", "44.6"],
        svec!["e", "fib of 8", "incorrect input - not a float"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_row_positional() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("inc")
        .arg("int(row[1]) + 1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_row_by_key() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("inc")
        .arg("int(row['number']) + 1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_row_by_attr() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("inc")
        .arg("int(row.number) + 1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_no_headers() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("int(row[1]) + 1")
        .arg("--no-headers")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_map_boolean() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("test")
        .arg("int(number) > 14")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "test"],
        svec!["a", "13", "False"],
        svec!["b", "24", "True"],
        svec!["c", "72", "True"],
        svec!["d", "7", "False"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_filter() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("filter").arg("int(number) > 14").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["b", "24"],
        svec!["c", "72"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_format() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["qty", "fruit", "unitcost"],
            svec!["20.5", "mangoes", "5"],
            svec!["10", "bananas", "20"],
            svec!["3", "strawberries", "3.50"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("formatted")
        .arg("f'{qty} {fruit} cost ${(float(unitcost) * float(qty)):.2f}'")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["qty", "fruit", "unitcost", "formatted"],
        svec!["20.5", "mangoes", "5", "20.5 mangoes cost $102.50"],
        svec!["10", "bananas", "20", "10 bananas cost $200.00"],
        svec!["3", "strawberries", "3.50", "3 strawberries cost $10.50"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn py_format_header_with_invalid_chars() {
    let wrk = Workdir::new("py");
    wrk.create(
        "data.csv",
        vec![
            svec!["qty-fruit/day", "1fruit", "unit cost usd"],
            svec!["20.5", "mangoes", "5"],
            svec!["10", "bananas", "20"],
            svec!["3", "strawberries", "3.50"],
        ],
    );
    let mut cmd = wrk.command("py");
    cmd.arg("map")
        .arg("formatted")
        .arg(
            "f'{qty_fruit_day} {_fruit} cost ${(float(unit_cost_usd) * float(qty_fruit_day)):.2f}'",
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["qty-fruit/day", "1fruit", "unit cost usd", "formatted"],
        svec!["20.5", "mangoes", "5", "20.5 mangoes cost $102.50"],
        svec!["10", "bananas", "20", "10 bananas cost $200.00"],
        svec!["3", "strawberries", "3.50", "3 strawberries cost $10.50"],
    ];
    assert_eq!(got, expected);
}
