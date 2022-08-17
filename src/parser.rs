#[derive(Clone, Debug, Default)]
pub struct LinkProperties {
    pub name: Option<String>,
    pub mass: Option<f64>,
    pub com_x: Option<f64>,
    pub com_y: Option<f64>,
    pub com_z: Option<f64>,
    pub ixx: Option<f64>,
    pub ixy: Option<f64>,
    pub ixz: Option<f64>,
    pub iyy: Option<f64>,
    pub iyz: Option<f64>,
    pub izz: Option<f64>,
}

pub fn parse_properties(text: String) -> LinkProperties {
    let lines = text.lines();

    let mut p = LinkProperties::default();

    for line in lines {
        if line.contains("物理プロパティ") {
            let name = line.split_whitespace().nth(1).unwrap().to_owned();
            p.name = Some(name);
        } else if line.contains("質量:") {
            let mass_str = line.split_whitespace().nth(1).unwrap_or("0.0");
            p.mass = Some(mass_str.parse().unwrap());
        } else if line.contains("X:") {
            let com_x_str = line.split_whitespace().nth(1).unwrap_or("0.0");
            p.com_x = Some(com_x_str.parse().unwrap());
        } else if line.contains("Y:") {
            let com_y_str = line.split_whitespace().nth(1).unwrap_or("0.0");
            p.com_y = Some(com_y_str.parse().unwrap());
        } else if line.contains("Z:") {
            let com_z_str = line.split_whitespace().nth(1).unwrap_or("0.0");
            p.com_z = Some(com_z_str.parse().unwrap());
        } else if line.contains("Ixx") && p.ixx.is_none() {
            let ixx_str = line.split_whitespace().nth(1).unwrap_or("0.0");
            p.ixx = Some(ixx_str.parse().unwrap());
        } else if line.contains("Iyx Iyy") {
            if p.ixy.is_none() {
                let ixy_str = line.split_whitespace().nth(2).unwrap_or("0.0");
                p.ixy = Some(ixy_str.parse().unwrap());
            }
            if p.iyy.is_none() {
                let iyy_str = line.split_whitespace().nth(8).unwrap_or("0.0");
                p.iyy = Some(iyy_str.parse().unwrap());
            }
        } else if line.contains("Izx Izy Izz") {
            if p.ixz.is_none() {
                let ixz_str = line.split_whitespace().nth(3).unwrap_or("0.0");
                p.ixz = Some(ixz_str.parse().unwrap());
            }
            if p.iyz.is_none() {
                let iyz_str = line.split_whitespace().nth(9).unwrap_or("0.0");
                p.iyz = Some(iyz_str.parse().unwrap());
            }
            if p.izz.is_none() {
                let izz_str = line.split_whitespace().nth(15).unwrap_or("0.0");
                p.izz = Some(izz_str.parse().unwrap());
            }
        }
    }
    p
}

#[test]
fn test_parse() {
    let file_path = std::path::Path::new("sample.txt");
    let text = std::fs::read_to_string(file_path).unwrap();
    let p = parse_properties(text);

    assert_eq!(p.name.unwrap(), "Part1".to_string());
    assert_eq!(p.mass.unwrap(), 6000.0);
    assert_eq!(p.com_x.unwrap(), 0.0);
}
