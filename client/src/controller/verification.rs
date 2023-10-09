use crate::utils::votes::VOTES;

pub fn verification(id : i64) -> i8 {
    let mut correct_node = 0;
    let mut faulty_node = 0;
    let votes = VOTES.lock();
    let mut vote = 0;
    let mut candidate = -1;
    if let Some(internal_map) = votes.get(&id) {
        for (_,(value,f,_)) in internal_map {
            if *value == 0 {
                faulty_node += 1;
            }else {
                correct_node += 1;
            }
            if vote == 0 {
                candidate = *f;
                vote = 1;
            }else {
                if *f == candidate {
                    vote += 1;
                }else {
                    vote -= 1;
                }
            }
        }
    }

    println!("C:{},F:{},f:{}",correct_node,faulty_node,candidate);
    if correct_node == 0 && faulty_node == 0 {
        return -1;
    }

    if faulty_node > candidate {
        -1
    }else {
        1
    }
}