mod puan_eval;
mod puan_core;

use puan_eval::evaluation_service_server::{EvaluationService, EvaluationServiceServer};
use puan_eval::{EvaluationRequest, EvaluationResponse, Interpretation};
use puan_core::{Composite, Bound, variable};

use tonic::{transport::Server, Request, Response, Status};

type InterpretationMap = std::collections::HashMap<String, i64>;

fn interpretation_map_from(interpretation: &Interpretation) -> InterpretationMap {
    interpretation.facts.iter().map(|fact| (fact.id.clone(), fact.value)).collect()
}

fn flip_mul(bound: Bound, direction: i32) -> Bound {
    match direction == 0 {
        true => bound,
        false => {
            Bound {
                lower: -1 * bound.upper,
                upper: -1 * bound.lower,
            }
        },
    }
}

fn evaluate(composite: &Composite, interpretation: &InterpretationMap) -> Bound {
    let inner_bound = composite.variables.iter().fold(
        Bound {lower: 0, upper: 0}, 
        |acc_bound, variable| {
            match &variable.part {
                Some(part) => {
                    let bound_part = flip_mul(
                        match part {
                            variable::Part::Primitive(inner_prim) => {
                                match interpretation.get(&inner_prim.id) {
                                    Some(value) => Bound {lower: *value, upper: *value},
                                    None => inner_prim.bound.clone().unwrap(),
                                }
                            },
                            variable::Part::Composite(inner_comp) => evaluate(
                                inner_comp,
                                interpretation,
                            )
                        },
                        composite.direction,
                    );
                    Bound {
                        lower: acc_bound.lower + bound_part.lower,
                        upper: acc_bound.upper + bound_part.upper,
                    }
                },
                None => return acc_bound,
            }
        }
    );
    return Bound {
        lower: (inner_bound.lower + composite.bias >= 0) as i64,
        upper: (inner_bound.upper + composite.bias >= 0) as i64,
    }
}

#[derive(Debug)]
struct PuanEvaluationService;

#[tonic::async_trait]
impl EvaluationService for PuanEvaluationService {

    async fn evaluate(
        &self,
        request: Request<EvaluationRequest>,
    ) -> Result<Response<EvaluationResponse>, Status> {
        let evaluation_request = request.into_inner();
        Ok(
            Response::new(
                EvaluationResponse {
                    evaluated: evaluation_request.proposition_interpretation_pairs.iter().map(
                        |prop_int_pair| {
                            evaluate(
                                &prop_int_pair.proposition.clone().unwrap(), 
                                &interpretation_map_from(&prop_int_pair.interpretation.clone().unwrap())
                            )
                        }
                    ).collect(),
                }
            )
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(
            EvaluationServiceServer::new(
                PuanEvaluationService {}
            )
        )
        .serve("[::1]:10000".parse().unwrap())
        .await?;

    Ok(())
}