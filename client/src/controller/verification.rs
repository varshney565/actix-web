use crate::utils::votes::VOTES;

fn verify(faulty : i64,correct : i64) -> i8 {
    let total = faulty+correct;
    let f = (total-1)/3;
    if faulty > f {
        -1
    }else {
        1
    }
}

pub fn verification(id : i64) -> i8 {
    let mut correct_node = 0;
    let mut faulty_node = 0;
    let votes = VOTES.lock();
    if let Some(internal_map) = votes.get(&id) {
        for (_,value) in internal_map {
            if *value == 0 {
                faulty_node += 1;
            }else {
                correct_node += 1;
            }
        }
    }
    println!("{},{}",correct_node,faulty_node);
    verify(faulty_node, correct_node)
}