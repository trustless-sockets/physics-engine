use indoc::indoc;

// Testing by parsing code and printing its display, making sure we get back the formatted code.
#[test]
fn format_test() {
    let parser = sierra::ProgramParser::new();
    assert_eq!(
        parser
            .parse(indoc! {"
            // Some comment.
            type ConcreteTypeId =  TypeId; // Other comment.
            type ConcreteTypeId  = TypeId<arg>;
            type  ConcreteTypeId = TypeId<arg1, 4>;
            type [123] = TypeId<[12],  4>;
            ext CalleeId = ExtensionId ;
            // Additional comment.
            ext OtherCalleeId = ExtensionId <arg, 4>;
            ext [5642] = ExtensionId<[22 ], 4>;
            ext CallFunction = Call<&Function>;
            callee() -> ();
            callee(arg1) -> (res1);
            callee( arg1, arg2) -> ( res1, res2);
            callee() { 5( ) };
            callee(arg1 , arg2) { fallthrough() 7(res1 ) 5(res1, res2) };
            [12345]([12]) { 2([37]) fallthrough() };
            return();
            return ( r);
            return(r1 , r2);
            return ([1], [45], [0]);

            Name@5() -> ();
            Other@3([5]: T1) -> (T2);
            [343]@3([5]: [6343]) -> ([341]);
        "},)
            .map(|p| p.to_string()),
        Ok(indoc! {"
            type ConcreteTypeId = TypeId;
            type ConcreteTypeId = TypeId<arg>;
            type ConcreteTypeId = TypeId<arg1, 4>;
            type [123] = TypeId<[12], 4>;

            ext CalleeId = ExtensionId;
            ext OtherCalleeId = ExtensionId<arg, 4>;
            ext [5642] = ExtensionId<[22], 4>;
            ext CallFunction = Call<&Function>;

            callee() -> ();
            callee(arg1) -> (res1);
            callee(arg1, arg2) -> (res1, res2);
            callee() { 5() };
            callee(arg1, arg2) { fallthrough() 7(res1) 5(res1, res2) };
            [12345]([12]) { 2([37]) fallthrough() };
            return();
            return(r);
            return(r1, r2);
            return([1], [45], [0]);

            Name@5() -> ();
            Other@3([5]: T1) -> (T2);
            [343]@3([5]: [6343]) -> ([341]);

"}
        .to_string())
    );
}