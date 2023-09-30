use std::{
    env::var_os,
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};

fn define_error(mut out: impl Write, name: &str, error_info: &str) {
    if name.contains(|c: char| !c.is_ascii_alphanumeric() || c.is_whitespace()) {
        panic!("The provided error type name is invalid: {}", name);
    }
    let kind = format!("{}Kind", name);

    struct ErrorVariantInfo<'a> {
        module: &'a str,
        name: &'a str,
        meaning: Vec<&'a str>,
    }

    impl<'a> ErrorVariantInfo<'a> {
        fn new(module: &'a str, name: &'a str) -> Self {
            Self {
                module,
                name,
                meaning: Vec::new(),
            }
        }
    }

    let mut variants = Vec::with_capacity(10);

    let mut current_module = "";
    let mut current_variant = None;
    for (line_number, line) in error_info.lines().enumerate() {
        let indent = if let Some(indent) = line.find(|v| v != ' ') {
            indent
        } else {
            continue;
        };

        let rest = &line[indent..];
        if indent % 4 != 0 {
            panic!(
                "incorrectly indention for error \"{}\", content of affected line is \"{}\" (line {})",
                name, line, line_number
            );
        }
        let indent = indent / 4;
        match indent {
            0 if !rest.is_empty() => {
                // Error module
                current_module = rest;
            }
            1 => {
                // Error variant
                if rest.starts_with("0x") {
                    // Error variant's HRESULT value.
                    // TODO: maybe insert a static assert that the winapi constant
                    // matches this value.
                    continue;
                }
                if rest.contains(' ') {
                    panic!(
                        "error variant can't contain whitespace, error type \"{}\", line {}, variant name: {}",
                        name, line_number, rest
                    );
                }
                if current_module.is_empty() {
                    panic!(
                        "no module specified for error variant \"{}\" of \"{}\" at line {}",
                        rest, name, line_number
                    )
                }
                if let Some(previous) =
                    current_variant.replace(ErrorVariantInfo::new(current_module, rest))
                {
                    variants.push(previous);
                }
            }
            2 => {
                // Meaning
                if let Some(current) = &mut current_variant {
                    current.meaning.push(rest);
                } else {
                    panic!(
                        "no error variant defined for documentation of \"{}\" at line {}",
                        name, line_number
                    );
                }
            }
            _ => panic!(
                "too much indentation for \"{}\" at line {}",
                name, line_number
            ),
        }
    }
    variants.extend(current_variant.take());
    let to_kind = variants
        .iter()
        .map(|v| format!("{}::{} => {}::{},\n", v.module, v.name, kind, v.name))
        .collect::<String>();
    let enum_variants = variants
        .iter()
        .map(|v| {
            let mut text = String::new();
            for (index, doc) in v.meaning.iter().enumerate() {
                if index != 0 {
                    text.push_str("///\n");
                }
                text.push_str("/// ");
                text.push_str(doc);
                text.push('\n');
            }
            text.push_str(v.name);
            text.push_str(",\n");
            text
        })
        .collect::<String>();

    write!(
        out,
        r###"
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct {name}(pub HRESULT);
impl {name} {{
    pub fn kind(self) -> {kind} {{
        match self.0 {{
            {to_kind}
            _ => {kind}::OTHER,
        }}
    }}
}}
impl fmt::Display for {name} {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        write!(f, "{{}}: {{:?}} (HRESULT: {{:#X}})", stringify!({name}), self.kind(), self.0)
    }}
}}
impl fmt::Debug for {name} {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        struct AsHex(HRESULT);
        impl fmt::Debug for AsHex {{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                write!(f, "{{:#X}}", self.0)
            }}
        }}
        f.debug_struct(stringify!({name}))
            .field("HRESULT", &self.0)
            .field("HRESULT-AsHex", &AsHex(self.0))
            .field("kind", &self.kind())
            .finish()
    }}
}}
impl StdError for {name} {{}}
impl core::convert::From<HRESULT> for {name} {{
    fn from(value: HRESULT) -> Self {{
        Self(value)
    }}
}}
impl core::convert::From<{name}> for HRESULT {{
    fn from(value: {name}) -> Self {{
        value.0
    }}
}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum {kind} {{
    {enum_variants}
    OTHER,
}}
"###,
        name = name,
        kind = kind,
        to_kind = to_kind,
        enum_variants = enum_variants,
    )
    .unwrap();
}

fn index_of(sub: &str, parent: &str) -> usize {
    let sub_num = sub.as_ptr() as usize;
    let parent_num = parent.as_ptr() as usize;
    sub_num
        .checked_sub(parent_num)
        .filter(|&ix| ix <= parent.len())
        .expect("substring was not inside the provided parent string")
}

fn main() {
    let out_dir = PathBuf::from(var_os("OUT_DIR").expect("out directory for build script"));
    let out_path = out_dir.join("errors.rs");
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&out_path)
            .unwrap_or_else(|_| {
                panic!("failed to create output file at \"{}\"", out_path.display())
            }),
    );

    static ERRORS: &str = include_str!("./errors.md");

    let mut name_iter = ERRORS
        .lines()
        .skip(1) // First Markdown Header
        .filter(|line| !line.is_empty() && !line.starts_with(|c: char| c.is_whitespace()))
        .peekable();

    while let Some(name) = name_iter.next() {
        let start_ix = index_of(name, ERRORS) + name.len();
        let error_info = match name_iter.peek() {
            Some(next) => &ERRORS[start_ix..index_of(next, ERRORS)],
            None => &ERRORS[start_ix..],
        };

        if name.is_empty() {
            continue;
        }
        if name.starts_with(|c: char| c.is_whitespace()) {
            panic!("Error name should be defined at start of a line");
        }
        let name = name.replace(' ', "") + "Error";

        let error_info = format!("\nall_errors\n{error_info}");

        define_error(&mut out, &name, &error_info);
    }

    out.into_inner().unwrap().sync_all().unwrap();
}
