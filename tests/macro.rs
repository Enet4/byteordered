#[macro_use]
extern crate byteordered;

use byteordered::Endianness;

#[test]
fn test_macro_one_read() {
    let x: &[u8] = &[1, 2, 1, 2];
    let mut c = x;
    let v = with_order!(&mut c, Endianness::le_iff(2 + 2 == 4), |data| {
        let v = data.read_u16().unwrap();
        assert_eq!(v, 513);
        v
    });
    assert_eq!(v, 513);

    let v = with_order!(&mut c, Endianness::Big, |data| {
        let v = data.read_u16().unwrap();
        assert_eq!(v, 258);
        v
    });
    assert_eq!(v, 258);
}

#[test]
fn test_macro_one_read_2() {
    let v = with_order!([16, 1].as_ref(), Endianness::Little, |data| {
        data.read_u16().unwrap()
    });
    assert_eq!(v, 272);

    let v = with_order!(([16, 1].as_ref()), Endianness::Little, |data| {
        data.read_u16().unwrap()
    });
    assert_eq!(v, 272);
}

#[test]
fn test_macro_multi_pipe() {
    let x: &[u8] = &[1, 2, 3, 4];
    let mut sink = Vec::new(); 
    let mut c = x;

    with_order!((&mut c, &mut sink), Endianness::le_iff(2 + 2 == 4), |input, output| {
        let v = input.read_u16().unwrap();
        output.write_u16(v + 10).unwrap();
    });

    with_order!((&mut c, &mut sink), Endianness::Big, |input, output| {
        let v = input.read_u16().unwrap();
        output.write_u16(v + 100).unwrap();
    });
    
    assert_eq!(sink, vec![11, 2, 3, 104]);
}
