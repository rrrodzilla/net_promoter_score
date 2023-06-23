//! Net Promoter Score (NPS®) [![Crates.io](https://img.shields.io/crates/v/net_promoter_score.svg)](https://crates.io/crates/net_promoter_score)
//!
//! The net_promoter_score crate is a Rust library for working with and calculating Net Promoter Scores (NPS) from survey responses. The library is highly customizable, making it easy to work with different respondent ID types and score ranges. It includes methods for adding single and multiple responses, bulk responses, and auto-generated unique respondent IDs. Furthermore, the crate efficiently handles edge cases and has detailed error messages to ensure your application is robust and accurate.
//!
//! ## Features
//!
//! - Calculate NPS from survey responses
//! - Support for custom respondent ID types
//! - Detailed error messages
//! - Efficient performance
//! - Clear and easy-to-understand examples
//! - Auto-generate unique respondent IDs (for `i32` identifiers)
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! net_promoter_score = "0.1.1"
//! ```
//!
//! ## Example Usage
//!
//! ### Creating a survey and adding responses
//!
//! To create a new survey and add single responses to it, you can use the following code:
//!
//! ```rust
//! use net_promoter_score::prelude::*;
//! use anyhow::Result;
//!
//! fn main() -> Result<()> {
//!     let mut survey = Survey::new();
//!
//!     survey.add_response(1, 9)?;
//!     survey.add_response(2, 8)?;
//!     survey.add_response(3, 6)?;
//!
//!     let nps = survey.score();
//!     println!("The calculated NPS is: {}", nps);
//!
//!     Ok(())
//! }
//! ```
//!
//! Adding multiple responses at once can be done using `add_multiple_responses` as shown below:
//!
//! ```rust
//! use net_promoter_score::prelude::*;
//! use anyhow::Result;
//!
//! fn main() -> Result<(), Vec<NetPromoterScoreError>> {
//!     let mut survey = Survey::new();
//!
//!     survey.add_multiple_responses(vec![(1, 9), (2, 8), (3, 6)])?;
//!
//!     let nps = survey.score();
//!     println!("The calculated NPS is: {}", nps);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Adding bulk responses with auto-generated respondent IDs of type i32
//!
//! ```rust
//! use net_promoter_score::prelude::*;
//! use anyhow::Result;
//!
//! fn main() -> Result<(), Vec<NetPromoterScoreError>> {
//!     let rating_quantities: &[(NpsRating, ScoreCount)] = &[
//!         (1, 2),
//!         (4, 1),
//!         (5, 2),
//!         (7, 8),
//!         (8, 10),
//!         (10, 10),
//!     ];
//!     let mut survey = Survey::new();
//!     survey.add_bulk_responses_auto_id(&rating_quantities)?;
//!
//!     let nps = survey.score();
//!     println!("The calculated NPS is: {}", nps);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Here's an example using `&str` as the respondent ID type for the Survey:
//!
//! ```rust
//! use net_promoter_score::prelude::*;
//! use anyhow::Result;
//!
//! fn main() -> Result<()> {
//!     // Create a new survey with &str respondent IDs
//!     let mut survey: Survey<&str> = Survey::new();
//!
//!     // Adding responses with string slice respondent IDs
//!     survey.add_response("customer_1", 9)?;
//!     survey.add_response("customer_2", 8)?;
//!     survey.add_response("customer_3", 6)?;
//!
//!     // Calculate the NPS
//!     let nps = survey.score();
//!     println!("The calculated NPS is: {}", nps);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Want more control over how IDs are generated?
//! Here's an example using a respondent ID generator function with custom `String` respondent IDs:
//!
//! ```rust
//! use net_promoter_score::prelude::*;
//! use anyhow::Result;
//!
//! fn main() -> Result<(), Vec<NetPromoterScoreError>> {
//!     let rating_quantities: &[(NpsRating, ScoreCount)] = &[
//!         (1, 2),
//!         (4, 1),
//!         (5, 2),
//!         (7, 8),
//!         (8, 10),
//!         (10, 10),
//!     ];
//!
//!     let mut survey: Survey<String> = Survey::new();
//!
//!     let mut respondent_id_number = 1;
//!     let respondent_id_fn = || {
//!         let current_id = format!("customer_{}", respondent_id_number);
//!         respondent_id_number += 1;
//!         current_id
//!     };
//!
//!     survey.add_bulk_responses(respondent_id_fn, rating_quantities)?;
//!
//!     let nps = survey.score();
//!     println!("The calculated NPS is: {}", nps);
//!
//!     Ok(())
//! }
//! ```
//!
//! In this example, we first define `rating_quantities` containing the NPS rating and respective count for each rating. We create a new `Survey` with `String` as the respondent ID type. Next, we create a respondent ID generator function `respondent_id_fn` which generates unique custom `String` respondent IDs in the format `"customer_{number}"`.
//!
//! Finally, we call `add_bulk_responses` with the generator function and the rating_quantities, which adds responses to the survey with unique respondent IDs. The calculated NPS is printed to the console.
//!
//! ## Feedback and Contributions
//!
//! I appreciate any feedback and suggestions to improve this crate. Feel free to [open an issue](https://github.com/rrrodzilla/net_promoter_score/issues/new) or [submit a pull request](https://github.com/rrrodzilla/net_promoter_score/compare) if you want to contribute to the project directly.
//!
//! ## License
//!
//! This crate is free software: you can redistribute it and/or modify it under the terms of the [MIT](https://choosealicense.com/licenses/mit) License.
//!
//! Net Promoter®, NPS®, NPS Prism®, and the NPS-related emoticons are registered trademarks of Bain & Company, Inc., NICE Systems, Inc., and Fred Reichheld. Net Promoter Score and Net Promoter System are service marks of Bain & Company, Inc., NICE Systems, Inc., and Fred Reichheld.
//!
//! ---
//!
//! ## Like this crate?
//!
//! ⭐ Star     <https://github.com/rrrodzilla/net_promoter_score>
//!
//! 🐦 Follow   <https://twitter.com/rrrodzilla>
//!

