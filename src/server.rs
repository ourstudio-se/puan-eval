mod puan_core;
use puan_core::{LinearBoundedTree, Bound, bic_or_bound, BicOrBound};
use puan_core::lbt_evaluation_service_server::{LbtEvaluationService, LbtEvaluationServiceServer};

use tonic::{transport::Server, Request, Response, Status};
use std::collections::VecDeque;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream, StreamExt};

pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");

// Propagates a Linear Bounded Tree (LBT) by propagating all Binary Inequality Constraints (BICs)
// in the tree. This is done by looping through all nodes in the tree, and propagating the BICs
// that have all their children propagated. If a BIC has a child that is not propagated, we push
// the current node to the back of the queue and wait for the child to be propagated. If a BIC has
// a child that is not in the tree, we cannot ever propagate the BIC, so we just ignore it.
//
// The algorithm is implemented as a function that takes a LinearBoundedTree and returns a new
// LinearBoundedTree. This is done to make it easier to test the algorithm, since we can just
// create a LinearBoundedTree and pass it to the function, and then check the result.
//
// # Arguments
//
// * `tree` - A LinearBoundedTree to propagate
//
// # Returns
//
// A new LinearBoundedTree with all BICs propagated
fn propagate(tree: &LinearBoundedTree) -> LinearBoundedTree {
    let mut _tree = tree.clone();
    let mut queue: VecDeque<String> = tree.nodes.keys().cloned().collect();

    while queue.len() > 0 {
        // Extract current node by popping from the queue
        let current = queue.pop_front().unwrap();
        let current_node = _tree.nodes.get(&current).unwrap();

        // We should not have any nodes without parts
        // so we panic if we encounter one
        match &current_node.part {
            Some(part) => {

                // A node could be either a Binary Inequality Constraint (BIC)
                // or a Bound. We are actually only interested in BICs, since
                // bounds are already propagated.
                match part {
                    bic_or_bound::Part::Bic(bic) => {

                        // If all children are bounds, we can propagate the BIC
                        // Otherwise, two things may happen. Either we push the
                        // current node to the back of the queue and wait for its
                        // children to be propagated, or we find a child that is 
                        // not in the tree, in which case we cannot ever propagate
                        // the BIC.
                        let mut new_lower_bound: i64 = 0;
                        let mut new_upper_bound: i64 = 0;

                        // Since we don't really know when looping is over if we've taken all
                        // relations into account or not, we need to count all the ones that was
                        // taken into account.
                        let mut relations_taken_into_account: usize = 0;

                        for child in bic.relations.iter() {
                            match _tree.nodes.get(&child.id) {
                                Some(child_node) => {
                                    match &child_node.part {
                                        Some(child_part) => {
                                            match child_part {
                                                // Here we find a node that hasn't been propagated
                                                // so we push the current node to the back of the queue
                                                // and hoping for all being propagated in the future.
                                                bic_or_bound::Part::Bic(_) => {
                                                    queue.push_back(child.id.to_string());
                                                    break;
                                                },
                                                bic_or_bound::Part::Bound(bound) => {
                                                    // We need to flip the child's bound if the coefficient is negative
                                                    // (because -1*(0,1) === (-1,0))
                                                    if child.coefficient < 0 {
                                                        new_lower_bound += bound.upper * child.coefficient;
                                                        new_upper_bound += bound.lower * child.coefficient;
                                                    } else {
                                                        new_lower_bound += bound.lower * child.coefficient;
                                                        new_upper_bound += bound.upper * child.coefficient;
                                                    }

                                                    // We've taken this relation into account
                                                    relations_taken_into_account += 1;
                                                },
                                            }
                                        },
                                        None => panic!("Node {} has no part", child.id),
                                    }
                                },
                                // Here's when the child node cannot ever be propagated
                                // so we just don't do anything. Not calculating the new
                                // bound, nor pushing the current node to the back of the
                                // queue.
                                None => break,
                            }
                        }
                        
                        if relations_taken_into_account == bic.relations.len() {
                            // We've taken all relations into account, so we can propagate
                            // the BIC. Update the node in the tree
                            _tree.nodes.entry(current.clone()).and_modify(|existing_value| {
                                *existing_value = BicOrBound{
                                    part: Some(bic_or_bound::Part::Bound(
                                        Bound {
                                            lower: (new_lower_bound >= 0) as i64,
                                            upper: (new_upper_bound >= 0) as i64,
                                        }
                                    )),
                                }
                            });
                        }
                    },
                    // We're not interested in bounds, since they are already propagated
                    bic_or_bound::Part::Bound(_) => {},
                }
            },
            None => panic!("Node {} has no part", current),
        }
    }

    return _tree;
}

#[derive(Debug)]
struct PuanEvaluationService;

#[tonic::async_trait]
impl LbtEvaluationService for PuanEvaluationService {

    type PropagateLbtStreamedStream = Pin<Box<dyn Stream<Item = Result<LinearBoundedTree, Status>> + Send + 'static>>;
    
    async fn propagate_lbt_streamed(
        &self,
        request: Request<tonic::Streaming<LinearBoundedTree>>,
    ) -> Result<Response<Self::PropagateLbtStreamedStream>, Status> {
        
        let mut stream = request.into_inner();

        // Create a channel for sending results to the stream
        let (tx, rx) = mpsc::unbounded_channel();

        // Spawn a task to process the stream and send results to the channel
        tokio::spawn(async move {
            while let Some(lbt) = stream.next().await {
                let result = Ok(
                    propagate(
                        &lbt.unwrap(),
                    )
                );
                
                // Send the result to the channel
                if let Err(_) = tx.send(result) {
                    // Handle sender error (if the receiver is dropped)
                    break;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::PropagateLbtStreamedStream))

    }

    async fn propagate_lbt(
        &self,
        request: Request<LinearBoundedTree>,
    ) -> Result<Response<LinearBoundedTree>, Status> {
        let lbt = request.into_inner();
        let propagated_lbt = propagate(&lbt);
        Ok(Response::new(propagated_lbt))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(
            LbtEvaluationServiceServer::new(
                PuanEvaluationService {}
            )
        )
        .add_service(reflection_server)
        .serve("[::1]:10000".parse().unwrap())
        .await?;

    Ok(())
}