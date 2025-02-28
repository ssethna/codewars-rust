#[cfg(test)]
use l5kyu::fundamentals::*;

mod tests {
    use super::*;

    #[test]
    fn code_test() {
        let data1 =  "What do you remember? When I looked at his streaky glasses, I wanted \
         to leave him. And before that? He stole those cherries for me at midnight. We were walking \
         in the rain and I loved him. And before that? I saw him coming \
         toward me that time at the picnic, edgy, foreign.";
        let exp = "\x0bctg?.nadr d gdbW\n\x0b,i    lnis tl eh\n\x0b mtIAakietboaara\n\x0beeo nnigsoe st?t\n\
         \x0bd wsddnh lfls   \n\x0bgaaa  gtfeoeehWd\n\
         \x0bytrwbI .o rasiho\n\x0b, d e i rtev,se \n\x0b t hflnW h e  ny\n\x0bfhmioo emot Is o\n\x0boeemrvt eshh tIu\n\x0br   eehw eaiwr  \n\
         \x0beptc deea tmaelr\n\x0biihot  rtc?.naoe\n\x0bgcamhhre h  tkom\n\x0bnntiaia meHAeyke\n\x0b.i ntmiwirend em".to_string();

        let ans = code(&data1);
        println!("code is {}\n\n", ans);
        assert_eq!(ans, exp, "Testing: {}", exp);
    }

    #[test]
    fn decode_test() {
        let exp =  "And on black ground a bear-skin rug of snow. The sparks made no attempt to be the moon. ".to_string();
        let data1_sol = "\x0bet pnnacA\n\x0b  nao  kn\n\x0bmtorwrb d\n\x0boo k.ueg \n\x0bo as garo\n\x0bnbt T ron\n\x0b.etmho-u \n\x0b  eaefsnb\n\x0b\x0btmd  kdl\n\x0b\x0bhpessi a";

        let ans = decode(data1_sol);
        println!("decode is {}\n\n", ans);
        assert_eq!(ans, exp, "Testing: {}", exp);
    }
}