pub mod prelude;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::iter::{Extend, FromIterator};
use std::ops::Deref;

/// A `Survey` represents a collection of survey responses, where each response
/// includes a respondent's ID of type `T` and a score in the range of 0 to 10.
/// Responses are stored in a BTreeMap, which maintains the responses in order.
///
/// The primary purpose of the [net_promoter_score](crate) crate is to calculate the Net
/// Promoter Rating (NPS) based on the gathered responses.
///
/// # Example
///
/// ```
/// use net_promoter_score::prelude::*;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let mut survey = Survey::new();
///
///     survey.add_response("customer 1", 9)?;
///     survey.add_response("customer 17", 8)?;
///     survey.add_response("customer 34", 6)?;
///
///     let nps = survey.score();
///     println!("The calculated NPS is: {}", nps);
///
///     Ok(())
/// }
/// ```
pub struct Survey<T> {
    responses: BTreeMap<T, SurveyResponse<T>>,
    nps_cache: Option<i32>,
}

/// Represents the count or frequency of a particular score in a Net Promoter Score (NPS) survey.
///
/// `ScoreCount` is an alias for the `usize` type from the `net_promoter_score` crate's prelude,
/// indicating the number of occurrences or responses associated with a specific NPS score. It is
/// typically used in conjunction with `NpsRating` to store and manipulate NPS survey data.
///
/// # Examples
///
/// ```
/// # use net_promoter_score::prelude::*;
///
/// fn main() {
///     let count: ScoreCount = 10;
///     println!("Score count: {}", count);
/// }
/// ```
///
/// In the above example, the `ScoreCount` type alias from the `net_promoter_score` crate's prelude
/// is used to declare a variable `count`, which represents the count of occurrences for a specific
/// NPS score. The value `10` is assigned to `count`, and it is then printed to the console.
pub type ScoreCount = usize;
/// Represents the rating of a Net Promoter Score (NPS) survey on a scale from 0 to 10.
///
/// `NpsRating` is an alias for the `u8` type, indicating the numerical value assigned to the
/// likelihood of recommending a product, service, or experience in an NPS survey. It is typically
/// used in conjunction with `ScoreCount` to store and analyze NPS survey data.
///
/// # Example
///
/// ```
/// # use net_promoter_score::prelude::*;
/// let rating: NpsRating = 8;
/// println!("NPS Rating: {}", rating);
/// ```
///
/// In the above example, the `NpsRating` type alias is used to declare a variable `rating`,
/// which represents the NPS rating given by a participant. The value `8` is assigned to `rating`,
/// and it is then printed to the console.
pub type NpsRating = u8;

impl<T: PartialEq + Ord + Clone> Survey<T> {
    /// Creates a new empty survey.
    pub fn new() -> Self {
        Default::default()
    }

