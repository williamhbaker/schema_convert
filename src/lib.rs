use arrow::datatypes::{DataType, Field, Schema};

pub fn json_to_arrow(json_str: &str) -> Result<Schema, anyhow::Error> {
    let parsed: serde_json::Value = serde_json::from_str(json_str)?;

    let val = match parsed.get("properties") {
        Some(v) => v,
        None => anyhow::bail!("JSON object missing key \"properties\""),
    };

    let val = match val.as_object() {
        Some(v) => v,
        None => anyhow::bail!("\"properties\" property is not an object"),
    };

    let fields = props_to_fields(val);
    match fields {
        Some(f) => Ok(Schema::new(f)),
        None => anyhow::bail!("could not create arrow schema"),
    }
}

fn props_to_fields(input: &serde_json::Map<String, serde_json::Value>) -> Option<Vec<Field>> {
    input
        .iter()
        .map(|(name, val)| prop_to_field(name, val))
        .collect()
}

fn prop_to_field(name: &str, prop: &serde_json::Value) -> Option<Field> {
    match prop.as_object()?.get("type")?.as_str()? {
        "object" => {
            let props = prop.get("properties")?;
            let nested = DataType::Struct(props_to_fields(props.as_object()?)?);
            Some(Field::new(name, nested, true))
        }
        "integer" => Some(Field::new(name, DataType::Int64, true)),
        "number" => Some(Field::new(name, DataType::Float64, true)),
        "boolean" => Some(Field::new(name, DataType::Boolean, true)),
        "null" => Some(Field::new(name, DataType::Null, true)),
        _ => None,
    }
}
