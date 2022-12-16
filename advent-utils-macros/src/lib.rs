use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Meta};

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

/// Wraps a solution function with a timer, as well as an output formatter/logger.
/// Measures the time it takes to run the function (up to the nanosecond), and
/// prints the result to stdout.
/// TODO: Pase attrs to get the day and part number, and print that as well.
#[proc_macro_attribute]
pub fn solution(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as syn::ItemFn);
    // Actual function name so we can replace it with our wrapper
    let fn_name = input.clone().sig.ident;

    // The inner function stuff that actually computes the solution
    let inner_fn_name = format_ident!("{}_wrapper", fn_name);
    let inner_fn_block = input.block;

    // // The day parsed from the attribute
    // let attrs = parse_macro_input!(attrs as Meta);

    // println!("attr: \"{:?}\"", attrs);

    quote! {
        fn #fn_name(input: &str) -> i64 {
            let start = std::time::Instant::now();
            let result = #inner_fn_name(input);
            let elapsed = start.elapsed();

            println!("{}: {}", stringify!(#fn_name), result);
            println!(
                "Time elapsed: {}s {}ms {}µs {}ns",
                elapsed.as_secs(),
                elapsed.as_millis(),
                elapsed.as_micros(),
                elapsed.as_nanos() % 1000, // get only the last 3 digits, which are the nanoseconds
            );

            result
        }

        fn #inner_fn_name(input: &str) -> i64 {
            #inner_fn_block
        }
    }
    .into()
}
