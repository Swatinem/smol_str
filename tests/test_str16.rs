use std::sync::Arc;

use proptest::{prop_assert, prop_assert_eq, proptest};

use smol_buf::Str16;

#[test]
#[cfg(target_pointer_width = "64")]
fn str16_is_smol() {
    assert_eq!(
        ::std::mem::size_of::<Str16>(),
        ::std::mem::size_of::<Arc<str>>(),
    );
}

#[test]
fn assert_traits() {
    fn f<T: Send + Sync + ::std::fmt::Debug + Clone>() {}
    f::<Str16>();
}

#[test]
fn conversions() {
    let s: Str16 = "Hello, World!".into();
    let s: String = s.into();
    assert_eq!(s, "Hello, World!");

    let s: Str16 = Arc::<str>::from("Hello, World!").into();
    let s: Arc<str> = s.into();
    assert_eq!(s.as_ref(), "Hello, World!");
}

#[test]
fn const_fn_ctor() {
    const EMPTY: Str16 = Str16::new_inline("");
    const A: Str16 = Str16::new_inline("A");
    const HELLO: Str16 = Str16::new_inline("HELLO");
    const LONG: Str16 = Str16::new_inline("ABCDEFGHIZKLMNO");

    assert_eq!(EMPTY, Str16::from(""));
    assert_eq!(A, Str16::from("A"));
    assert_eq!(HELLO, Str16::from("HELLO"));
    assert_eq!(LONG, Str16::from("ABCDEFGHIZKLMNO"));
}

fn check_props(std_str: &str, smol: Str16) -> Result<(), proptest::test_runner::TestCaseError> {
    prop_assert_eq!(smol.as_str(), std_str);
    prop_assert_eq!(smol.len(), std_str.len());
    prop_assert_eq!(smol.is_empty(), std_str.is_empty());
    if smol.len() <= 15 {
        prop_assert!(!smol.is_heap_allocated());
    }
    Ok(())
}

