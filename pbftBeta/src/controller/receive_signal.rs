use actix_web::{HttpResponse,web};

use crate::utils::metadata::{Proposal, PROPOSALS, ACCEPTER, NODES};

pub async fn receive_signal(proposal : web::Json<Proposal>) -> HttpResponse {
    /*
     * check whether both the proposals are same or not. 
     * if yes increase the value of getting accepted by 1.
     */
    println!("RECEIVED A FAVOR");
    let proposals = PROPOSALS.lock();
    let received_proposal = proposals.get(&proposal.0.id).unwrap();
    if proposal.0 == *received_proposal {
        drop(proposals);
        let mut accepter = ACCEPTER.lock();
        if let Some(val) = accepter.get_mut(&proposal.0.id) {
            *val += 1;
        }else {
            accepter.insert(proposal.0.id, 1);
        }
        drop(accepter);
    }

    /*
     * now find the how many votes are in favor of the proposal.
     * if more than 2*f or equal are in favor brodcast the votes. 
     */
    let accepter = ACCEPTER.lock();
    let total_favor = *accepter.get(&proposal.0.id).unwrap_or_else(|| &0);
    drop(accepter);
    let mut nodes = NODES.lock();
    let state = nodes.get_mut(&proposal.0.id).unwrap();
    if state.2 == false && 2*state.1 <=  total_favor {
        /*
        * brodcast the vote to all the nodes
        * */
        println!("BRODCAST START");
        state.2 = true;
    }

    drop(nodes);
    HttpResponse::Ok().json("success")
}