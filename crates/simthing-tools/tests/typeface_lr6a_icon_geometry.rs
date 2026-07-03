use simthing_tools::IconVector;

const SIMPLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>
"##;

#[test]
fn icon_vector_geometry_is_deterministic() {
    let a = IconVector::from_svg(SIMPLE_SVG).expect("a");
    let b = IconVector::from_svg(SIMPLE_SVG).expect("b");
    assert_eq!(a, b);
    assert_eq!(a.geometry_hash(), b.geometry_hash());
}
