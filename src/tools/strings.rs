// requisites for byte pattern being a string:
//    - the bytes of the array have to be between 0x20 and 0x7E (ASCII)
//    - +1 lenght
pub fn string_scanner(aob: &[u8]) -> Vec<(usize, String)> { // tuple with the string + where it was found
    let mut temporal_string: String = String::new();
    let mut string_list: Vec<(usize, String)> = Vec::new();
    let mut start_offset: usize = 0;

    for i in 0..aob.len() {
        if aob[i] >= 0x20 && aob[i] <= 0x7E {
            if temporal_string.is_empty() {
                start_offset = i;
            }
            temporal_string.push(aob[i] as char);
        }
        else {
            if temporal_string.len() < 2 {
                temporal_string.clear();
            }
            else{
                string_list.push((start_offset, temporal_string.clone()));
                temporal_string.clear();
            }
        }
    }

    string_list
}