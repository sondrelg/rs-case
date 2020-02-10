

use super::*;

#[test]
fn valid_snake_case_translations() {
    assert_eq!(snake_case("PascalCase").unwrap(), "pascal_case");
    assert_eq!(snake_case("camelCase").unwrap(), "camel_case");
    assert_eq!(snake_case("snake_case").unwrap(), "snake_case");
    assert_eq!(snake_case("kebab-case").unwrap(), "kebab_case");
    assert_eq!(snake_case("TRAIN-CASE").unwrap(), "t_r_a_i_n__c_a_s_e");
    assert_eq!(snake_case("").unwrap(), "");
    assert_eq!(snake_case("1").unwrap(), "1");
    assert_eq!(snake_case("-22").unwrap(), "-22");
    assert_eq!(snake_case("lower").unwrap(), "lower");
    assert_eq!(snake_case("UPPER").unwrap(), "u_p_p_e_r");
}

#[test]
fn valid_camel_case_translations() {
    assert_eq!(camel_case("this_is-a_Test").unwrap(), "thisIsATest");
    assert_eq!(camel_case("PascalCase").unwrap(), "pascalCase");
    assert_eq!(camel_case("camelCase").unwrap(), "camelCase");
    assert_eq!(camel_case("snake_case").unwrap(), "snakeCase");
    assert_eq!(camel_case("kebab-case").unwrap(), "kebabCase");
    assert_eq!(camel_case("TRAIN-CASE").unwrap(), "tRAINCASE");
    assert_eq!(camel_case("").unwrap(), "");
    assert_eq!(camel_case("1").unwrap(), "1");
    assert_eq!(camel_case("-22").unwrap(), "-22");
    assert_eq!(camel_case("lower").unwrap(), "lower");
    assert_eq!(camel_case("UPPER").unwrap(), "uPPER");
}

#[test]
fn valid_pascal_case_translations() {
    assert_eq!(pascal_case("PascalCase").unwrap(), "PascalCase");
    assert_eq!(pascal_case("camelCase").unwrap(), "CamelCase");
    assert_eq!(pascal_case("snake_case").unwrap(), "SnakeCase");
    assert_eq!(pascal_case("kebab-case").unwrap(), "KebabCase");
    assert_eq!(pascal_case("TRAIN-CASE").unwrap(), "TRAINCASE");
    assert_eq!(pascal_case("").unwrap(), "");
    assert_eq!(pascal_case("1").unwrap(), "1");
    assert_eq!(pascal_case("-22").unwrap(), "-22");
    assert_eq!(pascal_case("lower").unwrap(), "Lower");
    assert_eq!(pascal_case("UPPER").unwrap(), "UPPER");
}


#[test]
fn valid_kebab_case_translations() {
    assert_eq!(kebab_case("PascalCase").unwrap(), "pascal-case");
    assert_eq!(kebab_case("camelCase").unwrap(), "camel-case");
    assert_eq!(kebab_case("snake_case").unwrap(), "snake-case");
    assert_eq!(kebab_case("kebab-case").unwrap(), "kebab-case");
    assert_eq!(kebab_case("TRAIN-CASE").unwrap(), "t-r-a-i-n--c-a-s-e");
    assert_eq!(kebab_case("").unwrap(), "");
    assert_eq!(kebab_case("1").unwrap(), "1");
    assert_eq!(kebab_case("-22").unwrap(), "-22");
    assert_eq!(kebab_case("lower").unwrap(), "lower");
    assert_eq!(kebab_case("UPPER").unwrap(), "u-p-p-e-r");
}

#[test]
fn valid_train_case_translations() {
    assert_eq!(train_case("PascalCase").unwrap(), "PASCAL-CASE");
    assert_eq!(train_case("camelCase").unwrap(), "CAMEL-CASE");
    assert_eq!(train_case("snake_case").unwrap(), "SNAKE-CASE");
    assert_eq!(train_case("kebab-case").unwrap(), "KEBAB-CASE");
    assert_eq!(train_case("TRAIN-CASE").unwrap(), "T-R-A-I-N--C-A-S-E");
    assert_eq!(train_case("").unwrap(), "");
    assert_eq!(train_case("1").unwrap(), "1");
    assert_eq!(train_case("-22").unwrap(), "-22");
    assert_eq!(train_case("lower").unwrap(), "LOWER");
    assert_eq!(train_case("UPPER").unwrap(), "U-P-P-E-R");
}
