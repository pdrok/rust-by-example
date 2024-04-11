fn main() {
    // Make `optional` of type `Option<i32`
    let mut optional = Some(0);

    // This reads: "While `let` destructures `optional` into"
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }

        // ^ Less rightward drft sand doesn't require
        // explicity handling the failing case.
    }

    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}