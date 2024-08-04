use fields::{AllFields, Fields};

#[test]
fn simple() {
    #[derive(Debug, Default, Clone, PartialEq, Fields)]
    struct Test {
        valid: bool,
        id: u32,
        name: String,
    }

    let mut test = Test::default();

    test.set(TestField::Valid(true));
    assert_eq!(test.valid, true);

    test.set_all([TestField::Id(123), TestField::Name("foo".into())]);
    assert_eq!(
        test,
        Test {
            valid: true,
            id: 123,
            name: "foo".into()
        }
    );
}

#[test]
fn generic() {
    #[derive(Debug, Clone, PartialEq, Fields)]
    struct Generic<'a, S, T>
    where
        S: Clone + Into<String>,
    {
        name: S,
        value: &'a T,
    }

    let mut generic = Generic::<&str, u32> {
        name: "foo",
        value: &123,
    };

    generic.set(GenericField::<&str, u32>::Name("bar"));
    assert_eq!(generic.name, "bar");
}

#[test]
fn tuple() {
    use std::f32::consts::PI;

    #[derive(Debug, Default, Clone, PartialEq, Fields)]
    struct Tuple(i32, f32);

    let mut tuple = Tuple::default();

    tuple.set(TupleField::Field1(PI));
    assert_eq!(tuple.1, PI);
}

#[test]
fn attributes() {
    #[derive(Debug, Default, Fields)]
    #[fields(name = "MyFields", visibility(pub, pub(crate)))]
    #[allow(unused)]
    struct Test {
        private: i32,
        pub public: i32,
        pub(crate) restricted: i32,
    }

    let mut test = Test::default();

    test.set(MyFields::Public(123));
    assert_eq!(test.public, 123);

    test.set(MyFields::Restricted(123));
    assert_eq!(test.restricted, 123);
}

#[test]
fn derives() {
    #[derive(Debug, Default, Fields)]
    #[fields(derive(Debug, Clone, PartialEq))]
    struct Test {
        name: String,
    }

    let field = TestField::Name("foo".into());
    let cloned = field.clone();
    assert_eq!(field, cloned);
}

#[test]
fn all() {
    #[derive(Debug, Default, Fields, AllFields)]
    #[fields(derive(Debug, Clone, PartialEq))]
    struct Test {
        valid: bool,
        id: u32,
        name: String,
    }

    let test = Test::default();
    let all = test.all().collect::<Vec<_>>();
    assert!(all.iter().any(|field| matches!(field, TestField::Valid(_))));
    assert!(all.iter().any(|field| matches!(field, TestField::Id(_))));
    assert!(all.iter().any(|field| matches!(field, TestField::Name(_))));
}
