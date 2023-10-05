use crate::utils::metadata::VOTES;
use crate::utils::metadata::NODES;

pub async fn verify(id : i64) -> i8 {
    let n = NODES.lock();
    let mut correct_node = 0;
    let votes = VOTES.lock();
    if let Some(internal_map) = votes.get(&id) {
        for (_,value) in internal_map {
            if *value == 1 {
                correct_node += 1;
            }
        }
    }

    let f = (*n-1)/3;
    let faulty_node = *n-correct_node;
    println!("{},{}",correct_node,faulty_node);
    if faulty_node > f {
        -1
    }else {
        1
    }
}