
pub fn la_plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else{
            y
        }
}