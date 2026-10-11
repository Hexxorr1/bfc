
#[test]
fn decommenting(){
    let mut code="hewwo +<>AiUSe and4+=- weloc~ome .,[D67][as~~d[2c..,".to_string();
    crate::core::preprocess::remove_comments(&mut code, false);
    assert_eq!(code, "+<>+-.,[][[..,");

    let mut sbf_code="hewwo +<>AiUSe and~4+=- welocome .,[D67][a][s~~~d[2c..,".to_string();
    crate::core::preprocess::remove_comments(&mut sbf_code, true);
    assert_eq!(sbf_code, "+<>~4+-.,[67][][~~~[2..,");
}

#[test]
fn shortening(){
    let mut code="++++++<[->]++++-<<<<>++--++++[++++]++<+->".to_string();
    crate::core::preprocess::shorten(&mut code).unwrap();
    assert_eq!(code, "+6<[->]+3<3+4[+4]+2<>");
}

#[test]
fn shortening_single(){
    use crate::core::preprocess::shorten_ins;

    let test=shorten_ins(('i', 1)).unwrap();
    assert_eq!(test, "+");
    let test=shorten_ins(('i', -1)).unwrap();
    assert_eq!(test, "-");
    let test=shorten_ins(('i', 2)).unwrap();
    assert_eq!(test, "+2");
    let test=shorten_ins(('i', -2)).unwrap();
    assert_eq!(test, "-2");
    let test=shorten_ins(('m', 1)).unwrap();
    assert_eq!(test, ">");
    let test=shorten_ins(('m', 2)).unwrap();
    assert_eq!(test, ">2");
    let test=shorten_ins(('m', -1)).unwrap();
    assert_eq!(test, "<");
    let test=shorten_ins(('m', -2)).unwrap();
    assert_eq!(test, "<2");

    let test=shorten_ins((' ', 0));
    assert_eq!(test, Ok("".to_string()));

    let test=shorten_ins((' ', 1));
    assert_eq!(test, Err(crate::error::BfcError::InvalidInstruction(' '.to_string())));
}

#[test]
fn to_operation_list(){
    use crate::core::builder::Instruction::*;
    let code="[+5<3.>2]>.,+[+<-]+6.+5".to_string();
    let output=crate::core::builder::build_instruction_list(&code).unwrap();
    let desired_output=vec![LoopBegin(5), Add(5), Move(-3), Print, Move(2), LoopEnd(0), Move(1), Print, Read, Add(1), LoopBegin(14), Add(1), Move(-1), Add(255), LoopEnd(10), Add(6), Print, Add(5)];
    assert_eq!(output, desired_output);
}

#[test]
fn operation_optimization_pattern_matching(){
    use crate::core::builder::optimize_code;
    use crate::core::builder::Instruction::*;

    let uop=vec![LoopBegin(2), Add(255), LoopEnd(0), Add(67)];
    let op=optimize_code(&uop, 2).unwrap().0;
    assert_eq!(op, vec![Set(67)]);
}
