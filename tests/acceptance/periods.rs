mod literal_periods {
    use crate::common::*;
    use rstest::rstest;

    #[rstest]
    fn period_filter_should_show_this_week_when_specified(
        #[values("this-week", "tw")] this_week: &str,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-02
        - #dev 2h Task2
        ## TT 2020-01-20
        - #dev 1h Task3";

        Cmd::given()
            .at_date("2020-01-01") // Testing as if we're running on Jan 1st
            .period_filter(this_week)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output("week 1, 2020")
            .expect_project("dev")
            .taking("3h 00m") // Only tasks from Jan 1-2
            .validate();
    }

    #[rstest]
    fn period_filter_should_show_last_week_when_specified(
        #[values("last-week", "lw")] last_week: &str,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-02
        - #dev 2h Task2
        ## TT 2020-01-08
        - #dev 1h Task3";

        Cmd::given()
            .at_date("2020-01-08") // Testing as if we're running on Jan 8th
            .period_filter(last_week)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output("week 1, 2020")
            .expect_project("dev")
            .taking("3h 00m") // Only tasks from Jan 1-2 (last week)
            .validate();
    }

    #[rstest]
    fn period_filter_should_show_last_month_when_specified(
        #[values("last-month", "lm")] last_month: &str,
        #[values("2020-02-01", "2020-02-02", "2020-02-28")] at_date: &str,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-31
        - #dev 2h Task2
        ## TT 2020-02-01
        - #dev 1h Task3";

        Cmd::given()
            .at_date(at_date)
            .period_filter(last_month)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output("2020-01")
            .expect_project("dev")
            .taking("3h 00m") // Only tasks from Jan (last month)
            .validate();
    }

    #[rstest]
    fn period_filter_should_show_this_month_when_specified(
        #[values("this-month", "tm")] this_month: &str,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-31
        - #dev 2h Task2
        ## TT 2020-02-01
        - #dev 1h Task3";

        Cmd::given()
            .at_date("2020-01-01")
            .period_filter(this_month)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output("2020-01")
            .expect_project("dev")
            .taking("3h 00m") // Only tasks from Jan (this month)
            .validate();
    }

    #[rstest]
    fn period_filter_should_show_today_when_specified(
        #[values("today", "t")] today: &str,
        #[values(
            ("2020-01-01", "1h 00m"),
            ("2020-01-02", "2h 00m"),
            ("2020-01-03", "4h 00m"),
        )]
        at_date_and_expected_duration: (&str, &str),
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-02
        - #dev 2h Task2
        ## TT 2020-01-03
        - #dev 4h Task3";

        let at_date = at_date_and_expected_duration.0;

        let expected_output = format!("of {}", at_date);
        let expected_duration = at_date_and_expected_duration.1;

        Cmd::given()
            .at_date(at_date)
            .period_filter(today)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_duration) // Only tasks of 'today'
            .validate();
    }

    struct YesterdayReport<'a> {
        input_clock_date: &'a str,
        expected_reported_date: &'a str,
        expected_reported_duration: &'a str,
    }

    #[rstest]
    fn period_filter_should_show_yesterday_when_specified(
        #[values("yesterday", "y")] yesterday: &str,
        #[values(
            YesterdayReport{input_clock_date: "2020-01-02", expected_reported_date: "2020-01-01", expected_reported_duration: "1h 00m"},
            YesterdayReport{input_clock_date: "2020-01-03", expected_reported_date: "2020-01-02", expected_reported_duration: "2h 00m"},
            YesterdayReport{input_clock_date: "2020-01-04", expected_reported_date: "2020-01-03", expected_reported_duration:    "4h 00m"},
        )]
        test_data: YesterdayReport,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-02
        - #dev 2h Task2
        ## TT 2020-01-03
        - #dev 4h Task3";

        let clock_date = test_data.input_clock_date;

        let expected_output = format!("of {}", test_data.expected_reported_date);
        let expected_duration = test_data.expected_reported_duration;

        Cmd::given()
            .at_date(clock_date)
            .period_filter(yesterday)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_duration) // Only tasks of 'yesterday'
            .validate();
    }
}

mod value_periods {
    use crate::common::*;
    use rstest::rstest;

