use phper::{
    arrays::Array,
    functions::{call, Argument},
    modules::Module,
    values::Val,
};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_functions_call",
        |_: &mut [Val]| -> phper::Result<()> {
            let mut arr = Array::new();
            arr.insert("a", Val::new(1));
            arr.insert("b", Val::new(2));
            let ret = call("array_sum", &mut [Val::new(arr)])?;
            assert_eq!(ret.as_long()?, 3);
            Ok(())
        },
        vec![],
    );

    module.add_function(
        "integrate_functions_call_callable",
        |arguments: &mut [Val]| {
            if let [head, tail @ ..] = arguments {
                Ok::<_, phper::Error>(head.call(tail)?)
            } else {
                unreachable!()
            }
        },
        vec![Argument::by_val("fn")],
    );
}
