use itertools::Itertools;

#[inline]
pub fn std_join() -> String {
    vec!["a", "b", "c", "d", "e", "f"].into_iter().join(" ")
}

#[inline]
pub fn join_iter_with_intersperse() -> String {
    let strs = vec!["a", "b", "c", "d", "e", "f"]
        .into_iter()
        .intersperse(" ")
        .collect::<Vec<&str>>();

    let mut string = String::with_capacity(strs.len());

    for str in strs {
        string.push_str(str)
    }

    string
}

#[inline]
pub fn join_iter_strs() -> String {
    let strs = vec!["a", "b", "c", "d", "e", "f"].into_iter();

    let mut string = String::with_capacity(strs.len() * 2);

    for str in strs {
        string.push_str(str);
        string.push(' ')
    }

    string.pop();
    string
}

#[inline]
pub fn join_iter_chars() -> String {
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f'].into_iter();

    let mut string = String::with_capacity(chars.len() * 2);

    for char in chars {
        string.push(char);
        string.push(' ')
    }

    string.pop();
    string
}

#[inline]
pub fn prefix_collect(string: String) -> String {
    let mut s = vec!['a', 'b', 'c', 'd', 'e', 'f'];

    for char in string.chars() {
        s.push(char)
    }

    s.iter().collect()
}

#[inline]
pub fn prefix_string_capacity(string: String) -> String {
    let prefix = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let mut s = String::with_capacity(prefix.len() + string.len());

    for char in prefix {
        s.push(char)
    }

    for char in string.chars() {
        s.push(char)
    }

    s
}

#[inline]
pub fn join_chars_with_collect() -> String {
    vec!['a', 'b', 'c', 'd', 'e', 'f'].iter().collect()
}

#[inline]
pub fn join_chars_with_string_capacity() -> String {
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f'];

    let mut s = String::with_capacity(chars.len());

    for char in chars {
        s.push(char)
    }

    s
}
