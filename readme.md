# Net Promoter Score (NPS®) 
[![Crates.io](https://img.shields.io/crates/v/net_promoter_score.svg)](https://crates.io/crates/net_promoter_score) [![build and test](https://github.com/rrrodzilla/net_promoter_score/actions/workflows/build%20and%20test.yml/badge.svg)](https://github.com/rrrodzilla/net_promoter_score/actions/workflows/build%20and%20test.yml) [![Documentation](https://docs.rs/net_promoter_score/badge.svg)](https://docs.rs/net_promoter_score) [![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

The `net_promoter_score` crate is a Rust library for working with and calculating Net Promoter Scores (NPS) from survey responses. The library is highly customizable, making it easy to work with different respondent ID types and score ranges. It includes methods for adding single and multiple responses, bulk responses, and auto-generated unique respondent IDs. Furthermore, the crate efficiently handles edge cases and has detailed error messages to ensure your application is robust and accurate.

## Features

- Calculate NPS from survey responses
- Support for custom respondent ID types
- Detailed error messages
- Efficient performance
- Clear and easy-to-understand examples
- Auto-generate unique respondent IDs (for `i32` identifiers)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
net_promoter_score = "0.2.0"
```

## Example Usage

### Creating a survey and adding responses

To create a new survey and add single responses to it, you can use the following code:

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

Adding multiple responses at once can be done using `add_multiple_responses` as shown below:

```rust
use net_promoter_score::prelude::*;
use anyhow::Result;

fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    let mut survey = Survey::new();

    survey.add_multiple_responses(vec![(1, 9), (2, 8), (3, 6)])?;

    let nps = survey.score();
    println!("The calculated NPS is: {}", nps);

    Ok(())
}
```

### Adding bulk responses with auto-generated respondent IDs of type i32

```rust
use net_promoter_score::prelude::*;
use anyhow::Result;

fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    let rating_quantities: &[(NpsRating, ScoreCount)] = &[
        (1, 2),
        (4, 1),
        (5, 2),
        (7, 8),
        (8, 10),
        (10, 10),
    ];
    let mut survey = Survey::new();
    survey.add_bulk_responses_auto_id(&rating_quantities)?;

    let nps = survey.score();
    println!("The calculated NPS is: {}", nps);

    Ok(())
}
```
### Here's an example using `&str` as the respondent ID type for the Survey:

```rust
use net_promoter_score::prelude::*;
use anyhow::Result;

fn main() -> Result<()> {
    // Create a new survey with &str respondent IDs
    let mut survey: Survey<&str> = Survey::new();

    // Adding responses with string slice respondent IDs
    survey.add_response("customer_1", 9)?;
    survey.add_response("customer_2", 8)?;
    survey.add_response("customer_3", 6)?;

    // Calculate the NPS
    let nps = survey.score();
    println!("The calculated NPS is: {}", nps);

    Ok(())
}
```

### Want more control over how IDs are generated?
Here's an example using a respondent ID generator function with custom `String` respondent IDs:

```rust
use net_promoter_score::prelude::*;
use anyhow::Result;

fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    let rating_quantities: &[(NpsRating, ScoreCount)] = &[
        (1, 2),
        (4, 1),
        (5, 2),
        (7, 8),
        (8, 10),
        (10, 10),
    ];

    let mut survey: Survey<String> = Survey::new();

    let mut respondent_id_number = 1;
    let respondent_id_fn = || {
        let current_id = format!("customer_{}", respondent_id_number);
        respondent_id_number += 1;
        current_id
    };

    survey.add_bulk_responses(respondent_id_fn, rating_quantities)?;

    let nps = survey.score();
    println!("The calculated NPS is: {}", nps);

    Ok(())
}
```

In this example, we first define `rating_quantities` containing the NPS rating and respective count for each rating. We create a new `Survey` with `String` as the respondent ID type. Next, we create a respondent ID generator function `respondent_id_fn` which generates unique custom `String` respondent IDs in the format `"customer_{number}"`.

Finally, we call `add_bulk_responses` with the generator function and the rating_quantities, which adds responses to the survey with unique respondent IDs. The calculated NPS is printed to the console.


**For specific details and more advanced use cases, please consult the crate documentation and source code.**

## API Overview

The main type provided by the crate is the `Survey` struct. The `Survey` includes methods for adding responses and calculating the NPS:

- `new()`: Creates a new empty survey.
- `add_response(respondent_id: T, score: u8)`: Adds a single survey response with the given respondent ID and score.
- `add_multiple_responses(responses: impl IntoIterator<Item = (T, u8)>)`: Adds multiple survey responses.
- `add_bulk_responses(respondent_id_fn: F, quantities: &[(u8, usize)])`: Adds bulk survey responses with a respondent ID generator function and a slice of tuples (rating, quantity).
- `add_bulk_responses_auto_id(quantities: &[(u8, usize)])`: Adds bulk survey responses with auto-generated unique respondent IDs of type `i32`, starting at 1 (specialized implementation for respondent IDs of type i32).
- `from_responses(responses: impl IntoIterator<Item = (T, u8)>)`: Creates a new survey from a set of responses. If any responses have an invalid rating, an error will be returned.
- `score()`: Calculates and returns the Net Promoter Score (NPS) of the survey.

## Feedback and Contributions

I appreciate any feedback and suggestions to improve this crate. Feel free to [open an issue](https://github.com/rrrodzilla/net_promoter_score/issues/new) or [submit a pull request](https://github.com/rrrodzilla/net_promoter_score/compare) if you want to contribute to the project directly.

## License

This crate is free software: you can redistribute it and/or modify it under the terms of the [MIT](https://choosealicense.com/licenses/mit) License.

Net Promoter®, NPS®, NPS Prism®, and the NPS-related emoticons are registered trademarks of Bain & Company, Inc., NICE Systems, Inc., and Fred Reichheld. Net Promoter Score and Net Promoter System are service marks of Bain & Company, Inc., NICE Systems, Inc., and Fred Reichheld.

---

## Like this crate?

⭐ Star     https://github.com/rrrodzilla/net_promoter_score

🐦 Follow   https://twitter.com/rrrodzilla

---

<h5 align="center">readme created with <a href="https://crates.io/crates/cargo-markdown">cargo-markdown</a></h5>
