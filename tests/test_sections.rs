use string_sections::prelude::Sections;

#[test]
fn test_simple() {
    let doc = r#"
Text

<!--
to find
-->
text
text"#;

    let sections = doc
        .sections(
            |line| line.contains("<!--"),
            |sect| sect.end_line.contains("-->"),
        )
        .collect::<Vec<_>>();

    assert_eq!(sections.len(), 1);
    let section = sections[0];
    assert_eq!(section.as_str(), "to find");
    assert_eq!(section.start_line.as_str(), "<!--");
    assert_eq!(section.end_line.as_str(), "-->");
}
