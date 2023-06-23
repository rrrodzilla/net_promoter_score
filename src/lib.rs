pub mod prelude; // Add this line to include the prelude module
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::iter::{Extend, FromIterator};
use std::ops::Deref;

/// A `Survey` represents a collection of survey responses, where each response
/// includes a respondent's ID of type `T` and a score in the range of 0 to 10.
///
/// The primary purpose of the [net_promoter_score] crate is to calculate the Net
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
///     survey.add_response(1, 9)?;
///     survey.add_response(2, 8)?;
///     survey.add_response(3, 6)?;
///
///     let nps = survey.score();
///     println!("The calculated NPS is: {}", nps);
///
///     Ok(())
/// }
/// ```
pub struct Survey<T> {
    responses: Vec<SurveyResponse<T>>,
}

impl<T> Survey<T> {
    /// Creates a new empty survey.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds survey responses with their quantities to the survey.
    ///
    /// This method accepts a mutable respondent ID generator function and a slice of tuples,
    /// where the first element is the rating (from 0 to 10) and the second element is its quantity.
    /// It repeats the insertion of a response with the given rating `quantity` number of times.
    ///
    /// # Examples
    ///
    /// ```
    /// use net_promoter_score::prelude::*;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    ///     let rating_quantities = [
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
    ///     survey.add_bulk_responses(respondent_id_fn, &rating_quantities)?;
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
        quantities: &[(u8, usize)],
    ) -> Result<(), Vec<NetPromoterScoreError>>
    where
        F: FnMut() -> T,
    {
        let errors: Vec<NetPromoterScoreError> = quantities
            .iter()
            .filter(|&&(score, _)| score <= 10)
            .flat_map(|&(score, quantity)| {
                let responses: Vec<(T, u8)> =
                    (0..quantity).map(|_| (respondent_id_fn(), score)).collect();
                self.add_multiple_responses(responses)
                    .err()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .collect();

        if errors.is_empty() {
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
    pub fn from_responses<E>(responses: impl IntoIterator<Item = (T, u8)>) -> Result<Self, Vec<E>>
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
    pub fn add_response(
        &mut self,
        respondent_id: T,
        score: u8,
    ) -> Result<(), NetPromoterScoreError> {
        let response = SurveyResponse::new(respondent_id, score)?;
        self.responses.push(response);
        Ok(())
    }

    /// Adds multiple responses to the survey.
    ///
    /// If any of the responses have an invalid rating, a `Vec<NetPromoterScoreError>` is returned.
    pub fn add_multiple_responses(
        &mut self,
        responses: impl IntoIterator<Item = (T, u8)>,
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
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns a slice of the survey responses.
    pub fn responses(&self) -> &[SurveyResponse<T>] {
        &self.responses
    }

    /// Returns the Net Promoter Score (NPS) of the survey.
    ///
    /// The NPS is calculated as the percentage of promoters minus the percentage of detractors.
    ///
    /// # Example
    ///
    /// ```
    /// # use net_promoter_score::prelude::*;
    /// # let mut survey: Survey<u8> = Survey::new();
    /// let nps = survey.score();
    /// ```
    pub fn score(&self) -> i32 {
        let total_responses = self.responses.len() as i32;
        if total_responses == 0 {
            return 0;
        }

        let promoters = self
            .responses
            .iter()
            .filter(|response| Classification::from(response.score()) == Classification::Promoter)
            .count() as i32;

        let detractors = self
            .responses
            .iter()
            .filter(|response| Classification::from(response.score()) == Classification::Detractor)
            .count() as i32;

        let promoter_percent = 100 * promoters / total_responses;
        let detractor_percent = 100 * detractors / total_responses;

        promoter_percent - detractor_percent
    }
}
/// A specialized implementation of the [`Survey`] struct for respondent IDs of type i32.
///
/// This implementation provides an additional method, [`add_bulk_responses_auto_id`],
/// which adds bulk responses with auto-generated respondent IDs.
///
/// # Example
///
/// ```
/// use net_promoter_score::prelude::*;
/// use anyhow::Result;
///
/// fn main() -> Result<(), Vec<NetPromoterScoreError>> {
///     let rating_quantities = [
///         (1, 2),
///         (4, 1),
///         (5, 2),
///         (7, 8),
///         (8, 10),
///         (10, 10),
///     ];
///
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
    /// This method accepts a slice of tuples, where the first element is the rating (from 0 to 10),
    /// and the second element is its quantity. It repeats the insertion of a response with the
    /// given rating `quantity` number of times.
    ///
    /// # Examples
    ///
    /// ```
    /// use net_promoter_score::prelude::*;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<(), Vec<NetPromoterScoreError>> {
    ///     let rating_quantities = [
    ///         (1, 2),
    ///         (4, 1),
    ///         (5, 2),
    ///         (7, 8),
    ///         (8, 10),
    ///         (10, 10),
    ///     ];
    ///
    ///     let mut survey = Survey::new();
    ///     survey.add_bulk_responses_auto_id(&rating_quantities)?;
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
        quantities: &[(u8, usize)],
    ) -> Result<(), Vec<NetPromoterScoreError>> {
        let mut respondent_id = 1i32;
        let respondent_id_fn = || {
            let current_id = respondent_id;
            respondent_id += 1;
            current_id
        };

        self.add_bulk_responses(respondent_id_fn, quantities)
    }
}
// Default trait implementation to create a new empty survey
impl<T> Default for Survey<T> {
    fn default() -> Self {
        Self {
            responses: Vec::new(),
        }
    }
}

// Implementing IntoIterator for the Survey type, to allow iterating through survey responses.
impl<'a, T> IntoIterator for &'a Survey<T> {
    type Item = &'a SurveyResponse<T>;
    type IntoIter = std::slice::Iter<'a, SurveyResponse<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.responses.iter()
    }
}

impl<T> IntoIterator for Survey<T> {
    type Item = SurveyResponse<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.responses.into_iter()
    }
}

