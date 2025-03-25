pub fn first_word(s: &string) -> usize{
    let bytes = s.as_butes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}