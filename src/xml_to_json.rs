use quickxml_to_serde::Config;
use lazy_static::lazy_static;

lazy_static! {
    static ref XML_CONFIG_DEFAULT: Config = Config::new_with_defaults();
}

pub fn xml_str_to_json(xml: &str) -> Result<serde_json::Value, minidom::error::Error> {
    let config = &*XML_CONFIG_DEFAULT;
    let json = quickxml_to_serde::xml_str_to_json(xml, config);
    json
}

pub fn xml_string_to_json(xml: String) -> Result<serde_json::Value, minidom::error::Error> {
    let config = &*XML_CONFIG_DEFAULT;
    let json = quickxml_to_serde::xml_string_to_json(xml, config);
    json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_to_json() {
        let xml = r#"<root><a>1</a><b>2</b></root>"#;
        let json = xml_str_to_json(xml).unwrap();
        let expected = serde_json::json!({
            "root": {
                "a": 1,
                "b": 2
            }
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn test_xml_attr_to_json() {
        let xml = r#"<root a="1" b="2"></root>"#;
        let json = xml_str_to_json(xml).unwrap();
        let expected = serde_json::json!({
            "root": {
                "@a": 1,
                "@b": 2
            }
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn test_xml_attr_and_child_to_json() {
        let xml = r#"<root a="1"><b>2</b></root>"#;
        let json = xml_str_to_json(xml).unwrap();
        let expected = serde_json::json!({
            "root": {
                "@a": 1,
                "b": 2
            }
        });
        assert_eq!(json, expected);
    }
}