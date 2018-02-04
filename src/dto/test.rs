use dto::Request;

#[test]
fn should_not_build_request_for_no_args() {
    let args = vec![];
    let request = Request::new(&args, None);
    assert_eq!(request.current, None);
    assert_eq!(request.next.is_empty(), true);
}

#[test]
fn should_build_request_from_vector_of_strings() {
    let args = vec![s!("first-command"), s!("second-command"), s!("third-command")];

    let request = Request::new(&args, None);

    assert_eq!(request.current, Some(&args[0]));
    assert_eq!(request.next, &args[1..3]);
}

#[test]
fn should_advance_request_with_multiple_args() {
    let args = vec![s!("first-command"), s!("second-command"), s!("third-command")];

    let request = Request::new(&args, None);

    let next_request = request.next();

    assert_eq!(next_request.current, Some(&args[1]));
    assert_eq!(next_request.next, &args[2..3]);
}

#[test]
fn should_create_request_with_single_arg() {
    let args = vec![s!("first-command")];

    let request = Request::new(&args, None);

    assert_eq!(request.current, Some(&args[0]));
    assert_eq!(request.next.len(), 0);
}

#[test]
fn should_advance_request_with_single_arg() {
    let args = vec![s!("first-command")];

    let request = Request::new(&args, None).next();

    assert_eq!(request.current, None);
    assert_eq!(request.next.len(), 0);
}

#[test]
fn should_not_request_autocomplete_when_autocomplete_index_is_zero() {
    let args = vec![s!("first-command")];

    let request = Request::new(&args, None);

    assert_eq!(request.autocomplete(), false);
}

#[test]
fn should_not_request_autocomplete_when_autocomplete_index_matched_current_command() {
    let args = vec![s!("first-command")];

    let request = Request::new(&args, Some(1));

    assert_eq!(request.autocomplete(), false);
}

#[test]
fn should_not_request_autocomplete_when_completed_index_is_greater_than_total_commands() {
    let args = vec![s!("first-command")];

    let request = Request::new(&args, Some(2));

    assert_eq!(request.autocomplete(), false);
}

#[test]
fn should_request_autocomplete_when_completed_index_is_one_less_than_current_command() {
    let args = vec![s!("first-command"), s!("second-command"), s!("third-command")];

    let request = Request::new(&args, Some(1)).next();

    assert_eq!(request.autocomplete(), true);
}

#[test]
fn should_be_autocomplete_enable_if_an_argument_requires_completion() {
    let args = vec![s!("first-command")];

    let request = Request::new(&args, Some(1));

    assert_eq!(request.autocomplete_enabled(), true);
}