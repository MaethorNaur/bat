use serde::{Deserialize, Serialize};
use std::fmt;

const NEW_LINE: &str = "\n";

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Feature {
    name: String,
    scenarios: Vec<Scenario>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Scenario {
    name: String,
    scenario_type: Keyword,
    steps: Vec<StepType>,
    examples: Option<Examples>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Examples {
    fields: Vec<String>,
    values: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum StepType {
    Step(Step),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Step {
    keyword: StepKeyword,
    text: String,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum StepKeyword {
    Given,
    When,
    Then,
    And,
    But,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Keyword {
    Feature,
    Example,
    Examples,
    Scenario,
    Background,
    ScenarioOutline,
}

impl Feature {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn scenario(&mut self, scenario: Scenario) -> &Self {
        self.scenarios.push(scenario);
        self
    }
}

impl Scenario {
    pub fn new(name: String) -> Self {
        Self {
            name,
            scenario_type: Keyword::Scenario,
            steps: vec![],
            examples: None,
        }
    }

    pub fn examples(&self, examples: Examples) -> Self {
        Self {
            name: self.name.clone(),
            scenario_type: Keyword::ScenarioOutline,
            examples: Some(examples),
            steps: self.steps.clone(),
        }
    }

    pub fn step(&mut self, step: StepType) -> &Self {
        self.steps.push(step);
        self
    }
}
impl Examples {
    pub fn new(fields: Vec<String>) -> Self {
        Self {
            fields,
            values: vec![],
        }
    }
    pub fn values(&mut self, values: Vec<String>) -> &Self {
        self.values.push(values);
        self
    }
}
impl StepType {
    pub fn step(keyword: StepKeyword, text: String) -> Self {
        StepType::Step(Step { keyword, text })
    }
}
impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Keyword::Feature.fmt(f)?;
        f.write_str(&self.name)?;
        for scenario in self.scenarios.iter() {
            f.write_str(NEW_LINE)?;
            f.write_str(NEW_LINE)?;
            write!(f, "{:ident$}{}", "", scenario, ident = 2)?;
        }
        Ok(())
    }
}

impl fmt::Display for Scenario {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.scenario_type.fmt(f)?;
        f.write_str(&self.name)?;
        for step in self.steps.iter() {
            let StepType::Step(Step { keyword, .. }) = step;
            match keyword {
                StepKeyword::Given | StepKeyword::When | StepKeyword::Then => {
                    f.write_str(NEW_LINE)?
                }
                _ => (),
            };
            f.write_str(NEW_LINE)?;
            write!(f, "{:ident$}{}", "", step, ident = 4)?;
        }
        if let Some(examples) = &self.examples {
            write!(f, "{:ident$}{}", "", examples, ident = 4)?;
        }
        Ok(())
    }
}
impl fmt::Display for Examples {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(NEW_LINE)
            .and(write!(f, "{:ident$}{}", "", Keyword::Examples, ident = 4))
            .and(f.write_str(NEW_LINE))
            .and(write!(f, "{:ident$}|", "", ident = 6))
            .and(f.write_str(&self.fields.join("|")))
            .and(f.write_str("|"))
            .and(f.write_str(NEW_LINE))?;
        self.values.iter().for_each(|values| {
            write!(f, "{:ident$}|", "", ident = 6)
                .and(f.write_str(&values.join("|")))
                .and(f.write_str("|"))
                .and(f.write_str(NEW_LINE))
                .unwrap();
        });
        Ok(())
    }
}
impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let keyword = match self {
            Keyword::Feature => "Feature: ",
            Keyword::Scenario => "Scenario: ",
            Keyword::ScenarioOutline => "Scenario outline: ",
            Keyword::Example => "Example: ",
            Keyword::Examples => "Examples: ",
            Keyword::Background => "Background: ",
        };
        f.write_str(keyword)
    }
}

impl fmt::Display for StepKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let keyword = match self {
            StepKeyword::Given => "Given ",
            StepKeyword::When => "When ",
            StepKeyword::Then => "Then ",
            StepKeyword::And => "And ",
            StepKeyword::But => "But ",
        };
        f.write_str(keyword)
    }
}

impl fmt::Display for StepType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StepType::Step(step) => step.fmt(f),
        }
    }
}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.keyword.fmt(f).and(f.write_str(&self.text))
    }
}