// Implementing the FromIterator trait for the Survey type.
// This allows constructing a Survey from a list of valid SurveyResponse items.
impl<T> FromIterator<Result<SurveyResponse<T>, NetPromoterScoreError>> for Survey<T> {
    fn from_iter<I: IntoIterator<Item = Result<SurveyResponse<T>, NetPromoterScoreError>>>(
        iter: I,
    ) -> Self {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        let mut survey = Survey {
            responses: Vec::with_capacity(lower_bound),
        };
        survey.extend(iterator);
        survey
    }
}

// Implementing the Extend trait for the Survey type.
// This allows extending a survey with additional valid SurveyResponses.
//---------------------------------------------------------------------------
impl<T> Extend<Result<SurveyResponse<T>, NetPromoterScoreError>> for Survey<T> {
    fn extend<I: IntoIterator<Item = Result<SurveyResponse<T>, NetPromoterScoreError>>>(
        &mut self,
        iter: I,
    ) {
        self.responses
            .extend(iter.into_iter().filter_map(Result::ok));
    }
}

/// A single survey response, including the respondent ID of type `T` and the score of type `Rating`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurveyResponse<T> {
    respondent_id: T,
    score: Rating,
}

impl<T> SurveyResponse<T> {
    /// Creates a new survey response with the given respondent ID and score.
    pub fn new(respondent_id: T, score: u8) -> Result<Self, NetPromoterScoreError> {
        let nps_score = Rating::try_from(score)?;
        Ok(Self {
            respondent_id,
            score: nps_score,
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
        let responses = vec![
            SurveyResponse::new(1, 6),
            SurveyResponse::new(2, 8),
            SurveyResponse::new(3, 10),
        ];

        let survey: Survey<u32> = responses.into_iter().collect();
        let respondent_ids: Vec<u32> = survey
            .into_iter()
            .map(|response| *response.respondent_id())
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
        let survey_result: Result<Survey<u32>, Vec<Error>> =
            Survey::<u32>::from_responses(vec![(1, 10), (2, 9), (3, 9), (4, 8), (5, 7), (6, 6)]);

        match survey_result {
            Ok(ref survey) => {
                assert_eq!((&survey).into_iter().count(), 6);
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
}
