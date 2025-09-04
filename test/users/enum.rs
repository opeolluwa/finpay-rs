#[test]
fn test_freelance_type_is_correctly_deserialize() {
    let stringified_type = "freelancer".to_string();
    assert_eq!(AccountType::Freelancer.to_string(), stringified_type)
}

#[test]
fn test_company_type_is_correctly_deserialize() {
    let stringified_type = "company".to_string();
    assert_eq!(AccountType::Company.to_string(), stringified_type)
}
