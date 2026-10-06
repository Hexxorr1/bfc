
//only leaves symbols that matter for the code
pub fn remove_comments(code: &mut String, is_shortened: bool){
    //basic bf chars
    let basic_chars=vec!['+', '-', '<', '>', '[', ']', ',', '.'];

    //additional sbf chars
    let additional_chars=vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '(', ')'];
    let valid_chars=if is_shortened{
        [basic_chars, additional_chars].concat()
    }else{
        basic_chars
    };

    *code=code.chars().filter(|c| valid_chars.contains(&c)).collect::<String>();
}

//only call after remove_comments(code)
//mess, to be optimised
pub fn shorten(code: &mut String){
    let mut shortened_code=String::new();

    let shortenable_chars=vec!['+', '-', '<', '>'];
    let mut counter=(' ', 0u32);

    for c in code.chars(){
        if shortenable_chars.contains(&c){
            if counter.0==c||counter.0==' '{
                counter=(c, counter.1+1);
            }else{
                shortened_code+=&("(".to_string()+counter.1.to_string().as_str()+counter.0.to_string().as_str()+")");
                counter=(c, 1);
            }
        }else{
            shortened_code+=c.to_string().as_str();
        }
    }

    *code=shortened_code;
}
