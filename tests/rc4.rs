#[cfg(all(test, feature = "rc4"))]
mod test {
    use cryptographer::rc4;

    struct TestCase {
        key: Vec<u8>,
        keystream: Vec<u8>,
    }

    impl TestCase {
        fn new(key: Vec<u8>, keystream: Vec<u8>) -> Self {
            Self { key, keystream }
        }
    }

    fn golden() -> Vec<TestCase> {
        vec![
            TestCase::new(
                vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
                vec![0x74, 0x94, 0xc2, 0xe7, 0x10, 0x4b, 0x08, 0x79],
            ),
            TestCase::new(
                vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xde, 0x18, 0x89, 0x41, 0xa3, 0x37, 0x5d, 0x3a],
            ),
            TestCase::new(
                vec![0xef, 0x01, 0x23, 0x45],
                vec![0xd6, 0xa1, 0x41, 0xa7, 0xec, 0x3c, 0x38, 0xdf, 0xbd, 0x61],
            ),
            TestCase::new(
                vec![0x4b, 0x65, 0x79],
                vec![0xeb, 0x9f, 0x77, 0x81, 0xb7, 0x34, 0xca, 0x72, 0xa7, 0x19],
            ),
            TestCase::new(
                vec![0x57, 0x69, 0x6b, 0x69],
                vec![0x60, 0x44, 0xdb, 0x6d, 0x41, 0xb7],
            ),
            TestCase::new(
                vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![
                    0xde, 0x18, 0x89, 0x41, 0xa3, 0x37, 0x5d, 0x3a, 0x8a, 0x06, 0x1e, 0x67, 0x57,
                    0x6e, 0x92, 0x6d, 0xc7, 0x1a, 0x7f, 0xa3, 0xf0, 0xcc, 0xeb, 0x97, 0x45, 0x2b,
                    0x4d, 0x32, 0x27, 0x96, 0x5f, 0x9e, 0xa8, 0xcc, 0x75, 0x07, 0x6d, 0x9f, 0xb9,
                    0xc5, 0x41, 0x7a, 0xa5, 0xcb, 0x30, 0xfc, 0x22, 0x19, 0x8b, 0x34, 0x98, 0x2d,
                    0xbb, 0x62, 0x9e, 0xc0, 0x4b, 0x4f, 0x8b, 0x05, 0xa0, 0x71, 0x08, 0x50, 0x92,
                    0xa0, 0xc3, 0x58, 0x4a, 0x48, 0xe4, 0xa3, 0x0a, 0x39, 0x7b, 0x8a, 0xcd, 0x1d,
                    0x00, 0x9e, 0xc8, 0x7d, 0x68, 0x11, 0xf2, 0x2c, 0xf4, 0x9c, 0xa3, 0xe5, 0x93,
                    0x54, 0xb9, 0x45, 0x15, 0x35, 0xa2, 0x18, 0x7a, 0x86, 0x42, 0x6c, 0xca, 0x7d,
                    0x5e, 0x82, 0x3e, 0xba, 0x00, 0x44, 0x12, 0x67, 0x12, 0x57, 0xb8, 0xd8, 0x60,
                    0xae, 0x4c, 0xbd, 0x4c, 0x49, 0x06, 0xbb, 0xc5, 0x35, 0xef, 0xe1, 0x58, 0x7f,
                    0x08, 0xdb, 0x33, 0x95, 0x5c, 0xdb, 0xcb, 0xad, 0x9b, 0x10, 0xf5, 0x3f, 0xc4,
                    0xe5, 0x2c, 0x59, 0x15, 0x65, 0x51, 0x84, 0x87, 0xfe, 0x08, 0x4d, 0x0e, 0x3f,
                    0x03, 0xde, 0xbc, 0xc9, 0xda, 0x1c, 0xe9, 0x0d, 0x08, 0x5c, 0x2d, 0x8a, 0x19,
                    0xd8, 0x37, 0x30, 0x86, 0x16, 0x36, 0x92, 0x14, 0x2b, 0xd8, 0xfc, 0x5d, 0x7a,
                    0x73, 0x49, 0x6a, 0x8e, 0x59, 0xee, 0x7e, 0xcf, 0x6b, 0x94, 0x06, 0x63, 0xf4,
                    0xa6, 0xbe, 0xe6, 0x5b, 0xd2, 0xc8, 0x5c, 0x46, 0x98, 0x6c, 0x1b, 0xef, 0x34,
                    0x90, 0xd3, 0x7b, 0x38, 0xda, 0x85, 0xd3, 0x2e, 0x97, 0x39, 0xcb, 0x23, 0x4a,
                    0x2b, 0xe7, 0x40,
                ],
            ),
        ]
    }

    #[test]
    fn test_golden() {
        let goldens = golden();

        for (i, v) in goldens.iter().enumerate() {
            let mut data = vec![0u8; v.keystream.len()];
            for j in 0..data.len() {
                data[j] = j as u8;
            }

            let mut expect = vec![0u8; v.keystream.len()];
            for j in 0..expect.len() {
                expect[j] = (j as u8) ^ v.keystream[j];
            }

            for size in 1..(v.keystream.len() + 1) {
                let mut c = rc4::new(v.key.as_slice())
                    .expect(format!("#{} failed to rc4::new", i).as_str());

                let mut offset = 0usize;
                loop {
                    if offset >= v.keystream.len() {
                        break;
                    }

                    let mut n = v.keystream.len() - offset;
                    if n > size {
                        n = size;
                    }

                    let src = &data[offset..(offset + n)];
                    let expect = &expect[offset..(offset + n)];

                    let mut dst = vec![0u8; src.len()];

                    c.xor_key_stream(dst.as_mut_slice(), src);
                    for (j, v) in expect.iter().enumerate() {
                        assert_eq!(
                            *v,
                            dst[j],
                            "#{}@[{}..{}]: mismatch at byte {}: have {:x?}, want {:x?}",
                            i,
                            offset,
                            offset + n,
                            j,
                            dst.as_slice(),
                            expect
                        );
                    }

                    offset += n;
                }
            }
        }
    }

    #[test]
    fn block() {
        let goldens = golden();

        let mut c1a = rc4::new(goldens[0].key.as_slice()).expect("failed to new c1a");
        let mut c1b = rc4::new(goldens[1].key.as_slice()).expect("failed to new c1b");
        let mut data1 = vec![0u8; 1 << 20];

        for i in 0..data1.len() {
            let x = vec![data1[i]];
            c1a.xor_key_stream(&mut data1[i..(i + 1)], &x);

            let x = vec![data1[i]];
            c1b.xor_key_stream(&mut data1[i..(i + 1)], &x);
        }

        let mut c2a = rc4::new(goldens[0].key.as_slice()).expect("failed to new c2a");
        let mut c2b = rc4::new(goldens[1].key.as_slice()).expect("failed to new c2b");
        let mut data2 = vec![0u8; 1 << 20];

        let x = data2.clone();
        c2a.xor_key_stream(&mut data2, &x);
        let x = data2.clone();
        c2b.xor_key_stream(&mut data2, &x);

        assert_eq!(data1, data2);
    }
}
