use crate::parser::LinkProperties;

pub fn generate(p: &LinkProperties) -> String {
    format!(
        "<link name=\"{}\">\n",
        p.name.clone().unwrap_or_else(|| "LINK".to_string())
    ) + "  <inertial>\n".to_string().as_str()
        + format!(
            "    <origin xyz=\"{} {} {}\" rpy=\"0 0 0\"/>\n",
            p.com_x.unwrap_or(0.0),
            p.com_y.unwrap_or(0.0),
            p.com_z.unwrap_or(0.0)
        )
        .as_str()
        + format!("    <mass value=\"{}\"/>\n", p.mass.unwrap_or(1.0)).as_str()
        + format!(
            "    <inertia ixx=\"{}\" ixy=\"{}\" ixz=\"{}\" iyy=\"{}\" iyz=\"{}\" izz=\"{}\"/>\n",
            p.ixx.unwrap_or(1.0),
            p.ixy.unwrap_or(0.0),
            p.ixz.unwrap_or(0.0),
            p.iyy.unwrap_or(1.0),
            p.iyz.unwrap_or(0.0),
            p.izz.unwrap_or(1.0)
        )
        .as_str()
        + "  </inertial>\n".to_string().as_str()
        + "</link>\n".to_string().as_str()
}

#[test]
fn test_generate() {
    assert_eq!(
        generate(&LinkProperties::default()),
        "<link name=\"LINK\">
  <inertial>
    <origin xyz=\"0 0 0\" rpy=\"0 0 0\"/>
    <mass value=\"1\"/>
    <inertia ixx=\"1\" ixy=\"0\" ixz=\"0\" iyy=\"1\" iyz=\"0\" izz=\"1\"/>
  </inertial>
</link>
"
    );
}
