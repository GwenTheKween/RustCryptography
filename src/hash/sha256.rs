pub fn calculate(s: &str){
//    let bytes = s.as_bytes();
    let len = get_size(s);
    let mut block_count = (len/64) + 1;
    let final_block: [u8;64] = preprocess_final_block(s, len);
    if block_count == 1 {
        println!("single loop hash");
    }else{
        println!("multi loop hash, not yet supported");
        return;
    }

    //initializing constants
    let mut h0:u32 = 0x6a09e667;
    let mut h1:u32 = 0xbb67ae85;
    let mut h2:u32 = 0x3c6ef372;
    let mut h3:u32 = 0xa54ff53a;
    let mut h4:u32 = 0x510e527f;
    let mut h5:u32 = 0x9b05688c;
    let mut h6:u32 = 0x1f83d9ab;
    let mut h7:u32 = 0x5be0cd19;

    while(block_count >1){
        //multiblock thingy-me-bobbers, 
        block_count -=1;
    }
}

fn get_size(s: &str) -> usize{
    //TO BE CHANGED, GET THE SIZE OF THE FILE
    let bytes = s.as_bytes();
    bytes.len()
}

fn preprocess_final_block(s: &str, len :usize) -> [u8;64]{
    //TO BE CHANGED, makes the preprocessing of the final block of the text
    let mut bytes: [u8;64] = [0,0,0,0,0,0,0,0,
                              0,0,0,0,0,0,0,0,
                              0,0,0,0,0,0,0,0,
                              0,0,0,0,0,0,0,0,
                              0,0,0,0,0,0,0,0,
                              0,0,0,0,0,0,0,0,
                              0,0,0,0,0,0,0,0,
                              0,0,0,0,0,0,0,0];
    let mut it = 0;
    let ss = s.as_bytes();
    let len_array = len.to_be_bytes();
    while(it < len){
        bytes[it] = ss[it];
        it += 1;
    }
    bytes[it] = 0x80;
    it += 1;
    while(it < (64 - len_array.len())){
        bytes[it] = 0;
        it += 1
    }
    for i in 0..len_array.len(){
        bytes[it+i] = len_array[i];
    }
    bytes
}
