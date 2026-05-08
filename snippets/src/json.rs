use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct ExampleDto {
    // #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use serde::Deserialize;
    use serde_json::Value;

    use crate::json::ExampleDto;

    const DATA: &str = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    const DATA2: &str = r#"
        {
            "name": "Jane Doe",
            "age": 43,
            "phones": [
                "+44 789789",
                "+44 2345678"
            ]
        }"#;

    #[test]
    #[ignore] // Uncomment to view assert error message
    fn compare_json_value_neq() {
        let v1: Value = serde_json::from_str(DATA).unwrap();
        let v2: Value = serde_json::from_str(DATA2).unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn compare_json_value() {
        let v1: Value = serde_json::from_str(DATA).unwrap();
        let v3 = r#"
        {
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "name": "John Doe"
        }"#;
        let v3: Value = serde_json::from_str(v3).unwrap();
        assert_eq!(v1, v3);
    }

    #[test]
    #[ignore] // Uncomment to view assert error message
    fn compare_json_string()  {
        let v1: Value = serde_json::from_str(DATA).unwrap();
        let v2: Value = serde_json::from_str(DATA2).unwrap();
        assert_eq!(v1.to_string(), v2.to_string());
    }

    fn deserialize<'de, T: Deserialize<'de>>(content: &'de str) -> Result<T> {
        let v = serde_json::from_str::<T>(content)?;
        Ok(v)
    }

    #[test]
    fn test_generic_conversion() {
        let json = r#"
        {
            "id": "423f552e-12f6-43e1-a6e0-43b47857f838",
            "name": "John Doe"
        }"#;

        let result = deserialize::<ExampleDto>(json).unwrap();
        assert_eq!(result.name, "John Doe");
    }
}