    #[rstest]
    fn period_filter_should_show_month_1_when_specified(
        #[values("month-1", "m-1")] month_1: &str,
        #[values(
            ("2019", "1h 00m", "2019-01"),
            ("2020", "2h 00m", "2020-01"),
            ("2021", "8h 00m", "2021-01"),
        )]
        data: (&str, &str, &str),
    ) {
        let content = r"## TT 2019-01-01
        - #dev 1h Task1
        ## TT 2020-01-01
        - #dev 2h Task2
        ## TT 2020-02-01
        - #dev 4h Task4
        ## TT 2021-01-01
        - #dev 8h Task5";

        let at_year = data.0;
        let at_date = format!("{at_year}-01-01");
        let expected_taking = data.1;
        let expected_output = data.2;

        Cmd::given()
            .at_date(&at_date) // Testing as if we are running in year 'at_year'
            .period_filter(month_1)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_taking)
            .validate();
    }

    #[rstest]
    fn period_filter_should_show_month_2_when_specified(
        #[values("month-2", "m-2")] month_2: &str,
        #[values(
            ("2019", "1h 00m", "2019-02"),
            ("2020", "2h 00m", "2020-02"),
            ("2021", "4h 00m", "2021-02"),
        )]
        data: (&str, &str, &str),
    ) {
        let content = r"## TT 2019-02-01
        - #dev 1h Task1
        ## TT 2020-02-01
        - #dev 2h Task2
        ## TT 2021-02-01
        - #dev 4h Task3";

        let at_year = data.0;
        let at_date = format!("{at_year}-01-01");
        let expected_taking = data.1;
        let expected_output = data.2;

        Cmd::given()
            .at_date(&at_date) // Testing as if we are running in year 'at_year'
            .period_filter(month_2)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_taking)
            .validate();
    }

    struct DateValueSpecified<'a> {
        date_value: &'a str,
        expectations: (&'a str, &'a str),
    }

    #[rstest]
    fn period_filter_should_show_specific_date_when_specified(
        #[values(
            DateValueSpecified{date_value: "2020-01-01", expectations:  ("2020-01-01", "1h 00m")},
            DateValueSpecified{date_value: "2020-01-02", expectations:  ("2020-01-02", "2h 00m")}
        )]
        test_data: DateValueSpecified,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-02
        - #dev 2h Task2
        ";

        let date_value = test_data.date_value;

        let expected_output = test_data.expectations.0;
        let expected_taking = test_data.expectations.1;

        Cmd::given()
            .period_filter(date_value)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_taking)
            .validate();
    }

    struct MonthValueSpecified<'a> {
        month_value: &'a str,
        expectations: (&'a str, &'a str),
    }

    #[rstest]
    fn period_filter_should_show_specific_month_when_specified(
        #[values(
            MonthValueSpecified{month_value: "2020-01", expectations:  ("2020-01", "1h 00m")},
            MonthValueSpecified{month_value: "2020-1", expectations:  ("2020-01", "1h 00m")},
            MonthValueSpecified{month_value: "2020-02", expectations:  ("2020-02", "2h 00m")},
        )]
        test_data: MonthValueSpecified,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-02-01
        - #dev 2h Task2
        ";

        let month_value = test_data.month_value;

        let expected_output = test_data.expectations.0;
        let expected_taking = test_data.expectations.1;

        Cmd::given()
            .period_filter(month_value)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_taking)
            .validate();
    }

    struct YearValueSpecified<'a> {
        year_value: &'a str,
        expectations: (&'a str, &'a str),
    }

    #[rstest]
    fn period_filter_should_show_specific_year_when_specified(
        #[values(
            YearValueSpecified{year_value: "2020", expectations:  ("2020", "1h 00m")},
            YearValueSpecified{year_value: "2021", expectations:  ("2021", "2h 00m")}
        )]
        test_data: YearValueSpecified,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2021-01-01
        - #dev 2h Task2
        ";

        let year_value = test_data.year_value;

        let expected_output = test_data.expectations.0;
        let expected_taking = test_data.expectations.1;

        Cmd::given()
            .period_filter(year_value)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_taking)
            .validate();
    }

    struct WeekValueSpecified<'a> {
        week_value: &'a str,
        expectations: (&'a str, &'a str),
    }

    #[rstest]
    fn period_filter_should_show_specific_week_when_specified(
        #[values(
            WeekValueSpecified{week_value: "2020-w01", expectations:  ("week 1", "1h 00m")},
            WeekValueSpecified{week_value: "2020-w1", expectations:  ("week 1", "1h 00m")},
            WeekValueSpecified{week_value: "2020-w02", expectations:  ("week 2", "2h 00m")}
        )]
        test_data: WeekValueSpecified,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1
        ## TT 2020-01-08
        - #dev 2h Task2
        ";

        let week_value = test_data.week_value;

        let expected_output = test_data.expectations.0;
        let expected_taking = test_data.expectations.1;

        Cmd::given()
            .period_filter(week_value)
            .a_file_with_content(content)
            .when_run()
            .should_succeed()
            .expect_output(&expected_output)
            .expect_project("dev")
            .taking(expected_taking)
            .validate();
    }
}

mod invalid_periods {
    use crate::common::*;
    use rstest::rstest;

    #[rstest]
    fn period_filter_should_fail_when_period_invalid(
        #[values(
            "abc",
            "month-0",
            "month-13",
            "m-0",
            "m-13",
            "2020-w00",
            "2020-w53",
            "2020-00",
            "2020-13",
            "2020-00-01",
            "2020-00-00",
            "2020-01-32"
        )]
        invalid_period_value: &str,
    ) {
        let content = r"## TT 2020-01-01
        - #dev 1h Task1";

        let at_date = "2020-01-01";

        Cmd::given()
            .at_date(at_date)
            .period_filter(invalid_period_value)
            .a_file_with_content(content)
            .when_run()
            .should_fail();
    }
}
