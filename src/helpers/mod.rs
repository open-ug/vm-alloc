use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use xmltree::{Element, XMLNode};

/// Recursively convert JSON value into XML element
fn value_to_xml(value: &Value, tag: &str) -> Element {
    let mut elem = Element::new(tag);

    if let Value::Object(map) = value {
        for (k, v) in map {
            if k.starts_with('@') {
                // Attribute
                elem.attributes.insert(
                    k.trim_start_matches('@').to_string(),
                    v.as_str().unwrap_or(&v.to_string()).to_string(),
                );
            } else if k == "#text" {
                // Inner text
                if let Some(s) = v.as_str() {
                    elem.children.push(XMLNode::Text(s.to_string()));
                } else {
                    elem.children.push(XMLNode::Text(v.to_string()));
                }
            } else {
                // Child element
                match v {
                    Value::Array(arr) => {
                        for item in arr {
                            elem.children.push(XMLNode::Element(value_to_xml(item, k)));
                        }
                    }
                    Value::Object(_) => {
                        elem.children.push(XMLNode::Element(value_to_xml(v, k)));
                    }
                    Value::Null => {
                        // self-closing empty tag
                        elem.children.push(XMLNode::Element(Element::new(k)));
                    }
                    _ => {
                        let mut child = Element::new(k);
                        child.children.push(XMLNode::Text(
                            v.as_str().unwrap_or(&v.to_string()).to_string(),
                        ));
                        elem.children.push(XMLNode::Element(child));
                    }
                }
            }
        }
    }

    elem
}

/// Convert any serializable struct into XML
pub fn struct_to_xml<T: Serialize>(value: &T, root_name: &str) -> String {
    let json = serde_json::to_value(value).unwrap();
    let el = value_to_xml(&json, root_name);

    let mut buffer = Vec::new();
    el.write_with_config(
        &mut buffer,
        xmltree::EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(false),
    )
    .unwrap();

    let xml_str = String::from_utf8(buffer).unwrap();
    xml_str
}

/// Recursively convert XML element to JSON value
fn xml_to_value(elem: &Element) -> Value {
    let mut map = serde_json::Map::new();

    // Attributes → @field
    for (k, v) in &elem.attributes {
        map.insert(format!("@{}", k), Value::String(v.clone()));
    }

    // Children
    for child in &elem.children {
        match child {
            XMLNode::Text(t) => {
                map.insert("#text".to_string(), Value::String(t.clone()));
            }
            XMLNode::Element(e) => {
                let child_val = xml_to_value(e);
                if let Some(existing) = map.get_mut(&e.name) {
                    if let Value::Array(arr) = existing {
                        arr.push(child_val);
                    } else {
                        let old = existing.take();
                        map.insert(e.name.clone(), Value::Array(vec![old, child_val]));
                    }
                } else {
                    map.insert(e.name.clone(), child_val);
                }
            }
            _ => {}
        }
    }

    if map.is_empty() {
        if let Some(text) = elem.get_text() {
            Value::String(text.to_string())
        } else {
            Value::Null
        }
    } else {
        Value::Object(map)
    }
}

/// Decode XML string into struct T (ignores unknown fields)
pub fn xml_to_struct<T: DeserializeOwned>(xml: &str) -> T {
    let root: Element = Element::parse(xml.as_bytes()).unwrap();
    let mut value = xml_to_value(&root);
    //print!("Converted XML to JSON Value: {}\n", value);
    value = normalize_value(value);
    serde_json::from_value(value).unwrap()
}

fn normalize_value(value: Value) -> Value {
    match value {
        Value::Object(mut map) => {
            if map.len() == 1 {
                if let Some(text) = map.remove("#text") {
                    return text;
                }
            }
            Value::Object(
                map.into_iter()
                    .map(|(k, v)| (k, normalize_value(v)))
                    .collect(),
            )
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(normalize_value).collect()),
        other => other,
    }
}
