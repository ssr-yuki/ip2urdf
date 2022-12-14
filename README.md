# ip2urdf

This project provides a tool converting CAD properties exported by Autodesk Inventor into a link description in URDF.

## Limitation

Since the tool is currently hard-coded, the Autodesk Inventor 2020 (Japanese) is only supported.

We welcome any contribution, including operation check reports in other Inventor versions!


## Installation

You can install the tool into your computer with the following command:

```bash
cargo install --path . --bin ip2urdf
```

## Usage

### Options

```
ip2urdf 0.1.0

USAGE:
    ip2urdf [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <direct>        Directly read from arguments
    -i <input>         Input file
    -o <output>        Output file, stdout if not present
```

### Examples

Load a text file and display the result:

```bash
ip2urdf -i sample.txt
```

Load a text file and write into a specified file:

```bash
ip2urdf -i sample.txt -o output.xml
```

Read properties from the argument and display the result:

```bash
ip2urdf -d '物理プロパティ Part1
一般的なプロパティ:
    材料:    {一般}
    密度:    1.00000000 g/cm^3
    質量:    6000.00000000 kg (相対誤差 = 0.000000%)
    領域:    22.00000000 m^2 (相対誤差 = 0.000000%)
    体積:    6.00000000 m^3 (相対誤差 = 0.000000%)
重心:
    X:    0.00000000 m (相対誤差 = 0.000000%)
    Y:    -0.00000000 m (相対誤差 = 0.000000%)
    Z:    0.00000000 m (相対誤差 = 0.000000%)
重心に関する慣性の質量モーメント(負の値を使って計算)
    Ixx            6500.00000000 kg m^2 (相対誤差 = 0.000000%)
    Iyx Iyy        -0.00000000 kg m^2 (相対誤差 = 0.000000%)    5000.00000000 kg m^2 (相対誤差 = 0.000000%)
    Izx Izy Izz    0.00000000 kg m^2 (相対誤差 = 0.000000%)    0.00000000 kg m^2 (相対誤差 = 0.000000%)    2500.00000000 kg m^2 (相対誤差 = 0.000000%)
グローバルに関する慣性の質量モーメント(負の値を使って計算)
    Ixx            6500.00000000 kg m^2 (相対誤差 = 0.000000%)
    Iyx Iyy        -0.00000000 kg m^2 (相対誤差 = 0.000000%)    5000.00000000 kg m^2 (相対誤差 = 0.000000%)
    Izx Izy Izz    0.00000000 kg m^2 (相対誤差 = 0.000000%)    0.00000000 kg m^2 (相対誤差 = 0.000000%)    2500.00000000 kg m^2 (相対誤差 = 0.000000%)
重心に関する慣性の基本モーメント
    I1:    6500.00000000 kg m^2 (相対誤差 = 0.000000%)
    I2:    5000.00000000 kg m^2 (相対誤差 = 0.000000%)
    I3:    2500.00000000 kg m^2 (相対誤差 = 0.000000%)
グローバルから焦点への回転
    Rx:    0.00000000 rad (相対誤差 = 0.000000%)
    Ry:    0.00000000 rad (相対誤差 = 0.000000%)
    Rz:    0.00000000 rad (相対誤差 = 0.000000%)
'
```

The output for these examples is:

```xml
<link name="Part1">
  <inertial>
    <origin xyz="0 -0 0" rpy="0 0 0"/>
    <mass value="6000"/>
    <inertia ixx="6500" ixy="-0" ixz="0" iyy="5000" iyz="0" izz="2500"/>
  </inertial>
</link>
```

### Tips

Using the `xsel` command, you can directly copy the result on the clipboard.

```bash
ip2urdf -i sample.txt | xsel --clipboard --input
```
