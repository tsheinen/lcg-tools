# lcg-tools

i'm putting all the neat LCG utilities I use in this library so i can stop copy/pasting python functions from old writeups every time i have to break an lcg for a CTF.

Currently it can solve an LCG forward and backwards and derive parameters when provided a collection of values



```rust
    #[test]
    fn it_generates_numbers_correctly_forward_and_backwards() {
        let mut rand = LCG {
            state: 32760.to_bigint().unwrap(),
            a: 5039.to_bigint().unwrap(),
            c: 76581.to_bigint().unwrap(),
            m: 479001599.to_bigint().unwrap(),
        };

        let mut forward = (0..10).map(|_| rand.next()).collect::<Vec<_>>();

        assert_eq!(
            forward,
            vec![
                165154221.to_bigint().unwrap(),
                186418737.to_bigint().unwrap(),
                41956685.to_bigint().unwrap(),
                180107137.to_bigint().unwrap(),
                330911418.to_bigint().unwrap(),
                58145764.to_bigint().unwrap(),
                326604388.to_bigint().unwrap(),
                389095148.to_bigint().unwrap(),
                96982646.to_bigint().unwrap(),
                113998795.to_bigint().unwrap()
            ]
        );
        forward.reverse();
        rand.next();
        assert_eq!(
            (0..10).filter_map(|_| rand.prev()).collect::<Vec<_>>(),
            forward
        );
    }

    #[test]
    fn it_cracks_lcg_correctly() {
        let mut rand = LCG {
            state: 32760.to_bigint().unwrap(),
            a: 5039.to_bigint().unwrap(),
            c: 0.to_bigint().unwrap(),
            m: 479001599.to_bigint().unwrap(),
        };

        let cracked_lcg = crack_lcg(
            (0..10)
                .map(|_| rand.next().to_isize().unwrap())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        assert_eq!(rand, cracked_lcg);

    }
```
