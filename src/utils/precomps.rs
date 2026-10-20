pub const fn op_chars() -> ([bool; 256], [bool; 256]){
    let mut normal_table=[false; 256];
    let normal_ops=b"+-<>[],.";

    let mut i=0;
    while i<normal_ops.len(){
        normal_table[normal_ops[i] as usize]=true;
        i+=1;
    }

    let mut sbf_table=normal_table;
    let sbf_ops=b"0123456789~";

    let mut i=0;
    while i<sbf_ops.len(){
        sbf_table[sbf_ops[i] as usize]=true;
        i+=1;
    }

    return (normal_table, sbf_table);
}

pub const fn get_modular_inverses() -> [u8; 128]{
    let mut inverses=[0u8; 128];
    let mut num=1u8;
    loop{
        inverses[(num as usize-1)/2]=modular_inverse(num);
        if num==255{break;}
        num+=2;
    }
    return inverses;
}

const fn modular_inverse(val: u8) -> u8{
    //only for odd numbers
    if val%2==0{return 0;}

    let mut x=val;

    //yay newton iteration
    x=x.wrapping_mul(2u8.wrapping_sub(val.wrapping_mul(x)));
    x=x.wrapping_mul(2u8.wrapping_sub(val.wrapping_mul(x)));
    x=x.wrapping_mul(2u8.wrapping_sub(val.wrapping_mul(x)));

    return x;
}
