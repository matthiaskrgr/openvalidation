//@TODO move the following code into a separate file so that MyData is imported
// external.rs will be generated by OV-cli based on parameters that are passed to it.
// it describes the data model that the function contained in a Rule is run on
// use mock external.rs so that we can build

// deny unsafe code
#![deny(unsafe_code)]

pub struct Model {
    age: u8,
    name: String,
}

pub mod huml {

    #[derive(Default)]
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
                let validation_failed: bool = (rule.function)(&data);
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
                fields: fields.into_iter().map(|s| s.into()).collect(),
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
    #[inline(always)]
    pub fn EQUALS<T: PartialEq>(left_operand: T, right_operand: T) -> bool {
        left_operand == right_operand
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn NOT_EQUALS<T: PartialEq>(left_operand: T, right_operand: T) -> bool {
        !EQUALS(left_operand, right_operand)
    }

    // partial ord so these work on floats too
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn LESS_THAN<T: PartialOrd>(left_operand: T, right_operand: T) -> bool {
        left_operand < right_operand
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn GREATER_THAN<T: PartialOrd>(left_operand: T, right_operand: T) -> bool {
        left_operand > right_operand
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn LESS_OR_EQUALS<T: PartialOrd>(left_operand: T, right_operand: T) -> bool {
        left_operand <= right_operand
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn GREATER_OR_EQUALS<T: PartialOrd>(left_operand: T, right_operand: T) -> bool {
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
    #[inline(always)]
    pub fn EXISTS<T>(item: Option<T>) -> bool {
        item.is_some()
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn NOT_EXISTS<T>(item: Option<T>) -> bool {
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
    #[inline(always)]
    pub fn EMPTY<T>(vec: Vec<T>) -> bool {
        vec.is_empty()
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn NOT_EMPTY<T>(vec: Vec<T>) -> bool {
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

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
/*
#[allow(non_snake_case)]
pub mod _HUML {
    // a function with static lifetime that operates on MyData and returns bool
    // this will be used to validate the data

    // Die Methode (callable), die zu einer rule gehört gibt true zurück
    // wenn gemäß dieser Regel ein Fehler vorliegt
    // und false otherwise

    //pub trait Callable: for<'r> Fn(&'r crate::MyData) -> bool {}

    #[derive(Default, Clone, Debug)]
    pub struct HUMLFramework<'a> {
        // list of rules
        rules: Vec<ValidationRule<'a>>,
    }

    impl<'a> HUMLFramework<'a> {
        pub fn new() -> Self {
            Self { rules: Vec::new() }
        }

        // add a rule to the HUML object
        pub fn append_rule<T: Fn(&crate::MyData) -> bool>(
            &mut self,
            name: String, // @TODO use AsRef<str> here
            fields: Vec<String>,
            error: String,
            disabled: bool,
            function: &'a T,
            data: &'a crate::MyData,
        ) {
            let rule = ValidationRule::new(name, fields, error, disabled, function, data);
            self.push_rule(rule);
        }

        // takes rule object and pushes it into huml object, consuming the rule
        pub fn push_rule(&mut self, rule: ValidationRule<'a>) {
            self.rules.push(rule);
        }

        // returns a list of only enabled rules
        pub fn enabled_rules(&'a self) -> Vec<&'a ValidationRule<'a>> {
            self.rules()
                .iter()
                .filter(|rule| rule.is_enabled())
                .collect::<Vec<&ValidationRule>>()
        }

        // SILAS:
        /// Die validate Methode gibt ein Objekt zurück, dass Informationen über alle gefundenen Fehler enthält.
        /// // Vec<Result<(),ValidationError>>
        // Methodennamen und -signaturen, sowie Datenstrukturen (z.B. das Rückgabeobjekt der validate Methode) sollen sich in allen Sprachen so exakt wie möglich entsprechen.

        #[must_use]
        pub fn validate(&mut self) -> ValidationSummary {
            //- In einem validate-Aufruf wird in einer Schleife die function der
            //  Regel auf Daten angewendet und gegebenenfalls die
            //  Fehlernachricht etc. dem Validierungsergebnis hinzugefügt.
            // gefundener Fehler heißt die function gibt true zurück

            let mut summary = ValidationSummary {
                errors: vec![],
                fields: vec![],
                has_errors: false,
            };

            for rule in self.enabled_rules().iter() {
                // apply the rule to the data
                if rule.validate() {
                    // gefundener fehler heißt die funktion gibt true zurück

                    let error = rule.errormsg.clone();
                    let fields = rule.fields.clone();
                    let val_error = ValidationError::new(error, fields.clone());
                    // add both of them to the summary
                    summary.errors.push(val_error);
                    summary.fields.extend(fields);
                } else {
                    // do  nothing
                }
            }
            summary.has_errors = !summary.errors.is_empty();
            // dedupe fields of the summary
            summary.fields.sort();
            summary.fields.dedup();
            summary
        }

        pub fn get_rule_by_name(&'a self, name: String) -> Option<&'a ValidationRule> {
            // do we need to verify that rule name are uniqe?
            // should we return a list of rules with matching name?
            self.rules.iter().find(|rule| rule.name == name)
        }

        // returns a reference to the contained rules
        pub fn rules(&'a self) -> &'a Vec<ValidationRule<'a>> {
            &self.rules
        }
    } // impl HumlFrameWork

    // a validation rule
    #[derive(Clone)]
    pub struct ValidationRule<'a> {
        // most of the fields contain rule metadata
        name: String,
        // json keys
        fields: Vec<String>,
        // error returned in case the rule does not validate
        errormsg: String,
        // rules might be disabled
        disabled: bool,
        // a function that is run
        function: &'a dyn Fn(&crate::MyData) -> bool,
        // the data that the rule is checked against
        data: &'a crate::MyData,
    }

    // we can't derive(Debug) because closures don't impl it
    // so do it manually
    impl<'a> std::fmt::Debug for ValidationRule<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            #[allow(clippy::write_literal)]
            write!(f, "ValidationRule {{ name: {:?}, fields: {:?}, errormsg: {:?}, disabled: {:?}, function: {}, data: {} }}", self.name, self.fields, self.errormsg, self.disabled, "(omitted):&'a dyn Fn(&crate::MyData) -> bool", "(omitted)&'a crate::MyData")
        }
    }

    impl<'a> ValidationRule<'a> {
        pub fn new<T: Fn(&crate::MyData) -> bool>(
            name: String,
            fields: Vec<String>,
            errormsg: String,
            disabled: bool,
            function: &'a T,
            data: &'a crate::MyData,
        ) -> Self {
            Self {
                name,
                fields,
                errormsg,
                disabled,
                function,
                data,
            }
        }

        pub fn name(&self) -> String {
            self.name.clone()
        }

        pub fn fields(&self) -> Vec<String> {
            self.fields.clone()
        }

        pub fn errormsg(&self) -> String {
            self.errormsg.clone()
        }

        pub fn is_disabled(&self) -> bool {
            self.disabled
        }

        pub fn is_enabled(&self) -> bool {
            !self.is_disabled()
        }

        pub fn data(&self) -> &crate::MyData {
            // we don't neccessarily know if the data impls Clone so borrow it
            &self.data
        }

        // return the closure by reference. Do we want something else here?
        pub fn function(&self) -> &dyn Fn(&crate::MyData) -> bool {
            self.function
        }

        // run the function of a rule against its data
        pub fn validate(&self) -> bool {
            self.function()(&self.data)
        }
    } // impl ValidationRule

    #[derive(Debug, Clone, Default)]
    pub struct ValidationSummary {
        errors: Vec<ValidationError>, // not deduped
        fields: Vec<String>,          // deduped
        has_errors: bool,
    }

    // the result of a validation
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct ValidationError {
        error: String,
        fields: Vec<String>,
    }

    impl ValidationError {
        pub fn new(error: String, fields: Vec<String>) -> Self {
            Self { error, fields }
        }

        pub fn fields(&self) -> Vec<String> {
            self.fields.clone()
        }

        pub fn error(&self) -> String {
            self.error.clone()
        }
    }

    /*
        pub struct ValidationSummary {
            errors: Vec<ValidationError>,
            fields: Vec<String>,
            has_errors: bool,
        }

        // by using the newtype pattern we auto-inherit all vec methods, yay
        type ValidationSummary = Vec<Result<(), ValidationError>>;

        pub fn get_errors(summary: &ValidationSummary) -> Vec<ValidationError> {
            summary
                .iter()
                .cloned()
                .filter(|result| result.is_err())
                .map(|r| r.unwrap_err())
                .collect()
        }


    */


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_huml() {
            use crate::MyData;

            let schema: MyData = MyData {
                age: 17,
                name: String::from("Hans"),
            };

            let mut huml: HUMLFramework = HUMLFramework::new();

            let rule = ValidationRule::new(
                String::from("rule1"),
                vec![String::from("field 1")],
                String::from("person is not old enough!"),
                false,
                &|x: &crate::MyData| x.age == 17,
                &schema,
            );

            huml.push_rule(rule);

            let rule_2 = ValidationRule::new(
                String::from("rule1"),
                vec![String::from("field 1")],
                String::from("BAD"),
                false,
                &|x: &crate::MyData| x.name == "Hans",
                &schema,
            );

            huml.push_rule(rule_2);

            // a rule that does not validate successfully
            let rule_3_false = ValidationRule::new(
                String::from("rule1"),
                vec![String::from("field 1")],
                String::from("person is not old enough!"),
                false,
                // is this   true/false correct v  ?
                &|x: &crate::MyData| x.age != 1,
                &schema,
            );

            huml.push_rule(rule_3_false);

            let result = huml.validate();
            println!("huml debug: {:?}", huml); // it works, yay!
            println!("\n\nresult: {:?}", result); // it works, yay!
        }

        #[test]
        fn equals() {
            // test numbers
            assert!(EQUALS(1, 1));
            assert!(EQUALS(-3, -3));
            assert!(!EQUALS(1, 2));

            // test vectors
            let mut v = Vec::new();
            v.push(1);
            v.push(3);
            assert!(EQUALS(vec![1, 3], v));
            assert!(EQUALS("hello", &"olleh".chars().rev().collect::<String>()));

            // test structs
            #[derive(Clone, PartialEq, Eq)]
            struct A {
                s: String,
                n: u8,
            }
            let a = A {
                s: "a".into(),
                n: 5,
            };
            assert!(EQUALS(a.clone(), a));

            let a = A {
                s: "a".into(),
                n: 5,
            };
            // modify b a bit to make sure they are not equal
            let mut b = a.clone();
            b.s = "oh no".into();
            b.n += 4;
            assert!(!EQUALS(a, b));
        }

        #[test]
        fn not_equals() {
            // test numbers
            assert!(!NOT_EQUALS(1, 1));
            assert!(!NOT_EQUALS(-3, -3));
            assert!(NOT_EQUALS(1, 2));

            // test vectors
            let mut v = Vec::new();
            v.push(1);
            v.push(3);
            assert!(!NOT_EQUALS(vec![1, 3], v));
            assert!(!NOT_EQUALS(
                "hello",
                &"olleh".chars().rev().collect::<String>()
            ));

            // test structs
            #[derive(Clone, PartialEq, Eq)]
            struct A {
                s: String,
                n: u8,
            }
            let a = A {
                s: "a".into(),
                n: 5,
            };
            assert!(!NOT_EQUALS(a.clone(), a));

            let a = A {
                s: "a".into(),
                n: 5,
            };
            // modify b a bit to make sure they are not equal
            let mut b = a.clone();
            b.s = "oh no".into();
            b.n += 4;
            assert!(NOT_EQUALS(a, b));
        }

        #[test]
        fn less_than() {
            // test numbers
            assert!(LESS_THAN(1, 3));
            assert!(LESS_THAN(0, 1));
            assert!(!LESS_THAN(0, 0));
            assert!(LESS_THAN(-1, 0));
            assert!(!LESS_THAN(std::i32::MAX, 3));
            assert!(LESS_THAN(99, std::i32::MAX));
            assert!(!LESS_THAN(std::i32::MAX, std::i32::MAX));
            assert!(LESS_THAN(std::u8::MIN, std::u8::MAX));
            assert!(LESS_THAN(std::u8::MIN, 1));
            assert!(!LESS_THAN(std::u8::MIN, 0));
            assert!(!LESS_THAN(std::u8::MAX, 255));
            assert!(LESS_THAN(std::u8::MAX - 1, 255));
            assert!(LESS_THAN(0.1, 10.0));
            assert!(LESS_THAN(0.00000001, 0.0001));
            assert!(!LESS_THAN(0.001, 0.001));
            assert!(LESS_THAN(std::u128::MIN, std::u128::MAX));
        }

        #[test]
        fn greater_than() {
            // test numbers
            assert!(!GREATER_THAN(1, 3));
            assert!(!GREATER_THAN(0, 1));
            assert!(!GREATER_THAN(0, 0));
            assert!(!GREATER_THAN(-1, 0));
            assert!(GREATER_THAN(std::i32::MAX, 3));
            assert!(!GREATER_THAN(99, std::i32::MAX));
            assert!(!GREATER_THAN(std::i32::MAX, std::i32::MAX));
            assert!(!GREATER_THAN(std::u8::MIN, std::u8::MAX));
            assert!(!GREATER_THAN(std::u8::MIN, 1));
            assert!(!GREATER_THAN(std::u8::MIN, 0));
            assert!(!GREATER_THAN(std::u8::MAX, 255)); // equals
            assert!(!GREATER_THAN(std::u8::MAX - 1, 255));
            assert!(!GREATER_THAN(0.1, 10.0));
            assert!(!GREATER_THAN(0.00000001, 0.0001));
            assert!(!GREATER_THAN(0.001, 0.001));
            assert!(!GREATER_THAN(std::u128::MIN, std::u128::MAX));
        }

        #[test]
        fn less_or_equals() {
            // test numbers
            assert!(LESS_OR_EQUALS(1, 3));
            assert!(LESS_OR_EQUALS(0, 1));
            assert!(LESS_OR_EQUALS(0, 0));
            assert!(LESS_OR_EQUALS(-1, 0));
            assert!(!LESS_OR_EQUALS(std::i32::MAX, 3));
            assert!(LESS_OR_EQUALS(99, std::i32::MAX));
            assert!(LESS_OR_EQUALS(std::i32::MAX, std::i32::MAX));
            assert!(LESS_OR_EQUALS(std::u8::MIN, std::u8::MAX));
            assert!(LESS_OR_EQUALS(std::u8::MIN, 1));
            assert!(LESS_OR_EQUALS(std::u8::MIN, 0));
            assert!(LESS_OR_EQUALS(std::u8::MAX, 255));
            assert!(LESS_OR_EQUALS(std::u8::MAX - 1, 255));
            assert!(LESS_OR_EQUALS(0.1, 10.0));
            assert!(LESS_OR_EQUALS(0.00000001, 0.0001));
            assert!(LESS_OR_EQUALS(0.001, 0.001));
            assert!(!LESS_OR_EQUALS(0.1, 0.0));
            assert!(LESS_OR_EQUALS(std::u128::MIN, std::u128::MAX));
        }

        #[test]
        fn greater_or_equals() {
            // test numbers
            assert!(!GREATER_OR_EQUALS(1, 3));
            assert!(!GREATER_OR_EQUALS(0, 1));
            assert!(GREATER_OR_EQUALS(0, 0));
            assert!(!GREATER_OR_EQUALS(-1, 0));
            assert!(GREATER_OR_EQUALS(std::i32::MAX, 3));
            assert!(!GREATER_OR_EQUALS(99, std::i32::MAX));
            assert!(GREATER_OR_EQUALS(std::i32::MAX, std::i32::MAX));
            assert!(!GREATER_OR_EQUALS(std::u8::MIN, std::u8::MAX));
            assert!(!GREATER_OR_EQUALS(std::u8::MIN, 1));
            assert!(GREATER_OR_EQUALS(std::u8::MIN, 0));
            assert!(GREATER_OR_EQUALS(std::u8::MAX, 255)); // equals
            assert!(!GREATER_OR_EQUALS(std::u8::MAX - 1, 255));
            assert!(!GREATER_OR_EQUALS(0.1, 10.0));
            assert!(!GREATER_OR_EQUALS(0.00000001, 0.0001));
            assert!(GREATER_OR_EQUALS(0.001, 0.001));
            assert!(!GREATER_OR_EQUALS(std::u128::MIN, std::u128::MAX));
        }

        #[test]
        fn one_of() {
            // empty vec
            assert!(!ONE_OF(vec![], &()));
            assert!(ONE_OF(vec![1], &(1)));
            assert!(!ONE_OF(vec![], &(1)));
            assert!(!ONE_OF(vec![1], &(0)));

            // vec from range
            (0..=1000_i32)
                .for_each(|x: i32| assert!(ONE_OF((0..=1000_i32).collect::<Vec<i32>>(), &x)));

            (0..=100_i32)
                .for_each(|x: i32| assert!(!ONE_OF((101..=1000_i32).collect::<Vec<i32>>(), &x)));

            assert!(ONE_OF(vec![1, 2, 3, 4, 5], &(1)));
            assert!(ONE_OF(vec!["a", "c", "f", "1", "z"], &("z")));
            assert!(ONE_OF(vec!["hello", "é", "this", "", "isatest"], &("")));
            assert!(!ONE_OF(vec!["hello", "é", "this", "", "isatest"], &("ö")));
            assert!(!ONE_OF(
                vec!["hello", "é", "this", "", "isatest"],
                &("veryverylong")
            ));
            assert!(ONE_OF(vec![1, 1, 1, 1, 1], &(1)));
            assert!(!ONE_OF(vec![1, 1, 1, 1, 1], &(2)));

            // nested vecs/structs
            assert!(ONE_OF(
                vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![],
                    vec![9, 4, 234, 1],
                    vec![4, 2]
                ],
                &(vec![4, 2])
            ));
        }

        #[test]
        fn exists() {
            assert!(EXISTS(Some(())));
            let none: Option<()> = None;
            assert!(!EXISTS(none));
            assert!(EXISTS(Some("hello")));
            assert!(EXISTS(Some(Some(vec![()]))));
            assert!(EXISTS(Some(none)));
        }

        #[test]
        fn not_exists() {
            assert!(!NOT_EXISTS(Some(())));
            let none: Option<()> = None;
            assert!(NOT_EXISTS(none));
            assert!(!NOT_EXISTS(Some("hello")));
            assert!(!NOT_EXISTS(Some(Some(vec![()]))));
            assert!(!NOT_EXISTS(Some(none)));
        }

        #[test]
        fn empty() {
            assert!(EMPTY(Vec::<()>::new()));
            assert!(!EMPTY(vec![0]));
            assert!(!EMPTY(vec![1, 2, 3]));
            assert!(!EMPTY(vec![vec![()]]));
        }

        #[test]
        fn not_empty() {
            assert!(!NOT_EMPTY(Vec::<()>::new()));
            assert!(NOT_EMPTY(vec![0]));
            assert!(NOT_EMPTY(vec![1, 2, 3]));
            assert!(NOT_EMPTY(vec![vec![()]]));
        }

        #[test]
        fn first() {
            // vec
            let v = vec![1, 2, 3, 4];
            assert!(FIRST(&v, 0).is_empty());
            assert_eq!(FIRST(&v, 1), vec![1]);
            assert_eq!(FIRST(&v, 2), vec![1, 2]);
            assert_eq!(FIRST(&v, 3), vec![1, 2, 3]);
            assert_eq!(FIRST(&v, 4), vec![1, 2, 3, 4]);
            assert_eq!(FIRST(&v, 5), vec![1, 2, 3, 4]);
            // empty vec
            assert_eq!(FIRST(&Vec::<u8>::new(), 0), vec![]);
            assert_eq!(FIRST(&Vec::<u8>::new(), 1000), vec![]);
            // slice // can we get these to work somehow?
            //assert_eq!(FIRST(&vec![1,2,3].as_slice(), 5), &vec![1, 2, 3].as_slice());
            //assert_eq!(FIRST(&vec![1,2,3][..], 5), vec![1, 2, 3][..]);
            // string
            //let s = String::from("Hello World");
            //assert_eq!(FIRST(&s.chars(), 5), "Hello".chars().collect::<Vec<_>>());
        }
    }
}

*/