proptest! {
    #[test]
    #[cfg_attr(miri, ignore)]
    fn roundtrip(s: String) {
        check_props(s.as_str(), Str16::new(s.clone()))?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn roundtrip_spaces(s in r"( )*") {
        check_props(s.as_str(), Str16::new(s.clone()))?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn roundtrip_newlines(s in r"\n*") {
        check_props(s.as_str(), Str16::new(s.clone()))?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn roundtrip_ws(s in r"( |\n)*") {
        check_props(s.as_str(), Str16::new(s.clone()))?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn from_string_iter(slices in proptest::collection::vec(".*", 1..100)) {
        let string: String = slices.iter().map(|x| x.as_str()).collect();
        let smol: Str16 = slices.into_iter().collect();
        check_props(string.as_str(), smol)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn from_str_iter(slices in proptest::collection::vec(".*", 1..100)) {
        let string: String = slices.iter().map(|x| x.as_str()).collect();
        let smol: Str16 = slices.iter().collect();
        check_props(string.as_str(), smol)?;
    }
}

#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    struct SmolStrStruct {
        pub(crate) s: Str16,
        pub(crate) vec: Vec<Str16>,
        pub(crate) map: HashMap<Str16, Str16>,
    }

    #[test]
    fn test_serde() {
        let s = Str16::new("Hello, World");
        let s = serde_json::to_string(&s).unwrap();
        assert_eq!(s, "\"Hello, World\"");
        let s: Str16 = serde_json::from_str(&s).unwrap();
        assert_eq!(s, "Hello, World");
    }

    #[test]
    fn test_serde_reader() {
        let s = Str16::new("Hello, World");
        let s = serde_json::to_string(&s).unwrap();
        assert_eq!(s, "\"Hello, World\"");
        let s: Str16 = serde_json::from_reader(std::io::Cursor::new(s)).unwrap();
        assert_eq!(s, "Hello, World");
    }

    #[test]
    fn test_serde_struct() {
        let mut map = HashMap::new();
        map.insert(Str16::new("a"), Str16::new("ohno"));
        let struct_ = SmolStrStruct {
            s: Str16::new("Hello, World"),
            vec: vec![Str16::new("Hello, World"), Str16::new("Hello, World")],
            map,
        };
        let s = serde_json::to_string(&struct_).unwrap();
        let _new_struct: SmolStrStruct = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_serde_struct_reader() {
        let mut map = HashMap::new();
        map.insert(Str16::new("a"), Str16::new("ohno"));
        let struct_ = SmolStrStruct {
            s: Str16::new("Hello, World"),
            vec: vec![Str16::new("Hello, World"), Str16::new("Hello, World")],
            map,
        };
        let s = serde_json::to_string(&struct_).unwrap();
        let _new_struct: SmolStrStruct = serde_json::from_reader(std::io::Cursor::new(s)).unwrap();
    }

    #[test]
    fn test_serde_hashmap() {
        let mut map = HashMap::new();
        map.insert(Str16::new("a"), Str16::new("ohno"));
        let s = serde_json::to_string(&map).unwrap();
        let _s: HashMap<Str16, Str16> = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_serde_hashmap_reader() {
        let mut map = HashMap::new();
        map.insert(Str16::new("a"), Str16::new("ohno"));
        let s = serde_json::to_string(&map).unwrap();
        let _s: HashMap<Str16, Str16> = serde_json::from_reader(std::io::Cursor::new(s)).unwrap();
    }

    #[test]
    fn test_serde_vec() {
        let vec = vec![Str16::new(""), Str16::new("b")];
        let s = serde_json::to_string(&vec).unwrap();
        let _s: Vec<Str16> = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_serde_vec_reader() {
        let vec = vec![Str16::new(""), Str16::new("b")];
        let s = serde_json::to_string(&vec).unwrap();
        let _s: Vec<Str16> = serde_json::from_reader(std::io::Cursor::new(s)).unwrap();
    }
}

#[test]
fn test_search_in_hashmap() {
    let mut m = ::std::collections::HashMap::<Str16, i32>::new();
    m.insert("aaa".into(), 17);
    assert_eq!(17, *m.get("aaa").unwrap());
}

#[test]
fn test_from_char_iterator() {
    let examples = [
        // Simple keyword-like strings
        ("if", false),
        ("for", false),
        ("impl", false),
        // Strings containing two-byte characters
        ("パーティーへ行かないか", true),
        ("パーティーへ行か", true),
        ("パーティー", false),
        ("和製漢語", false),
        ("部落格", false),
        ("사회과학원 어학연구소", true),
        // String containing diverse characters
        ("表ポあA鷗ŒéＢ逍Üßªąñ丂㐀𠀀", true),
    ];
    for (raw, is_heap) in &examples {
        let s: Str16 = raw.chars().collect();
        assert_eq!(s.as_str(), *raw);
        assert_eq!(s.is_heap_allocated(), *is_heap, "{raw}");
    }
    // String which has too many characters to even consider inlining: Chars::size_hint uses
    // (`len` + 3) / 4. With `len` = 89, this results in 23, so `from_iter` will immediately
    // heap allocate
    let raw: String = "a".repeat(23 * 4 + 1);
    let s: Str16 = raw.chars().collect();
    assert_eq!(s.as_str(), raw);
    assert!(s.is_heap_allocated());
}

#[test]
fn test_bad_size_hint_char_iter() {
    struct BadSizeHint<I>(I);

    impl<T, I: Iterator<Item = T>> Iterator for BadSizeHint<I> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1024, None)
        }
    }

    let data = "testing";
    let collected: Str16 = BadSizeHint(data.chars()).collect();
    let new = Str16::new(data);

    // If we try to use the type of the string (inline/heap) to quickly test for equality, we need to ensure
    // `collected` is inline allocated instead
    assert!(!collected.is_heap_allocated());
    assert!(!new.is_heap_allocated());
    assert_eq!(new, collected);
}
