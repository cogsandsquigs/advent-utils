use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Meta, MetaNameValue, NestedMeta};

/// Wraps a solution function with a timer, as well as an output formatter/logger.
/// Measures the time it takes to run the function (up to the nanosecond), and
/// prints the result to stdout.
/// TODO: Pase attrs to get the day and part number, and print that as well.
#[proc_macro_attribute]
pub fn solution(args: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item);
    let args: AttributeArgs = parse_macro_input!(args);

    // Actual function name so we can replace it with our wrapper
    let fn_name = input.clone().sig.ident;
    let fn_args = input.clone().sig.inputs;

    // The inner function stuff that actually computes the solution
    let inner_fn_name = format_ident!("{}_inner", fn_name);
    let inner_fn_block = input.block;
    let inner_fn_return = input.sig.output;

    let day = if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = &args[0] {
        if path.is_ident("day") {
            lit
        } else {
            panic!("expected name-value style attribute with name \"day\"");
        }
    } else {
        panic!("Expected a name-value style attribute");
    };

    let part = if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = &args[1]
    {
        if path.is_ident("part") {
            lit
        } else {
            panic!("expected name-value style attribute with name \"part\"");
        }
    } else {
        panic!("Expected a name-value style attribute");
    };

    quote! {
        fn #fn_name() -> String {
            use advent_utils::files::read;
            let is_test = std::env::args().nth(1) == Some("test".to_string());
            let input = read(&format!("day-{}/input{}.txt", #day, if is_test { ".test" } else { "" })).unwrap();

            let start = std::time::Instant::now();
            let result = #inner_fn_name(&input);
            let elapsed = start.elapsed();

            println!("Day {}, part {} solution: {}", #day, #part, result);
            println!(
                "Time elapsed: {}s {}ms {}µs {}ns",
                elapsed.as_secs(),
                elapsed.as_millis() % 1000, // get only the last 3 digits, which are the milliseconds
                elapsed.as_micros() % 1000, // get only the last 3 digits, which are the microseconds
                elapsed.as_nanos() % 1000, // get only the last 3 digits, which are the nanoseconds
            );

            result.to_string()
        }

        fn #inner_fn_name(#fn_args) #inner_fn_return {
            #inner_fn_block
        }
    }
    .into()
}
