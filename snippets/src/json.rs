#[cfg(test)]
mod tests {
    use anyhow::Result;
    use serde_json::Value;

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
    fn compare_json_value() -> Result<()> {
        let v1: Value = serde_json::from_str(DATA)?;
        let v2: Value = serde_json::from_str(DATA2)?;
        assert_eq!(v1, v2);
        Ok(())
    }

    #[test]
    fn compare_json_string() -> Result<()> {
        let v1: Value = serde_json::from_str(DATA)?;
        let v2: Value = serde_json::from_str(DATA2)?;
        assert_eq!(v1.to_string(), v2.to_string());
        Ok(())
    }

}
