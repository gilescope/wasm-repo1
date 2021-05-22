use wasm_bindgen_test::wasm_bindgen_test;

 #[wasm_bindgen_test]
#[test]
fn test() {
    let s: &[u8] = &[1];
    let masked = s[0];
    // The below cond should evaluate to false but,
    // what seems to happen is even though it evaluates to false,
    // the if takes the wrong branch!!!!
    let cond = s[0] as u32 == 0; //should evaluate to false.

    // Comment out line below to pass the test on --target=wasm32-unknown-unknown ????
    let m1 = masked << 0;

    assert_eq!(Err(()), if cond {
        Ok(3333)
    } else {
        //Should go here.
        Err(())
    });
}