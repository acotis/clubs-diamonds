
pub fn compare_sets(
    expected: &mut [&str],
    actual: &mut [&str],
) -> Result<(), String> 
{
    expected.sort(); expected.reverse();
    actual.sort();   actual.reverse();

    // Find what's missing and what's extra.

    let mut expected_not_found = vec![];
    let mut found_not_expected = vec![];

    let mut e = 0;
    let mut a = 0;

    while e < expected.len() || a < actual.len() {
        if actual.get(a) > expected.get(e) {
            found_not_expected.push(actual[a]);
            a = (a..).find(|&new_a| actual  .get(new_a) != actual  .get(a)).unwrap();
        } else if expected.get(e) > actual.get(a) {
            expected_not_found.push(expected[e]);
            e = (e..).find(|&new_e| expected.get(new_e) != expected.get(e)).unwrap();
        } else {
            a = (a..).find(|&new_a| actual  .get(new_a) != actual  .get(a)).unwrap();
            e = (e..).find(|&new_e| expected.get(new_e) != expected.get(e)).unwrap();
        }
    }

    // Find duplicated items.

    let mut duplicated_items = vec![];

    for i in 1..actual.len() {
        if actual[i] == actual[i-1] {
            if duplicated_items.last().is_some_and(|item: &(&str, usize)| item.0 == actual[i]) {
                duplicated_items.last_mut().unwrap().1 += 1;
            } else {
                duplicated_items.push((actual[i], 2));
            }
        }
    }

    // Create the error string and return it.

    let mut error_string = format!("");

    if !expected_not_found.is_empty() {
        error_string += &shorten(format!("Missing items: {}\n", expected_not_found.join(", ")));
    }

    if !found_not_expected.is_empty() {
        error_string += &shorten(format!("Unexpected items: {}\n", found_not_expected.join(", ")));
    }

    if !duplicated_items.is_empty() {
        error_string += &shorten(format!("Duplicated items: {}\n", duplicated_items.iter().map(|(item, count)| format!("{item} [{count}]")).collect::<Vec<_>>().join(", ")));
    }

    if error_string == "" {
        Ok(())
    } else {
        Err(error_string.trim_end().to_owned())
    }
}

fn shorten(mut string: String) -> String {
    if string.len() < 70 {
        string
    } else {
        string.truncate(70);
        string + "..."
    }
}

fn duplicated_items<'a>(list: &[&'a str]) -> Vec<(&'a str, usize)> {
    let mut ret = vec![];

    for i in 0..list.len() {
        let item = list[i];

        if list[i+1..].contains(&item) && !list[..i].contains(&item) {
            ret.push((
                item,
                list.iter().filter(|x| **x == item).count()
            ));
        }
    }

    ret
}

#[test]
fn compare_sets_out_of_order() {
    assert_eq!(
        compare_sets(
            &mut ["hi", "hello"],
            &mut ["hello", "hi"]
        ),
        Ok(())
    );
}

#[test]
fn compare_sets_just_missing() {
    assert_eq!(
        compare_sets(
            &mut ["hi", "hello", "world"],
            &mut ["hi", "world"],
        ),
        Err(format!("Missing items: hello"))
    );
}

#[test]
fn compare_sets_just_unexpected() {
    assert_eq!(
        compare_sets(
            &mut ["3+2*(4>>a)%9", "2%(8/4/a)"],
            &mut ["2%(8/4/a)", "3+2*(4>>a)%9", "2/9+a/4>>1"]
        ),
        Err(format!("Unexpected items: 2/9+a/4>>1"))
    );
}

#[test]
fn compare_sets_just_duplicated() {
    assert_eq!(
        compare_sets(
            &mut ["hi", "there", "you", "friend"],
            &mut ["hi", "there", "you", "friend", "you", "you", "hi"],
        ),
        Err(format!("Duplicated items: you [3], hi [2]"))
    )
}

#[test]
fn compare_sets_three_errors() {
    assert_eq!(
        compare_sets(
            &mut ["hi", "there", "you", "friend", "pona"],
            &mut ["there", "you", "friend", "world", "lettuce", "there", "lettuce"],
        ),
        Err(
            format!("Missing items: pona, hi\n") +
           &format!("Unexpected items: world, lettuce\n") +
           &format!("Duplicated items: there [2], lettuce [2]")
        )
    );
}