    fn calculate_nps(&mut self) {
        let total_responses = self.responses.len() as i32;
        if total_responses == 0 {
            self.nps_cache = Some(0);
            return;
        }

        let promoters = self.segment(Classification::Promoter).len() as i32;
        let detractors = self.segment(Classification::Detractor).len() as i32;

        let promoter_percent = 100 * promoters / total_responses;
        let detractor_percent = 100 * detractors / total_responses;

        self.nps_cache = Some(promoter_percent - detractor_percent);
    }
    /// Adds survey responses with their quantities to the survey.
    ///
    /// This method accepts a respondent ID generator function and a slice of tuples,
    /// where the first element is the rating (from 0 to 10) of type `NpsRating` and the second
    /// element is its quantity of type `ScoreCount`. It repeats the insertion of a response with
    /// the given rating `quantity` number of times.
    ///
    /// # Examples
    ///
    /// ```
    /// # use net_promoter_score::prelude::*;
    /// # use anyhow::Result;
    /// fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    ///     let rating_quantities: &[(NpsRating, ScoreCount)] = &[
    ///         (1, 2),
    ///         (4, 1),
    ///         (5, 2),
    ///         (7, 8),
    ///         (8, 10),
    ///         (10, 10),
    ///     ];
    ///
    ///     let mut survey = Survey::new();
    ///
    ///     let mut respondent_id = 1;
    ///     let respondent_id_fn = || {
    ///         let current_id = respondent_id;
    ///         respondent_id += 1;
    ///         current_id
    ///     };
    ///     
    ///     survey.add_bulk_responses(respondent_id_fn, rating_quantities)?;
    ///
    ///     let nps = survey.score();
    ///     println!("The calculated NPS is: {}", nps);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a vector of `NetPromoterScoreError` if any of the responses could not be added to the survey.
    pub fn add_bulk_responses<F>(
        &mut self,
        mut respondent_id_fn: F,
        nps_scores: &[(NpsRating, ScoreCount)],
    ) -> Result<(), Vec<NetPromoterScoreError>>
    where
        F: FnMut() -> T,
    {
        let errors: Vec<NetPromoterScoreError> = nps_scores
            .iter()
            .flat_map(|&(score, nps_scores)| {
                let responses: Vec<(T, u8)> = (0..nps_scores)
                    .map(|_| (respondent_id_fn(), score))
                    .collect();
                self.add_multiple_responses(responses)
                    .err()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .collect();

        if errors.is_empty() {
            self.calculate_nps();
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Creates a new survey from a given set of responses.
    ///
    /// If any of the responses have an invalid rating, an error will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use net_promoter_score::prelude::*;
    /// # use anyhow::Error;
    /// let survey_result: Result<Survey<u32>, Vec<Error>> =
    ///     Survey::<u32>::from_responses(vec![(1, 10), (2, 9), (3, 9), (4, 8), (5, 7), (6, 6)]);
    /// ```
    pub fn from_responses<E>(
        responses: impl IntoIterator<Item = (T, NpsRating)>,
    ) -> Result<Self, Vec<E>>
    where
        E: From<NetPromoterScoreError>,
    {
        let mut survey = Self::new();
        match survey.add_multiple_responses(responses) {
            Ok(_) => Ok(survey),
            Err(errors) => Err(errors.into_iter().map(E::from).collect()),
        }
    }

    /// Adds a response with the given respondent ID and score to the survey.
    ///
    /// Returns a result indicating whether the addition was successful.
    ///
    /// # Example
    ///
    /// ```
    /// use net_promoter_score::prelude::*;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let mut survey = Survey::new();
    ///
    ///     // Adding responses to the survey
    ///     survey.add_response("customer 1", 9)?;
    ///     survey.add_response("customer 17", 8)?;
    ///     survey.add_response("customer 34", 6)?;
    ///
    ///     // Calculating the NPS
    ///     let nps = survey.score();
    ///     println!("The calculated NPS is: {}", nps);
    ///     # assert_eq!(0,nps );
    ///     Ok(())
    /// }
    /// ```
    ///
    /// In the example above, a new `Survey` is created using the `new` method. Three responses are then added to the survey using the `add_response` method, with each response including a respondent ID and a score from 0 to 10. After adding the responses, the `score` method is called to calculate the Net Promoter Score (NPS) of the survey, which is then printed to the console.
    ///
    /// If any of the responses have an invalid rating, an error will be returned.
    pub fn add_response(
        &mut self,
        respondent_id: T,
        score: NpsRating,
    ) -> Result<(), NetPromoterScoreError> {
        let response = SurveyResponse::new(respondent_id.clone(), score)?;
        self.responses.insert(respondent_id, response);
        Ok(())
    }

    /// Adds multiple responses to the survey.
    ///
    /// If any of the responses have an invalid rating, a `Vec<NetPromoterScoreError>` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use net_promoter_score::prelude::*;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    ///     let mut survey = Survey::new();
    ///
    ///     // Adding multiple responses to the survey
    ///     survey.add_multiple_responses(vec![(1, 9), (2, 8), (3, 6)])?;
    ///
    ///     // Calculating the NPS
    ///     let nps = survey.score();
    ///     println!("The calculated NPS is: {}", nps);
    ///     # assert_eq!(nps,0 );
    ///     Ok(())
    /// }
    /// ```
    ///
    /// In the example above, a new `Survey` is created using the `new` method. Multiple responses are then added to the survey using the `add_multiple_responses` method, with each response including a respondent ID and a score from 0 to 10. After adding the responses, the `score` method is called to calculate the Net Promoter Score (NPS) of the survey, which is then printed to the console.
    ///
    /// If any of the responses have an invalid rating, a `Vec<NetPromoterScoreError>` is returned, which can be handled to identify and handle the specific errors.
    pub fn add_multiple_responses(
        &mut self,
        responses: impl IntoIterator<Item = (T, NpsRating)>,
    ) -> Result<(), Vec<NetPromoterScoreError>> {
        let errors: Vec<NetPromoterScoreError> = responses
            .into_iter()
            .filter_map(
                |(respondent_id, score)| match self.add_response(respondent_id, score) {
                    Ok(_) => None,
                    Err(e) => Some(e),
                },
            )
            .collect();
        if errors.is_empty() {
            self.calculate_nps();
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns a slice of the survey responses.
    pub fn responses(&self) -> impl Iterator<Item = &SurveyResponse<T>> {
        self.responses.values()
    }

    /// Returns a vector of survey responses matching the specified `Classification`.
    ///
    /// The `segment` method filters the survey responses based on the provided `Classification`
    /// parameter and returns a vector containing references to `SurveyResponse` items that match
    /// the classification. The classification can be one of `Detractor`, `Passive`, or `Promoter`.
    ///
    /// # Arguments
    ///
    /// * `classification` - A `Classification` enumeration value representing the desired segment
    /// (either `Detractor`, `Passive`, or `Promoter`) to filter the survey responses.
    ///
    /// # Example
    ///
    /// ```
    /// use net_promoter_score::prelude::*;
    ///
    /// let mut survey = Survey::new();
    /// // Add survey responses to the survey here...
    ///  let responses = vec![
    ///     ("r1", 10),
    ///     ("r2", 9),
    ///     ("r3", 9),
    ///     ("r4", 8),
    ///     ("r5", 7),
    ///     ("r6", 6),
    ///     ("r7", 5),
    ///     ("r8", 4),
    ///     ("r9", 3),
    ///     ("r10", 2),
    ///     ("r11", 1),
    ///     ("r12", 1),
    /// ];

    /// for (respondent_id, score) in responses {
    ///     survey.add_response(respondent_id, score).unwrap();
    /// }

    ///
    /// let detractors: Vec<&SurveyResponse<_>> = survey.segment(Classification::Detractor);
    /// let passives: Vec<&SurveyResponse<_>> = survey.segment(Classification::Passive);
    /// let promoters: Vec<&SurveyResponse<_>> = survey.segment(Classification::Promoter);
    ///
    /// println!("Detractors: {}", detractors.len());
    /// println!("Passives: {}", passives.len());
    /// println!("Promoters: {}", promoters.len());
    /// ```
    ///
    /// In the above example, the `segment` method is used to filter the survey responses based
    /// on the provided classification (`Detractor`, `Passive`, and `Promoter`). The filtered
    /// responses are collected into separate vectors, which are then printed to the console showing
    /// the number of responses in each classification.
    pub fn segment(&self, classification: Classification) -> Vec<&SurveyResponse<T>> {
        self.responses()
            .filter(|response| Classification::from(response.score()) == classification)
            .collect()
    }
    /// Returns the Net Promoter Score (NPS) of the survey.
    ///
    /// The NPS is a metric used to gauge the loyalty of customers or clients. It is calculated
    /// as the percentage of Promoters minus the percentage of Detractors. Promoters are respondents
    /// who gave a rating of 9 or 10, Detractors are respondents who gave a rating of 0 to 6, and
    /// Passives are respondents who gave a rating of 7 or 8.
    ///
    /// A higher NPS indicates a greater proportion of satisfied customers who are likely to recommend
    /// the product, service, or experience, whereas a lower NPS indicates dissatisfaction and a higher
    /// risk of customers switching to competitors. The NPS can range from -100 (all Detractors) to
    /// 100 (all Promoters).
    ///
    /// # Example
    ///
    /// ```
    /// use net_promoter_score::prelude::*;
    ///
    /// // Create a new survey and add responses
    /// let mut survey: Survey<u8> = Survey::new();
    /// // Add survey responses to the survey here...
    ///
    /// // Calculate the Net Promoter Score
    /// let nps = survey.score();
    /// println!("The calculated NPS is: {}", nps);
    /// ```
    ///
    /// In the above example, a new `Survey` is created, and survey responses are added to it.
    /// The `score` method is then called to calculate the Net Promoter Score (NPS) based on the
    /// given responses. The calculated NPS, which can range from -100 to 100, is then printed to the console.
    pub fn score(&mut self) -> i32 {
        if let Some(cached_nps) = self.nps_cache {
            cached_nps
        } else {
            self.calculate_nps();
            self.nps_cache.unwrap_or(0)
        }
    }
}
/// A specialized implementation of the [`Survey`] struct for respondent IDs of type i32.
///
/// This implementation provides an additional method, [add_bulk_responses_auto_id](crate),
/// which adds bulk responses with auto-generated respondent IDs.
///
/// # Example
///
/// ```
/// use net_promoter_score::prelude::*;
/// use anyhow::Result;
///
/// fn main() -> Result<(), Vec<NetPromoterScoreError>> {
///     let rating_quantities: &[(NpsRating, ScoreCount)] = &[
///         (1, 2),
///         (4, 1),
///         (5, 2),
///         (7, 8),
///         (8, 10),
///         (10, 10),
///     ];
///     let mut survey = Survey::new();
///     survey.add_bulk_responses_auto_id(&rating_quantities)?;
///
///     let nps = survey.score();
///     println!("The calculated NPS is: {}", nps);
///
///     Ok(())
/// }
/// ```
impl Survey<i32> {
    /// Adds survey responses with their quantities to the survey with auto-generated unique
    /// respondent IDs of type `i32`, starting at 1.
    ///
    /// This method accepts a slice of tuples, where the first element is the rating (from 0 to 10)
    /// of type `NpsRating`, and the second element is its quantity of type `ScoreCount`. It repeats
    /// the insertion of a response with the given rating `quantity` number of times.
    ///
    /// # Examples
    ///
    /// ```
    /// # use net_promoter_score::prelude::*;
    /// # use anyhow::Result;
    /// fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    ///     let rating_quantities: &[(NpsRating, ScoreCount)] = &[
    ///         (1, 2),
    ///         (4, 1),
    ///         (5, 2),
    ///         (7, 8),
    ///         (8, 10),
    ///         (10, 10),
    ///     ];
    ///
    ///     let mut survey = Survey::new();
    ///     survey.add_bulk_responses_auto_id(rating_quantities)?;
    ///
    ///     let nps = survey.score();
    ///     println!("The calculated NPS is: {}", nps);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a vector of `NetPromoterScoreError` if any of the responses could not be added to the survey.
    pub fn add_bulk_responses_auto_id(
        &mut self,
        nps_scores: &[(NpsRating, ScoreCount)],
    ) -> Result<(), Vec<NetPromoterScoreError>> {
        let mut respondent_id = 1i32;
        let respondent_id_fn = || {
            let current_id = respondent_id;
            respondent_id += 1;
            current_id
        };

        self.add_bulk_responses(respondent_id_fn, nps_scores)
    }
}
// Default trait implementation to create a new empty survey
impl<T> Default for Survey<T> {
    fn default() -> Self {
        Self {
            responses: BTreeMap::new(),
            nps_cache: Default::default(),
        }
    }
}

// Implementing IntoIterator for the Survey type, to allow iterating through survey responses.
impl<'a, T> IntoIterator for &'a Survey<T> {
    type Item = &'a SurveyResponse<T>;
    type IntoIter = std::collections::btree_map::Values<'a, T, SurveyResponse<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.responses.values()
    }
}

impl<T> IntoIterator for Survey<T> {
    type Item = (T, SurveyResponse<T>);
    type IntoIter = std::collections::btree_map::IntoIter<T, SurveyResponse<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.responses.into_iter()
    }
}
// Implementing the FromIterator trait for the Survey type.
// This allows constructing a Survey from a list of valid SurveyResponse items.
impl<T: Ord + Clone> FromIterator<Result<(T, SurveyResponse<T>), NetPromoterScoreError>>
    for Survey<T>
{
    fn from_iter<I: IntoIterator<Item = Result<(T, SurveyResponse<T>), NetPromoterScoreError>>>(
        iter: I,
    ) -> Self {
        let iterator = iter.into_iter();
        let mut survey = Survey {
            responses: BTreeMap::new(),
            nps_cache: Default::default(),
        };
        survey.extend(iterator.filter_map(Result::ok));
        survey
    }
}
// Implementing the Extend trait for the Survey type.
// This allows extending a survey with additional valid SurveyResponses.
//---------------------------------------------------------------------------
impl<T: Clone + Ord> Extend<(T, SurveyResponse<T>)> for Survey<T> {
    fn extend<I: IntoIterator<Item = (T, SurveyResponse<T>)>>(&mut self, iter: I) {
        self.responses.extend(iter);
    }
}

/// A single survey response, including the respondent ID of type `T` and the score of type `Rating`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurveyResponse<T> {
    respondent_id: T,
    score: Rating,
}

impl<T: PartialEq> SurveyResponse<T> {
    /// Creates a new survey response with the given respondent ID and score.
    pub fn new(respondent_id: T, rating: NpsRating) -> Result<Self, NetPromoterScoreError> {
        let nps_rating = Rating::try_from(rating)?;
        Ok(Self {
            respondent_id,
            score: nps_rating,
        })
    }

    /// Returns the respondent ID of the survey response.
    pub fn respondent_id(&self) -> &T {
        &self.respondent_id
    }

    /// Returns the score of the survey response.
    pub fn score(&self) -> &Rating {
        &self.score
    }
}

// Implementing Display for Rating to allow printing the rating value.
impl Display for Rating {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Classification of survey respondents, based on their score, into Detractor, Passive, and Promoter.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Detractor,
    Passive,
    Promoter,
}

/// The `Rating` represents a valid survey response score in the range of 0 to 10.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rating(u8);

// Conversion from a Rating to a Classification.
impl From<&Rating> for Classification {
    fn from(score: &Rating) -> Self {
        match score.0 {
            0..=6 => Classification::Detractor,
            7 | 8 => Classification::Passive,
            9 | 10 => Classification::Promoter,
            _ => unreachable!(),
        }
    }
}

impl From<Rating> for Classification {
    fn from(score: Rating) -> Self {
        match score.0 {
            0..=6 => Classification::Detractor,
            7 | 8 => Classification::Passive,
            9 | 10 => Classification::Promoter,
            _ => unreachable!(),
        }
    }
}

// Implementing the TryFrom trait for Rating, to allow conversion from a u8.
// This ensures that only valid rating values (0 to 10) can be converted to a Rating.
impl TryFrom<u8> for Rating {
    type Error = NetPromoterScoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 10 {
            Ok(Rating(value))
        } else {
            Err(NetPromoterScoreError::InvalidRating(value))
        }
    }
}

// Implementing the Deref trait for Rating, allowing users to access the inner u8 value.
impl Deref for Rating {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Errors that may occur when working with the Net Promoter Score.
#[derive(Debug, PartialEq, Eq)]
pub enum NetPromoterScoreError {
    InvalidRating(u8),
}

// Implementing the Error trait for NetPromoterScoreError.
impl std::error::Error for NetPromoterScoreError {}

// Implementing the Display trait for NetPromoterScoreError, to provide readable error messages.
impl std::fmt::Display for NetPromoterScoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetPromoterScoreError::InvalidRating(value) => {
                write!(f, "Invalid rating value: {}", value)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Error;
    use std::convert::TryInto;

    #[test]
    fn test_nps_calculation() {
        let mut survey = Survey::new();
        survey
            .add_multiple_responses(vec![(1, 10), (2, 9), (3, 9), (4, 8), (5, 7), (6, 6)])
            .unwrap();

        let nps = survey.score();
        assert_eq!(nps, 34);
    }
    #[test]
    fn test_create_survey_with_add_multiple_responses() {
        let mut survey = Survey::new();
        assert_eq!((&survey).into_iter().count(), 0);

        survey
            .add_multiple_responses(vec![(1, 6), (2, 8), (3, 10)])
            .unwrap();

        assert_eq!((&survey).into_iter().count(), 3);
    }
    // ... (rest of the tests)
    #[test]
    fn test_create_survey_from_iterator() -> Result<(), Error> {
        let response_results = vec![
            SurveyResponse::new(1, 6).map(|resp| (1, resp)),
            SurveyResponse::new(2, 8).map(|resp| (2, resp)),
            SurveyResponse::new(3, 10).map(|resp| (3, resp)),
        ];

        let survey: Survey<u32> = response_results.into_iter().collect();
        let respondent_ids: Vec<u32> = survey
            .into_iter()
            .map(|(respondent_id, _response)| respondent_id)
            .collect();
        assert_eq!(respondent_ids, vec![1, 2, 3]);
        Ok(())
    } // Add a test for the new `SurveyResponse` struct.

    #[test]
    fn test_create_survey() -> Result<(), Error> {
        let mut survey = Survey::new();
        assert_eq!((&survey).into_iter().count(), 0);

        survey.add_response(1, 6)?;
        assert_eq!((&survey).into_iter().count(), 1);

        let first_response = (&survey).into_iter().next().unwrap();
        assert_eq!(*first_response.respondent_id(), 1);
        Ok(())
    }

    #[test]
    fn test_survey_response() -> Result<(), Error> {
        let response = SurveyResponse::new(1, 7)?;
        assert_eq!(*response.respondent_id(), 1);
        assert_eq!(*response.score(), Rating(7));
        Ok(())
    }
    #[test]
    fn test_valid_score_conversion() -> Result<(), Error> {
        let nps_score: Rating = 5u8.try_into()?;
        assert_eq!(*nps_score, 5);
        Ok(())
    }

    #[test]
    fn test_invalid_score_conversion() {
        let nps_score: Result<Rating, NetPromoterScoreError> = 11u8.try_into();
        assert_eq!(
            nps_score.unwrap_err(),
            NetPromoterScoreError::InvalidRating(11)
        );
    }
    #[test]
    fn test_classification() -> Result<(), Error> {
        let detractor: Rating = 6u8.try_into()?;
        let passive: Rating = 8u8.try_into()?;
        let promoter: Rating = 10u8.try_into()?;

        let detractor_classification: Classification = detractor.into();
        let passive_classification: Classification = passive.into();
        let promoter_classification: Classification = promoter.into();

        assert_eq!(detractor_classification, Classification::Detractor);
        assert_eq!(passive_classification, Classification::Passive);
        assert_eq!(promoter_classification, Classification::Promoter);
        Ok(())
    }
    #[test]
    fn test_create_survey_from_responses() {
        let mut survey_result: Result<Survey<u32>, Vec<Error>> =
            Survey::<u32>::from_responses(vec![(1, 10), (2, 9), (3, 9), (4, 8), (5, 7), (6, 6)]);

        match survey_result {
            Ok(ref mut survey) => {
                assert_eq!(survey.responses.values().count(), 6);
                assert_eq!(survey.score(), 34);
            }
            Err(errors) => panic!("Unexpected errors while parsing responses: {:?}", errors),
        }
    }
    #[test]
    fn test_scores_comparison() {
        let score1 = Rating(5);
        let score2 = Rating(7);
        let score3 = Rating(10);

        assert!(score1 < score2);
        assert!(score2 > score1);
        assert!(score3 == 10.try_into().unwrap());
    }
    #[test]
    fn test_classification_order() {
        assert!(Classification::Detractor < Classification::Passive);
        assert!(Classification::Passive < Classification::Promoter);
    }

    #[test]
    fn test_survey_response_order() -> Result<(), Error> {
        let response1 = SurveyResponse::<_>::new(1, 5)?;
        let response2 = SurveyResponse::<_>::new(1, 7)?;
        let response3 = SurveyResponse::<_>::new(2, 7)?;

        assert!(response1 < response2);
        assert!(response1 < response3);
        assert!(response2 < response3);
        Ok(())
    }

    #[test]
    fn test_sort_survey_responses() -> Result<(), Error> {
        let mut responses = vec![
            SurveyResponse::<_>::new(1, 7)?,
            SurveyResponse::<_>::new(2, 5)?,
            SurveyResponse::<_>::new(3, 10)?,
            SurveyResponse::<_>::new(1, 5)?,
        ];

        responses.sort();

        let expected = vec![
            SurveyResponse::<_>::new(1, 5)?,
            SurveyResponse::<_>::new(1, 7)?,
            SurveyResponse::<_>::new(2, 5)?,
            SurveyResponse::<_>::new(3, 10)?,
        ];

        assert_eq!(responses, expected);
        Ok(())
    }
    #[test]
    fn test_add_bulk_responses_strings() {
        let rating_quantities = [
            (1, 2),
            (4, 1),
            (5, 1),
            (5, 1),
            (7, 8),
            (8, 10),
            (10, 5),
            (10, 5),
        ];

        let mut survey: Survey<String> = Survey::new();

        let mut respondent_id_number = 1;

        let respondent_id_fn = || {
            let current_id = format!("customer_{}", respondent_id_number);
            respondent_id_number += 1;
            current_id
        };

        survey
            .add_bulk_responses(respondent_id_fn, &rating_quantities)
            .unwrap();

        // Verify the score based on the provided rating quantities.
        let nps = survey.score();
        assert_eq!(nps, 15);
    }

    #[test]
    fn test_add_bulk_responses() {
        let rating_quantities = [
            (1, 2),
            (4, 1),
            (5, 1),
            (5, 1),
            (7, 8),
            (8, 10),
            (10, 5),
            (10, 5),
        ];

        let mut survey = Survey::new();

        let mut respondent_id = 1;
        let respondent_id_fn = || {
            let current_id = respondent_id;
            respondent_id += 1;
            current_id
        };

        survey
            .add_bulk_responses(respondent_id_fn, &rating_quantities)
            .unwrap();

        // Verify the score based on the provided rating quantities.
        let nps = survey.score();
        assert_eq!(nps, 15);
    }
    #[test]
    fn test_add_bulk_responses_auto_id() -> Result<(), Vec<NetPromoterScoreError>> {
        let rating_quantities = [(1, 2), (4, 1), (5, 2), (7, 8), (8, 10), (10, 10)];

        let mut survey = Survey::new();
        survey.add_bulk_responses_auto_id(&rating_quantities)?;

        let nps = survey.score();
        assert_eq!(nps, 15);

        Ok(())
    }

    #[test]
    fn test_responses_segmentation() -> Result<(), NetPromoterScoreError> {
        // Create a new survey and add responses
        let mut survey = Survey::new();
        survey.add_response(1, 9)?;
        survey.add_response(2, 8)?;
        survey.add_response(4, 6)?;
        survey.add_response(3, 6)?;

        // Use the segment method to directly get detractors, passives, and promoters.
        let detractors = survey.segment(Classification::Detractor);
        let passives = survey.segment(Classification::Passive);
        let promoters = survey.segment(Classification::Promoter);

        // Verify the counts of each segment
        assert_eq!(detractors.len(), 2);
        assert_eq!(passives.len(), 1);
        assert_eq!(promoters.len(), 1);

        let mut survey_alt = Survey::new();
        survey_alt.add_response("some respondent id", 9)?;

        // Use the segment method to directly get detractors, passives, and promoters.
        let detractors = survey_alt.segment(Classification::Detractor);
        let passives = survey_alt.segment(Classification::Passive);
        let promoters = survey_alt.segment(Classification::Promoter);

        assert_eq!(detractors.len(), 0);
        assert_eq!(passives.len(), 0);
        assert_eq!(promoters.len(), 1);

        // Assert that the ID of a single response is the one we put in
        assert_eq!(promoters[0].respondent_id(), &"some respondent id");
        Ok(())
    }

    #[test]
    fn test_survey_classification_segments() {
        // Create a survey with multiple responses
        let mut survey = Survey::new();
        let responses = vec![
            (1, 10),
            (2, 9),
            (3, 9),
            (4, 8),
            (5, 7),
            (6, 6),
            (7, 5),
            (8, 4),
            (9, 3),
            (10, 2),
            (11, 1),
            (12, 1),
        ];

        for (respondent_id, score) in responses {
            survey.add_response(respondent_id, score).unwrap();
        }

        // Get the response segments for each classification
        let detractors = survey.segment(Classification::Detractor);
        let passives = survey.segment(Classification::Passive);
        let promoters = survey.segment(Classification::Promoter);

        // Expected response sets for each classification
        let expected_detractors = vec![6, 5, 4, 3, 2, 1, 1];
        let expected_passives = vec![8, 7];
        let expected_promoters = vec![10, 9, 9];

        // Check the actual response scores for each segment
        let detractor_scores: Vec<u8> = detractors
            .iter()
            .map(|response| **response.score())
            .collect();
        let passive_scores: Vec<u8> = passives.iter().map(|response| **response.score()).collect();
        let promoter_scores: Vec<u8> = promoters
            .iter()
            .map(|response| **response.score())
            .collect();

        // Verify the segments match their expected response sets
        assert_eq!(
            detractor_scores, expected_detractors,
            "Detractors didn't match"
        );
        assert_eq!(passive_scores, expected_passives, "Passives didn't match");
        assert_eq!(
            promoter_scores, expected_promoters,
            "Promoters didn't match"
        );
    }
    #[test]
    fn test_survey_classification_segments_ids() {
        // Create a survey with multiple responses
        let mut survey = Survey::new();
        let responses = vec![
            ("r1", 10),
            ("r2", 9),
            ("r3", 9),
            ("r4", 8),
            ("r5", 7),
            ("r6", 6),
            ("r7", 5),
            ("r8", 4),
            ("r9", 3),
            ("r10", 2),
            ("r11", 1),
            ("r12", 1),
        ];

        for (respondent_id, score) in responses {
            survey.add_response(respondent_id, score).unwrap();
        }

        // Get the response segments for each classification
        let detractors = survey.segment(Classification::Detractor);
        let passives = survey.segment(Classification::Passive);
        let promoters = survey.segment(Classification::Promoter);

        // Expected respondent ID sets for each classification
        let mut expected_detractors = vec!["r6", "r7", "r8", "r9", "r10", "r11", "r12"];
        let mut expected_passives = vec!["r4", "r5"];
        let mut expected_promoters = vec!["r1", "r2", "r3"];

        expected_detractors.sort();
        expected_passives.sort();
        expected_promoters.sort();

        // Check the actual respondent IDs for each segment
        let mut detractor_ids: Vec<&str> = detractors
            .iter()
            .map(|response| *response.respondent_id())
            .collect();
        let mut passive_ids: Vec<&str> = passives
            .iter()
            .map(|response| *response.respondent_id())
            .collect();
        let mut promoter_ids: Vec<&str> = promoters
            .iter()
            .map(|response| *response.respondent_id())
            .collect();
        //
        // Sort the respondent_ids before asserting equality
        detractor_ids.sort();
        passive_ids.sort();
        promoter_ids.sort();
        // Verify the segments match their expected respondent ID sets
        assert_eq!(
            detractor_ids, expected_detractors,
            "Detractors' respondent_ids didn't match"
        );
        assert_eq!(
            passive_ids, expected_passives,
            "Passives' respondent_ids didn't match"
        );
        assert_eq!(
            promoter_ids, expected_promoters,
            "Promoters' respondent_ids didn't match"
        );
    }
}
