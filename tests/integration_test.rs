use net_promoter_score::prelude::*;
use anyhow::Result;

#[test]
fn test_integration() -> Result<()> {

    let mut survey = Survey::new();

    survey.add_response(1, 9)?;
    survey.add_response(2, 8)?;
    survey.add_response(3, 6)?;

    assert_eq!(0, survey.score());

    Ok(())
}

#[test]
fn test_integration_err() -> Result<()> {

    let mut survey = Survey::new();

    survey.add_response(1, 9)?;
    survey.add_response(2, 8)?;
    let res = survey.add_response(3, 16);

    assert!(res.is_err());

    Ok(())
}
