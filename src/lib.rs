use anyhow::Error;
use anyhow::Result;
use std::convert::TryFrom;
use std::iter::{Extend, FromIterator};
use std::ops::Deref;

pub struct Survey<T> {
    responses: Vec<SurveyResponse<T>>,
}

impl<T> Survey<T> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn from_responses<E>(responses: impl IntoIterator<Item = (T, u8)>) -> Result<Self, Vec<E>>
    where
        E: From<Error>,
    {
        let mut survey = Self::new();
        match survey.add_responses(responses) {
            Ok(_) => Ok(survey),
            Err(errors) => Err(errors.into_iter().map(E::from).collect()),
        }
    }
    pub fn add_response(&mut self, respondent_id: T, score: u8) -> Result<()> {
        let response = SurveyResponse::new(respondent_id, score)?;
        self.responses.push(response);
        Ok(())
    }

    pub fn add_responses(
        &mut self,
        responses: impl IntoIterator<Item = (T, u8)>,
    ) -> Result<(), Vec<Error>> {
        let errors: Vec<Error> = responses
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
    pub fn responses(&self) -> &[SurveyResponse<T>] {
        &self.responses
    }

    pub fn nps(&self) -> i32 {
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

// Default trait implementation to create a new empty survey
impl<T> Default for Survey<T> {
    fn default() -> Self {
        Self {
            responses: Vec::new(),
        }
    }
}
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
// FromIterator trait implementation to construct a Survey from a list of responses
impl<T> FromIterator<Result<SurveyResponse<T>, Error>> for Survey<T> {
    fn from_iter<I: IntoIterator<Item = Result<SurveyResponse<T>, Error>>>(iter: I) -> Self {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        let mut survey = Survey {
            responses: Vec::with_capacity(lower_bound),
        };
        survey.extend(iterator);
        survey
    }
}
// Extend trait implementation to extend a Survey with an iterator providing results
impl<T> Extend<Result<SurveyResponse<T>, Error>> for Survey<T> {
    fn extend<I: IntoIterator<Item = Result<SurveyResponse<T>, Error>>>(&mut self, iter: I) {
        self.responses
            .extend(iter.into_iter().filter_map(Result::ok));
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurveyResponse<T> {
    respondent_id: T,
    score: Score,
}

impl<T> SurveyResponse<T> {
    pub fn new(respondent_id: T, score: u8) -> Result<Self> {
        let nps_score = Score::try_from(score)?;
        Ok(Self {
            respondent_id,
            score: nps_score,
        })
    }
    pub fn respondent_id(&self) -> &T {
        &self.respondent_id
    }

    pub fn score(&self) -> &Score {
        &self.score
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Detractor,
    Passive,
    Promoter,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(u8);
impl From<&Score> for Classification {
    fn from(score: &Score) -> Self {
        Self::from(score.0)
    }
}
impl From<Score> for Classification {
    fn from(score: Score) -> Self {
        From::from(score.0)
    }
}
impl From<&u8> for Classification {
    fn from(value: &u8) -> Self {
        match *value {
            0..=6 => Classification::Detractor,
            7 | 8 => Classification::Passive,
            9 | 10 => Classification::Promoter,
            _ => unreachable!(),
        }
    }
}
impl From<u8> for Classification {
    fn from(value: u8) -> Self {
        match value {
            0..=6 => Classification::Detractor,
            7 | 8 => Classification::Passive,
            9 | 10 => Classification::Promoter,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Score {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 10 {
            Ok(Score(value))
        } else {
            Err(anyhow::anyhow!("Invalid NPS score value: {}", value))
        }
    }
}

impl Deref for Score {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Classification, Score, Survey, SurveyResponse};
    use anyhow::Error;
    use std::convert::TryInto;

    #[test]
    fn test_nps_calculation() {
        let mut survey = Survey::new();
        survey
            .add_responses(vec![(1, 10), (2, 9), (3, 9), (4, 8), (5, 7), (6, 6)])
            .unwrap();

        let nps = survey.nps();
        assert_eq!(nps, 34);
    }
    #[test]
    fn test_create_survey_with_add_responses() {
        let mut survey = Survey::new();
        assert_eq!((&survey).into_iter().count(), 0);

        survey.add_responses(vec![(1, 6), (2, 8), (3, 10)]).unwrap();

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
        assert_eq!(*response.score(), Score(7));
        Ok(())
    }
    #[test]
    fn test_valid_score_conversion() -> Result<(), Error> {
        let nps_score: Score = 5u8.try_into()?;
        assert_eq!(*nps_score, 5);
        Ok(())
    }

    #[test]
    fn test_invalid_score_conversion() -> Result<(), Error> {
        let nps_score: Result<Score, Error> = 11u8.try_into();
        assert!(nps_score.is_err());
        Ok(())
    }
    #[test]
    fn test_classification() -> Result<(), Error> {
        let detractor: Score = 6u8.try_into()?;
        let passive: Score = 8u8.try_into()?;
        let promoter: Score = 10u8.try_into()?;

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
                assert_eq!(survey.nps(), 34);
            }
            Err(errors) => panic!("Unexpected errors while parsing responses: {:?}", errors),
        }
    }
    #[test]
    fn test_scores_comparison() {
        let score1 = Score(5);
        let score2 = Score(7);
        let score3 = Score(10);

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
}
