//@TODO move the following code into a separate file so that MyData is imported
// external.rs will be generated by OV-cli based on parameters that are passed to it.
// it describes the data model that the function contained in a Rule is run on
// use mock external.rs so that we can build

// deny unsafe code
#![deny(unsafe_code)]
#![warn(
    ellipsis_inclusive_range_patterns,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused,
    unused_results,
    rust_2018_idioms
)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::perf,
    clippy::complexity,
    clippy::style,
    clippy::pedantic,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::pub_enum_variant_names,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::redundant_clone,
    clippy::empty_enum,
    clippy::explicit_iter_loop,
    clippy::match_same_arms,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::path_buf_push_overwrite
)]
#![allow(clippy::too_many_lines)]

pub struct Model {
    age: u8,
    name: String,
}

pub mod huml {

    #[derive(Default, Debug)]
    pub struct HumlFrameWork<'a> {
        rules: Vec<ValidationRule<'a>>,
    }

    impl<'a> HumlFrameWork<'a> {
        pub fn new() -> Self {
            Self { rules: vec![] }
        }
    }

    impl<'a> HumlFrameWork<'a> {
        #[must_use]
        pub fn validate(&self, data: &crate::Model) -> ValidationSummary {
            let mut val_errors: Vec<ValidationError> = Vec::new();
            let mut fields: Vec<String> = Vec::new();

            for rule in self
                .rules
                .iter()
                .filter(|rule| rule.toggle == RuleToggle::Enabled)
            {
                let validation_failed: bool = (rule.function)(data);
                if validation_failed {
                    let val_error = ValidationError {
                        error: rule.error.clone(),
                        fields: rule.fields.clone(),
                    };
                    fields.extend(rule.fields.clone());
                    val_errors.push(val_error);
                }
            }

            let mut summary = ValidationSummary {
                has_errors: !val_errors.is_empty(),
                errors: val_errors,
                fields,
            };
            summary.fields.sort();
            summary.fields.dedup();
            summary
        }

        pub fn append_rule<T: Into<String>>(
            &mut self,
            name: T,
            error: T,
            toggle: RuleToggle,
            fields: Vec<T>,
            function: &'a dyn Fn(&crate::Model) -> bool,
        ) {
            let rule = ValidationRule {
                name: name.into(),
                error: error.into(),
                toggle,
                fields: fields.into_iter().map(std::convert::Into::into).collect(),
                function,
            };
            self.rules.push(rule);
        }
    }
    /// expresses if a rule is enabled or disabe
    #[derive(PartialEq, Clone, Debug)]
    pub enum RuleToggle {
        Enabled,
        Disabled,
    }

    struct ValidationRule<'a> {
        name: String,
        error: String,
        toggle: RuleToggle,
        fields: Vec<String>,
        function: &'a dyn Fn(&crate::Model) -> bool,
    }

    impl<'a> std::fmt::Debug for ValidationRule<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //{{{{raw-helper}}}}
            // ^ make sure handlebars does not choke on "ValidationRule {{ ... }}"
            #[allow(clippy::write_literal)]
            write!(f, "ValidationRule {{ name: {:?}, fields: {:?}, errormsg: {:?}, disabled: {:?}, function: {}, data: {} }}", self.name, self.fields, self.error, self.toggle, "(omitted):&'a dyn Fn(&crate::MyData) -> bool", "(omitted)&'a crate::MyData")
            //{{{{/raw-helper}}}}
        }
    }

    #[derive(Debug)]
    pub struct ValidationSummary {
        pub has_errors: bool,
        pub errors: Vec<ValidationError>,
        pub fields: Vec<String>,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct ValidationError {
        pub error: String,
        pub fields: Vec<String>,
    }

    // a (in?)complete list of operators can be found here:
    // https://github.com/openvalidation/openvalidation/blob/09ae4e62c9e7d9efae89a307a7dd68f2db252649/openvalidation-common/src/main/java/io/openvalidation/common/ast/ASTComparisonOperator.java#L19
    // git grep "public enum AstComparisonOperator"

    // https://users.rust-lang.org/t/what-is-the-difference-between-eq-and-partialeq/15751/2

    #[allow(non_snake_case)]
    #[inline]
    pub fn EQUALS<T: PartialEq>(left_operand: &T, right_operand: &T) -> bool {
        left_operand == right_operand
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn NOT_EQUALS<T: PartialEq>(left_operand: &T, right_operand: &T) -> bool {
        !EQUALS(left_operand, right_operand)
    }

    // partial ord so these work on floats too
    #[allow(non_snake_case)]
    #[inline]
    pub fn LESS_THAN<T: PartialOrd>(left_operand: &T, right_operand: &T) -> bool {
        left_operand < right_operand
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn GREATER_THAN<T: PartialOrd>(left_operand: &T, right_operand: &T) -> bool {
        left_operand > right_operand
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn LESS_OR_EQUALS<T: PartialOrd>(left_operand: &T, right_operand: &T) -> bool {
        left_operand <= right_operand
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn GREATER_OR_EQUALS<T: PartialOrd>(left_operand: &T, right_operand: &T) -> bool {
        left_operand >= right_operand
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn ONE_OF<T>(iterable: T, item: &T::Item) -> bool
    where
        T: IntoIterator,
        T::Item: PartialEq,
    {
        // if there is at least one of
        iterable.into_iter().any(|elm| elm == *item)
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn NONE_OF<T>(iterable: T, item: &T::Item) -> bool
    where
        T: IntoIterator,
        T::Item: PartialEq,
    {
        // if there none
        !iterable.into_iter().any(|elm| elm == *item)
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn EXACTLY_ONE_OF<T>(iterable: T, item: &T::Item) -> bool
    where
        T: IntoIterator,
        T::Item: PartialEq,
    {
        // if there is at least one of
        iterable.into_iter().filter(|elm| elm == item).count() == 1
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn AT_LEAST_ONE_OF<T>(iterable: T, item: &T::Item) -> bool
    where
        T: IntoIterator,
        T::Item: PartialEq,
    {
        // if there is at least one of
        iterable.into_iter().filter(|elm| elm == item).count() >= 1
    }

    // from python: https://github.com/openvalidation/openvalidation/pull/37/files

    #[allow(non_snake_case)]
    #[inline]
    pub fn EXISTS<T>(item: &Option<T>) -> bool {
        item.is_some()
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn NOT_EXISTS<T>(item: &Option<T>) -> bool {
        item.is_none()
    }

    #[allow(non_snake_case)]
    pub fn SUM_OF<T>(iterable: T) -> T::Item
    where
        T: IntoIterator,
        T::Item: std::iter::Sum,
    {
        iterable.into_iter().sum()
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn EMPTY<T>(vec: &[Vec<T>]) -> bool {
        vec.is_empty()
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn NOT_EMPTY<T>(vec: &[Vec<T>]) -> bool {
        !EMPTY(vec)
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn WHERE<T, F>(iterable: T, function: F) -> bool
    where
        T: IntoIterator,
        T::Item: PartialEq,
        F: Fn(&T::Item) -> bool,
    {
        // if there is at least one of
        iterable.into_iter().filter(|elm| function(elm)).count() >= 1
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn FIRST<T>(iterable: &'_ T, number: usize) -> Vec<T::Item>
    where
        T: IntoIterator + Clone,
    {
        iterable
            .clone()
            .into_iter()
            .take(number)
            .collect::<Vec<_>>()
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn LAST<T>(iterable: &'_ T, number: usize) -> Vec<T::Item>
    where
        T: DoubleEndedIterator + ExactSizeIterator + Clone,
    {
        //@TODO can we avoid the double reverse somehow?
        iterable
            .clone()
            .rev() // reverse the list
            .take(number) // and only keep the the first N elemets
            .rev() // reverse back to original order
            .collect::<Vec<T::Item>>() // and return as Vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huml_empty() {
        let model = crate::Model {
            age: 17,
            name: "Hans".into(),
        };

        let huml = huml::HumlFrameWork::new();

        let res = huml.validate(&model);

        assert!(!res.has_errors);
        assert!(res.fields.is_empty());
        assert!(res.errors.is_empty());
    }

    #[test]
    fn huml_no_error() {
        // all validation rules conform to the data
        let model = crate::Model {
            age: 18,
            name: "Hans".into(),
        };

        let mut huml = huml::HumlFrameWork::new();
        huml.append_rule(
            "age check",
            "person is 18",
            huml::RuleToggle::Enabled,
            vec!["field1"],
            &|model: &crate::Model| model.age != 18,
        );

        let res = huml.validate(&model);

        assert!(!res.has_errors);
        assert!(res.fields.is_empty());
        assert!(res.errors.is_empty());
    }

    #[test]
    fn huml_error() {
        let model = crate::Model {
            age: 18,
            name: "Hans".into(),
        };

        let mut huml = huml::HumlFrameWork::new();
        huml.append_rule(
            "age check",
            "person is 18",
            huml::RuleToggle::Enabled,
            vec!["field1"],
            &|model: &crate::Model| model.age == 18,
        );

        let res = huml.validate(&model);

        assert!(res.has_errors);
        assert_eq!(res.fields, vec!["field1"]);
        assert_eq!(
            res.errors,
            vec![huml::ValidationError {
                error: "person is 18".into(),
                fields: vec!["field1".to_string()],
            }]
        );
    }

    #[test]
    fn huml() {
        let model = crate::Model {
            age: 17,
            name: "Hans".into(),
        };

        let mut huml = huml::HumlFrameWork::new();
        huml.append_rule(
            "rule1",
            "is not old enough!",
            huml::RuleToggle::Enabled,
            vec![],
            &|model: &crate::Model| model.age < 18,
        );

        huml.append_rule(
            "rule2",
            "is not named Hans!",
            huml::RuleToggle::Enabled,
            vec![],
            &|model: &crate::Model| model.name != "Hans",
        );

        let res = huml.validate(&model);

        println!("{:?}", res);
    }

    #[test]
    fn huml_misc() {
        let model_1 = crate::Model {
            age: 100,
            name: "Luigi".into(),
        };

        let model_2 = crate::Model {
            age: 200,
            name: "Mario".into(),
        };

        let mut huml = huml::HumlFrameWork::new();

        huml.append_rule(
            "luigi age",
            "is not old enough!",
            huml::RuleToggle::Enabled,
            vec![],
            &|model_1: &crate::Model| model_1.age < 18 && model_1.name != "Luigi",
        );

        huml.append_rule(
            "mario age",
            "is not named Hans!",
            huml::RuleToggle::Enabled,
            vec![],
            &|model_2: &crate::Model| model_2.name != "Mario" && model_2.age != 200,
        );

        let res = huml.validate(&model_1);

        println!("{:?}", res);
    }
}
