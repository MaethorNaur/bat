use openapi::v3_0::*;
use plugins::gherkin::*;
use slugify::slugify;
use std::collections::BTreeMap;

const BODY_RESPONSE: &str = "the body response is <body>";
const EMPTY_BODY_RESPONSE: &str = "the response does not have a body";

#[derive(Debug)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    TRACE,
    PATCH,
}
pub fn new(openapi: Spec) -> Feature {
    let mut feature = Feature::new(openapi.info.title);
    create_scenarios(&mut feature, openapi.paths);
    feature
}

fn create_scenarios(feature: &mut Feature, paths: BTreeMap<String, PathItem>) {
    paths
        .iter()
        .flat_map(|(path, item)| create_scenario(path, item))
        .for_each(|scenario| {
            feature.scenario(scenario);
        })
}

fn create_scenario(path: &str, item: &PathItem) -> Vec<Scenario> {
    let mut scenarios = Vec::new();
    if let Some(operation) = &item.get {
        create_scenario_from_operation(path, Method::GET, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }

    if let Some(operation) = &item.delete {
        create_scenario_from_operation(path, Method::DELETE, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }
    if let Some(operation) = &item.post {
        create_scenario_from_operation(path, Method::POST, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }
    if let Some(operation) = &item.put {
        create_scenario_from_operation(path, Method::PUT, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }
    if let Some(operation) = &item.options {
        create_scenario_from_operation(path, Method::OPTIONS, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }
    if let Some(operation) = &item.patch {
        create_scenario_from_operation(path, Method::PATCH, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }
    if let Some(operation) = &item.trace {
        create_scenario_from_operation(path, Method::TRACE, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }
    if let Some(operation) = &item.head {
        create_scenario_from_operation(path, Method::HEAD, operation)
            .into_iter()
            .for_each(|s| scenarios.push(s))
    }
    scenarios
}
fn create_scenario_from_operation(
    path: &str,
    method: Method,
    operation: &Operation,
) -> Vec<Scenario> {
    let mut scenearios = Vec::new();
    let summary = &operation.summary;
    let description = &operation.description;
    let path_str = &path.to_owned();
    let name = summary
        .as_ref()
        .or_else(|| description.as_ref())
        .unwrap_or(path_str);
    for (code_str, response) in &operation.responses {
        let response_name = &response.description.as_ref().unwrap_or(code_str);
        let mut scenario = Scenario::new(format!(
            "{}\n{:ident$}For: {}",
            name,
            "",
            response_name,
            ident = 4
        ));

        scenario.step(create_given(path.to_string()));
        let parameters = get_params(operation.parameters.clone());
        create_params_given_and(&parameters).iter().for_each(|s| {
            scenario.step(s.clone());
        });
        scenario.step(create_when(&method));
        create_params_when_and(&parameters).iter().for_each(|s| {
            scenario.step(s.clone());
        });
        create_then(code_str, response).iter().for_each(|s| {
            scenario.step(s.clone());
        });

        let mut fields = vec![];
        let has_code = is_response(code_str);
        if has_code {
            fields.push("code".to_string())
        }
        parameters
            .iter()
            .for_each(|s| fields.push(slugify!(&s.name, separator = "_")));
        let has_body = response.content.is_some();
        if has_body {
            fields.push("body".to_string())
        }
        let mut examples = Examples::new(fields);
        let mut values: Vec<String> = vec![];
        if has_code {
            values.push(code_str.to_string())
        }
        parameters
            .iter()
            .for_each(|_| values.push("\t".to_string()));
        if has_body {
            values.push("\t".to_string())
        }
        examples.values(values);
        scenearios.push(scenario.examples(examples));
    }
    scenearios
}
fn create_given(path: String) -> StepType {
    let url = path.replace("{", "<").replace("}", ">");
    StepType::step(StepKeyword::Given, format!("a resource {}", url))
}
fn create_when(method: &Method) -> StepType {
    StepType::step(
        StepKeyword::When,
        format!("a {:?} request is made on this resource", method),
    )
}
fn is_response(code_str: &str) -> bool {
    let code = i16::from_str_radix(code_str, 10).unwrap_or(500);
    code >= 200 && code < 400
}
fn create_then(code_str: &str, response: &Response) -> Vec<StepType> {
    let mut steps = Vec::new();
    let has_code = is_response(code_str);
    let body = match response.content {
        Some(_) => BODY_RESPONSE,
        None => EMPTY_BODY_RESPONSE,
    };

    if has_code {
        steps.push(StepType::step(
            StepKeyword::Then,
            "the response code is <code>".to_string(),
        ));
        steps.push(StepType::step(StepKeyword::And, body.to_string()));
    } else {
        steps.push(StepType::step(StepKeyword::Then, body.to_string()));
    }
    steps
}

fn get_params(maybe_parameters: Option<Vec<ObjectOrReference<Parameter>>>) -> Vec<Parameter> {
    maybe_parameters.map_or(Vec::new(), |parameters| {
        parameters
            .into_iter()
            .map(|o| match o {
                ObjectOrReference::Object(parameter) => Some(parameter),
                _ => None,
            })
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect()
    })
}
fn create_params_given_and(parameters: &[Parameter]) -> Vec<StepType> {
    parameters
        .iter()
        .filter(|p| p.location != "path")
        .map(|p| {
            StepType::step(
                StepKeyword::And,
                format!(
                    "a parameter \"{name}\" in {location}",
                    name = &p.name,
                    location = p.location
                ),
            )
        })
        .collect()
}
fn create_params_when_and(parameters: &[Parameter]) -> Vec<StepType> {
    parameters
        .iter()
        .filter(|p| p.location != "path")
        .map(|p| {
            StepType::step(
                StepKeyword::And,
                format!(
                    "the value for \"{name}\" is <{slug_name}>",
                    name = &p.name,
                    slug_name = slugify!(&p.name, separator = "_"),
                ),
            )
        })
        .collect()
}
