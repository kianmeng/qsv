use crate::workdir::Workdir;

#[test]
fn apply_ops_unknown_operation() {
    let wrk = Workdir::new("unknown_op");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("obfuscate")
        .arg("letter")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(&*got, "Unknown 'obfuscate' operation\n")
}

#[test]
fn apply_ops_upper() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "surname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_upper_rename() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("--rename")
        .arg("uname,usurname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["uname", "usurname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_upper_rename_invalid() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("--rename")
        .arg("uname")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(got, "Invalid arguments.\n");
}

#[test]
fn apply_ops_upper_index_params() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("1,2")
        .arg("--rename")
        .arg("uname,usurname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["uname", "usurname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_dynfmt() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec![
                "qty-fruit/day",
                "1fruit",
                "another col",
                "unit cost usd",
                "and another one"
            ],
            svec!["20.5", "mangoes", "a", "5", "z"],
            svec!["10", "bananas", "b", "20", "y"],
            svec!["3", "strawberries", "c", "3.50", "x"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("dynfmt")
        .arg("--formatstr")
        .arg("{qty_fruit_day} helpings of {1fruit} is good for you, even if it costs ${unit_cost_usd} each. {1fruit}, all {qty_fruit_day} - is just worth it!")
        .arg("--new-column")
        .arg("saying")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "qty-fruit/day",
            "1fruit",
            "another col",
            "unit cost usd",
            "and another one",
            "saying"
        ],
        svec![
            "20.5",
            "mangoes",
            "a",
            "5",
            "z",
            "20.5 helpings of mangoes is good for you, even if it costs $5 each. mangoes, all 20.5 - is just worth it!"
        ],
        svec![
            "10",
            "bananas",
            "b",
            "20",
            "y",
            "10 helpings of bananas is good for you, even if it costs $20 each. bananas, all 10 - is just worth it!"
        ],
        svec![
            "3",
            "strawberries",
            "c",
            "3.50",
            "x",
            "3 helpings of strawberries is good for you, even if it costs $3.50 each. strawberries, all 3 - is just worth it!"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_empty_shortcircuit() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec![""],
            svec![""],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations").arg("len").arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["4"],
        svec!["0"],
        svec!["0"],
        svec!["7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_titlecase() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle little star, how I wonder what you are"],
            svec!["a simple title to capitalize: an example"],
            svec!["new york city police department - NYPD"],
            svec!["department of human services"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("titlecase")
        .arg("description")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["The Quick Brown Fox Jumped Over the Lazy Dog."],
        svec!["Twinkle, Twinkle Little Star, How I Wonder What You Are"],
        svec!["A Simple Title to Capitalize: An Example"],
        svec!["New York City Police Department - NYPD"],
        svec!["Department of Human Services"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_check() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ you!"],
            svec!["F_u c_K"],
            svec!["fuuuuuuuck"],
            svec!["fluff truck"],
            svec!["fukushima"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor_check")
        .arg("description")
        .arg("--new-column")
        .arg("profanity_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "profanity_flag"],
        svec!["fuck", "true"],
        svec!["FUCK", "true"],
        svec!["fμ¢κ you!", "true"],
        svec!["F_u c_K", "true"],
        svec!["fuuuuuuuck", "true"],
        svec!["fluff truck", "false"],
        svec!["fukushima", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ that shit, faggot!"],
            svec!["F_u c_K that blowjoboobies"],
            svec!["fuuuuuuuck yooooouuuu"],
            svec!["kiss my ass!"],
            svec!["shittitties"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor")
        .arg("description")
        .arg("--new-column")
        .arg("censored_text")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "censored_text"],
        svec!["fuck", "****"],
        svec!["FUCK", "****"],
        svec!["fμ¢κ that shit, faggot!", "**** that ****, ******!"],
        svec!["F_u c_K that blowjoboobies", "*_* *_* that *************"],
        svec!["fuuuuuuuck yooooouuuu", "********** yooooouuuu"],
        svec!["kiss my ass!", "kiss my ***!"],
        svec!["shittitties", "***********"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_check_addlwords() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ you!"],
            svec!["F_u c_K"],
            svec!["fuuuuuuuck"],
            svec!["fluff truck"],
            svec!["fukushima"],
            svec!["shlong dong ding"],
            svec!["long john silver's shlong"],
            svec!["Whoa! I see her cameltoe thru her athleisure!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor_check")
        .arg("description")
        .arg("--comparand")
        .arg("shlong,dong,cameltoe")
        .arg("--new-column")
        .arg("profanity_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "profanity_flag"],
        svec!["fuck", "true"],
        svec!["FUCK", "true"],
        svec!["fμ¢κ you!", "true"],
        svec!["F_u c_K", "true"],
        svec!["fuuuuuuuck", "true"],
        svec!["fluff truck", "false"],
        svec!["fukushima", "false"],
        svec!["shlong dong ding", "true"],
        svec!["long john silver's shlong", "true"],
        svec!["Whoa! I see her cameltoe thru her athleisure!", "true"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_addlwords() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ that shit, faggot!"],
            svec!["F_u c_K that blowjoboobies"],
            svec!["fuuuuuuuck yooooouuuu"],
            svec!["kiss my ass!"],
            svec!["shittitties"],
            svec!["move your shlllooooonng!!!"],
            svec!["that cameltoe is so penistracting!"],
            svec!["ding dong the bitch is dead!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor")
        .arg("description")
        .arg("--comparand")
        .arg("shlong, dong, cameltoe, bitch")
        .arg("--new-column")
        .arg("censored_text")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "censored_text"],
        svec!["fuck", "****"],
        svec!["FUCK", "****"],
        svec!["fμ¢κ that shit, faggot!", "**** that ****, ******!"],
        svec!["F_u c_K that blowjoboobies", "*_* *_* that *************"],
        svec!["fuuuuuuuck yooooouuuu", "********** yooooouuuu"],
        svec!["kiss my ass!", "kiss my ***!"],
        svec!["shittitties", "***********"],
        svec!["move your shlllooooonng!!!", "move your *************!!!"],
        svec![
            "that cameltoe is so penistracting!",
            "that ******** is so *****tracting!"
        ],
        svec![
            "ding dong the bitch is dead!",
            "ding **** the ***** is dead!"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_replace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
            svec!["a simple title to capitalize: an example"],
            svec!["Mr. Brown is not pleased."],
            svec!["this is a brownado car"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("replace")
        .arg("description")
        .arg("--comparand")
        .arg("brown")
        .arg("--replacement")
        .arg("silver")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["THE quick silver fox jumped over the lazy dog."],
        svec!["twinkle, twinkle silverie star, how I wonder what you are"],
        svec!["a simple title to capitalize: an example"],
        svec!["Mr. Brown is not pleased."],
        svec!["this is a silverado car"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_regex_replace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["My SSN is 078-05-1120. Please do not share it."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
            svec!["Somebody from Nigeria called asked for my ssn - 987-65-4320."],
            svec!["Won't fall for that scam!"],
            svec!["Just enter 987-65-4329 when prompted. Also try 987-65-1234 if it doesn't work."],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("description")
        .arg("--comparand")
        .arg("(?:\\d{3}-\\d{2}-\\d{4})")
        .arg("--replacement")
        .arg("SSN")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["My SSN is SSN. Please do not share it."],
        svec!["twinkle, twinkle brownie star, how I wonder what you are"],
        svec!["Somebody from Nigeria called asked for my ssn - SSN."],
        svec!["Won't fall for that scam!"],
        svec!["Just enter SSN when prompted. Also try SSN if it doesn't work."],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_mtrim() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["(This is in parentheses)"],
            svec!["(This is in parentheses, but with a period)."],
            svec!["(Only left paren"],
            svec!["Only right paren)"],
            svec!["(((multiple parens)))"],
            svec!["Embedded (((multiple parens)))"],
            svec![")))reverse parens((("],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("mtrim")
        .arg("description")
        .arg("--comparand")
        .arg("()")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["This is in parentheses"],
        svec!["This is in parentheses, but with a period)."],
        svec!["Only left paren"],
        svec!["Only right paren"],
        svec!["multiple parens"],
        svec!["Embedded (((multiple parens"],
        svec!["reverse parens"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_chain() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("trim,upper,squeeze")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_mixed_case_chain() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("Trim,UPPER,squEeZe")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_no_headers() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["John   "],
            svec!["Mary"],
            svec!["  Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("trim,upper")
        .arg("1")
        .arg("--no-headers")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["JOHN"], svec!["MARY"], svec!["SUE"], svec!["HOPKINS"]];
    assert_eq!(got, expected);
}

#[test]
fn apply_rename() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Mary"],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name")
        .arg("--rename")
        .arg("upper_name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["upper_name"],
        svec!["JOHN"],
        svec!["MARY"],
        svec!["SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_new_column() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Mary"],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name")
        .arg("--new-column")
        .arg("upper_name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "upper_name"],
        svec!["John", "JOHN"],
        svec!["Mary", "MARY"],
        svec!["Sue", "SUE"],
        svec!["Hopkins", "HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_currencytonum() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["money"],
            svec!["$10.00"],
            svec!["$-10.00"],
            svec!["$ 12 500.00"],
            svec!["$5"],
            svec!["0"],
            svec!["5"],
            svec!["$0.25"],
            svec!["$ 10.05"],
            svec!["¥10,000,000.00"],
            svec!["£423.56"],
            svec!["€120.00"],
            svec!["֏99,999.50"],
            svec!["€300 999,55"],
            svec!["This is not money. Leave untouched."],
            svec!["₱1,234,567.89"],
            svec!["₽234,567.89"],
            svec!["₪ 567.89"],
            svec!["₩ 567.89"],
            svec!["₩ 89,123.0"],
            svec!["ƒ 123,456.00"],
            svec!["฿ 789,123"],
            svec!["₫ 456"],
            svec!["123,456.00 $"],
            svec!["USD 10,000"],
            svec!["EUR 1234.50"],
            svec!["JPY 9,999,999.99"],
            svec!["RMB 6543.21"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("currencytonum")
        .arg("money")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["money"],
        svec!["10.00"],
        svec!["-10.00"],
        svec!["12500.00"],
        svec!["5.00"],
        svec!["0"],
        svec!["5.00"],
        svec!["0.25"],
        svec!["10.05"],
        svec!["10000000.00"],
        svec!["423.56"],
        svec!["120.00"],
        svec!["99999.50"],
        svec!["300999.55"],
        svec!["This is not money. Leave untouched."],
        svec!["1234567.89"],
        svec!["234567.89"],
        svec!["567.89"],
        svec!["567.89"],
        svec!["89123.00"],
        svec!["123456.00"],
        svec!["789123.00"],
        svec!["456.00"],
        svec!["123456.00"],
        svec!["10000.00"],
        svec!["1234.50"],
        svec!["9999999.99"],
        svec!["6543.21"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_similarity() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Jonathan"],
            svec!["Edna"],
            svec!["Larry"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("simdln")
        .arg("name")
        .arg("--comparand")
        .arg("Joe")
        .arg("--new-column")
        .arg("name_sim_score")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "name_sim_score"],
        svec!["John", "0.5"],
        svec!["Jonathan", "0.25"],
        svec!["Edna", "0"],
        svec!["Larry", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_similarity_eudex() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Jonathan"],
            svec!["Michelle"],
            svec!["Larry"],
            svec!["Joel"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("lower,eudex")
        .arg("name")
        .arg("--comparand")
        .arg("michael")
        .arg("--new-column")
        .arg("eudex_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "eudex_flag"],
        svec!["John", "false"],
        svec!["Jonathan", "false"],
        svec!["Michelle", "true"],
        svec!["Larry", "false"],
        svec!["Joel", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_similarity_more_eudex() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["Jeuses"],
            svec!["Josephina"],
            svec!["Juan"],
            svec!["Juanita"],
            svec!["Michael"],
            svec!["Jingjing"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("lower,eudex")
        .arg("name")
        .arg("--comparand")
        .arg("Jesus")
        .arg("--new-column")
        .arg("eudex_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "eudex_flag"],
        svec!["Jeuses", "true"],
        svec!["Josephina", "false"],
        svec!["Juan", "true"],
        svec!["Juanita", "true"],
        svec!["Michael", "false"],
        svec!["Jingjing", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_sentiment() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["customer comment"],
            svec!["This is ridiculous! I will never buy from this company again!"],
            svec!["Josephina was awesome! She was very helpful and patient. I wish more customer service folks are like her!"],
            svec!["I can't believe that garbage is still out there. That is so false!"],
            svec!["5 stars! Highly recommended!"],
            svec!["What were they thinking!?!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("sentiment")
        .arg("customer comment")
        .arg("--new-column")
        .arg("sentiment_score")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["customer comment", "sentiment_score"],
        svec!["This is ridiculous! I will never buy from this company again!", "-0.47384376462380107"],
        svec!["Josephina was awesome! She was very helpful and patient. I wish more customer service folks are like her!", "0.9227060290926788"],
        svec!["I can't believe that garbage is still out there. That is so false!", "-0.07518070500292766"],
        svec!["5 stars! Highly recommended!", "0.3973495344831422"],
        svec!["What were they thinking!?!", "-0.19353437967075598"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_whatlang() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
       vec![
            svec!["description"],
            svec!["Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy competentes al escalar por las ramas."],
            svec!["See notes."],
            svec!["Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre la meva més gran preocupació."],
            svec![""],
            svec!["Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated fortifications still had defensive usefulness."],
            svec!["民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。"],
            svec!["Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である"],
            svec!["Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя заставил И лучше выдумать не мог."],
            svec!["Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high school?!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("whatlang")
        .arg("description")
        .arg("--new-column")
        .arg("language")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "language"],
        svec!["Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy competentes al escalar por las ramas.", "Spa"],
        svec!["See notes.", "Cat?"],
        svec!["Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre la meva més gran preocupació.", "Cat"],
        svec!["", ""],
        svec!["Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated fortifications still had defensive usefulness.", "Eng"],
        svec!["民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。", "Cmn"],
        svec!["Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である", "Jpn"],
        svec!["Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя заставил И лучше выдумать не мог.", "Rus"],
        svec!["Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high school?!", "Tgl"],
  ];
    assert_eq!(got, expected);
}

#[test]
fn apply_emptyreplace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec![" "],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("emptyreplace")
        .arg("--replacement")
        .arg("NA")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["John"],
        svec!["NA"],
        svec!["Sue"],
        svec!["Hopkins"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["Wed, 02 Jun 2021 06:31:39 GMT"],
            svec!["2009-01-20 05:00 EST"],
            svec!["July 4, 2005"],
            svec!["2021-05-01T01:17:02.604456Z"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt").arg("Created Date").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02T06:31:39+00:00"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04"],
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_multiple_cols() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date", "End Date"],
            svec![
                "September 17, 2012 10:09am EST",
                "September 18, 2012 10:09am EST"
            ],
            svec![
                "Wed, 02 Jun 2021 06:31:39 GMT",
                "Wed, 02 Jun 2021 08:31:39 GMT"
            ],
            svec!["2009-01-20 05:00 EST", "2009-01-21 05:00 EST"],
            svec!["July 4, 2005", "July 5, 2005"],
            svec!["2021-05-01T01:17:02.604456Z", "2021-05-02T01:17:02.604456Z"],
            svec![
                "This is not a date and it will not be reformatted",
                "This is not a date and it will not be reformatted"
            ],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date,End Date")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date", "End Date"],
        svec!["2012-09-17T15:09:00+00:00", "2012-09-18T15:09:00+00:00"],
        svec!["2021-06-02T06:31:39+00:00", "2021-06-02T08:31:39+00:00"],
        svec!["2009-01-20T10:00:00+00:00", "2009-01-21T10:00:00+00:00"],
        svec!["2005-07-04", "2005-07-05"],
        svec![
            "2021-05-01T01:17:02.604456+00:00",
            "2021-05-02T01:17:02.604456+00:00"
        ],
        svec![
            "This is not a date and it will not be reformatted",
            "This is not a date and it will not be reformatted"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_multiple_cols_rename() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date", "End Date"],
            svec![
                "September 17, 2012 10:09am EST",
                "September 18, 2012 10:09am EST"
            ],
            svec![
                "Wed, 02 Jun 2021 06:31:39 GMT",
                "Wed, 02 Jun 2021 08:31:39 GMT"
            ],
            svec!["2009-01-20 05:00 EST", "2009-01-21 05:00 EST"],
            svec!["July 4, 2005", "July 5, 2005"],
            svec!["2021-05-01T01:17:02.604456Z", "2021-05-02T01:17:02.604456Z"],
            svec![
                "This is not a date and it will not be reformatted",
                "This is not a date and it will not be reformatted"
            ],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date,End Date")
        .arg("--formatstr")
        .arg("%u")
        .arg("--rename")
        .arg("Created Weekday,End Weekday")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Weekday", "End Weekday"],
        svec!["1", "2"],
        svec!["3", "3"],
        svec!["2", "3"],
        svec!["1", "2"],
        svec!["6", "7"],
        svec![
            "This is not a date and it will not be reformatted",
            "This is not a date and it will not be reformatted"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_prefer_dmy() {
    let wrk = Workdir::new("apply_dmy");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["02/06/2021"],
            svec!["2009-01-20 05:00 EST"],
            svec!["July 4, 2005"],
            svec!["2021-05-01T01:17:02.604456Z"],
            svec!["10/05/71"],
            svec!["12/31/71"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date")
        .arg("--prefer-dmy")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04"],
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["1971-05-10"],
        svec!["1971-12-31"], // will still parse obviously valid mdy dates that are not valid as dmy
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_prefer_dmy_env() {
    let wrk = Workdir::new("apply_prefer_dmy_env");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["02/06/2021"],
            svec!["2009-01-20 05:00 EST"],
            svec!["July 4, 2005"],
            svec!["2021-05-01T01:17:02.604456Z"],
            svec!["10/05/71"],
            svec!["12/31/71"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.arg("datefmt").arg("Created Date").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04"],
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["1971-05-10"],
        svec!["1971-12-31"], // will still parse obviously valid mdy dates that are not valid as dmy
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_fmtstring() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["Wed, 02 Jun 2021 06:31:39 GMT"],
            svec!["2009-01-20 05:00 EST"],
            svec!["2015-09-30 18:48:56.35272715 UTC"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date")
        .arg("--formatstr")
        .arg("%a %b %e %T %Y %z")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["Mon Sep 17 15:09:00 2012 +0000"],
        svec!["Wed Jun  2 06:31:39 2021 +0000"],
        svec!["Tue Jan 20 10:00:00 2009 +0000"],
        svec!["Wed Sep 30 18:48:56 2015 +0000"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_fmtstring_with_literals() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["Wed, 02 Jun 2021 06:31:39 GMT"],
            svec!["2009-01-20 05:00 EST"],
            svec!["2015-09-30 18:48:56.35272715 UTC"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date")
        .arg("--formatstr")
        .arg("%c is day %j, week %V of %G")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["Mon Sep 17 15:09:00 2012 is day 261, week 38 of 2012"],
        svec!["Wed Jun  2 06:31:39 2021 is day 153, week 22 of 2021"],
        svec!["Tue Jan 20 10:00:00 2009 is day 020, week 04 of 2009"],
        svec!["Wed Sep 30 18:48:56 2015 is day 273, week 40 of 2015"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_fmtstring_notime() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["Wed, 02 Jun 2021 06:31:39 GMT"],
            svec!["2009-01-20 05:00 EST"],
            svec!["4/8/2014 14:13"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date")
        .arg("--formatstr")
        .arg("%Y-%m-%d")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17"],
        svec!["2021-06-02"],
        svec!["2009-01-20"],
        svec!["2014-04-08"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_geocode() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(  40.819342, -73.9532127    )"],
            svec!["< 40.819342,-73.9532127 >"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["The treasure is at these coordinates 40.66472342, -73.93867227. This should be geocoded."],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT be geocoded."],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("geocode").arg("Location").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["The Bronx, New York"],
        svec!["Brooklyn, New York"],
        svec!["Manhattan, New York"],
        svec!["Edgewater, New Jersey"],
        svec!["Edgewater, New Jersey"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brooklyn, New York"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
        svec!["The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT be geocoded."],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_geocode_fmtstring() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(40.819342, -73.9532127)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("geocode")
        .arg("Location")
        .arg("--formatstr")
        .arg("county-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Bronx, US"],
        svec!["Kings County, US"],
        svec!["New York County, US"],
        svec!["Bergen County, US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_geocode_fmtstring_intl() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["41.390205, 2.154007"],
            svec!["52.371807, 4.896029"],
            svec!["(52.520008, 13.404954)"],
            svec!["(14.55027,121.03269)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("geocode")
        .arg("Location")
        .arg("--formatstr")
        .arg("city-admin1-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Barcelona, Catalonia ES"],
        svec!["Amsterdam, North Holland NL"],
        svec!["Mitte, Berlin DE"],
        svec!["Makati City, Metro Manila PH"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}
