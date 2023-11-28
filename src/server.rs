mod puan_eval;
mod puan_core;

use puan_eval::evaluation_service_server::{EvaluationService, EvaluationServiceServer};
use puan_eval::{PropositionInterpretationSet, BoundCollection, BoundSet, Interpretation, PropositionInterpretationPair};
use puan_core::{Composite, Bound, variable};

use tonic::{transport::Server, Request, Response, Status};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream, StreamExt};

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
        request: Request<PropositionInterpretationSet>,
    ) -> Result<Response<BoundCollection>, Status> {
        let evaluation_request = request.into_inner();
        Ok(Response::new(BoundCollection {
            bound_sets: evaluation_request.propositions.iter().map(|proposition| {
                BoundSet {
                    bounds: evaluation_request.interpretations.iter().map(|interpretation| {
                        evaluate(&proposition, &interpretation_map_from(interpretation))
                    }).collect()
                }
            }).collect()
        }))
    }

    type EvaluatePairStream = Pin<Box<dyn Stream<Item = Result<Bound, Status>> + Send + 'static>>;
    
    async fn evaluate_pair(
        &self,
        request: Request<tonic::Streaming<PropositionInterpretationPair>>,
    ) -> Result<Response<Self::EvaluatePairStream>, Status> {
        
        let mut stream = request.into_inner();

        // Create a channel for sending results to the stream
        let (tx, rx) = mpsc::unbounded_channel();

        // Spawn a task to process the stream and send results to the channel
        tokio::spawn(async move {
            while let Some(pair) = stream.next().await {
                // Process the pair and produce a result
                let result = match pair {
                    Ok(pair) => {
                        // Process the pair and return the result
                        Ok(
                            evaluate(
                                &pair.proposition.unwrap(),
                                &interpretation_map_from(&pair.interpretation.unwrap())
                            )
                        )
                    }
                    Err(status) => {
                        // Handle the error condition indicated by the Status
                        Err(status)
                    }
                };
                
                // Send the result to the channel
                if let Err(_) = tx.send(result) {
                    // Handle sender error (if the receiver is dropped)
                    break;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::EvaluatePairStream))

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