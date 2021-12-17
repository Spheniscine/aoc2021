use std::collections::HashSet;

pub fn display_dots(dots: &HashSet<[i32; 2]>) -> String {
    if dots.is_empty() { return String::new(); }
    let mut res = String::new();
    let xmin = dots.iter().map(|p| p[0]).min().unwrap();
    let xmax = dots.iter().map(|p| p[0]).max().unwrap();
    let ymin = dots.iter().map(|p| p[1]).min().unwrap();
    let ymax = dots.iter().map(|p| p[1]).max().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if dots.contains(&[x, y]) {
                res.push('#');
            } else {
                res.push('.');
            }
        }
        res.push('\n');
    }
    
    res
}