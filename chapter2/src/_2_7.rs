mod mymessage {
    struct msg_info {
        number: i32,
        msg_body: String,
        is_sent: bool,
    }

    impl msg_info {
        fn _new(n: i32, m: String, ib: bool) -> msg_info {
            let ret: msg_info = msg_info {
                number: n,
                msg_body: m,
                is_sent: ib,
            };
            ret
        }

        fn _pt(m:&Self){
            println!("{} and {}",m.msg_body,m.is_sent);
        }

        fn _mod(m:&mut Self,s: String){
            m.msg_body = s;
        }
    }
}
