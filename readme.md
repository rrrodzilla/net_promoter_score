# Net Promoter Score (NPS&reg;) [![Crates.io](https://img.shields.io/crates/v/net_promoter_score.svg)](https://crates.io/crates/net_promoter_score)

A simple and easy-to-use Rust crate for calculating Net Promoter Scores (NPS) from survey responses. Survey responses are gathered with respondent IDs and accompanied by a rating from 0 to 10. The crate handles various edge cases and provides detailed error messages to help you write robust and accurate NPS calculators for your applications.

## Features

- Calculate NPS from survey responses
- Supports custom respondent ID types
- Detailed error messages
- Great performance
- Easy-to-understand usage examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
net_promoter_score = "0.1.0"
```

## Example Usage

```rust
use net_promoter_score::prelude::*;
use anyhow::Result;

fn main() -> Result<()> {
    let mut survey = Survey::new();

    survey.add_response(1, 9)?;
    survey.add_response(2, 8)?;
    survey.add_response(3, 6)?;

    let nps = survey.score();
    println!("The calculated NPS is: {}", nps);

    Ok(())
}
```

## API

The main type provided by the crate is the `Survey` struct. The `Survey` has the following methods for adding responses and calculating the NPS:

- `new()`: Creates a new empty survey.
- `add_response(respondent_id: T, score: u8)`: Adds a single survey response with the given respondent ID and score.
- `add_multiple_responses(responses: impl IntoIterator<Item = (T, u8)>)`: Adds multiple survey responses.
- `add_bulk_responses(respondent_id_fn: F, quantities: &[(u8, usize)])`: Adds bulk survey responses with a mutable respondent ID generator function and a slice of tuples (rating, quantity).
- `add_bulk_responses_auto_id(quantities: &[(u8, usize)])`: Adds bulk survey responses with auto-generated unique respondent IDs of type `i32`, starting at 1 (specialized implementation for respondent IDs of type i32).
- `from_responses(responses: impl IntoIterator<Item = (T, u8)>)`: Creates a new survey from a given set of responses. If any responses have an invalid rating, an error will be returned.
- `score()`: Calculates and returns the Net Promoter Score (NPS) of the survey.

## Feedback and Contributions

I appreciate every feedback and suggestion you might have to improve this crate. Feel free to [open an issue](https://github.com/rrrodzilla/net_promoter_score/issues/new) or [submit a pull request](https://github.com/rrrodzilla/net_promoter_score/compare) if you want to contribute to the project directly.

## License

This crate is free software: you can redistribute it and/or modify it under the terms of the [MIT](https://choosealicense.com/licenses/mit) License.

Net Promoter®, NPS®, NPS Prism®, and the NPS-related emoticons are registered trademarks of Bain & Company, Inc., NICE Systems, Inc., and Fred Reichheld. Net Promoter Score and Net Promoter System are service marks of Bain & Company, Inc., NICE Systems, Inc., and Fred Reichheld.

--- 

## Like this crate?

⭐ Star     https://github.com/rrrodzilla/net_promoter_score

🐦 Follow   https://twitter.com/rrrodzilla

---

<h5 align="center">readme created with <a href="https://crates.io/crates/cargo-markdown">cargo-markdown</a></h5>
