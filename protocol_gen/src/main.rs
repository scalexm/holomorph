use docopt::Docopt;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};

fn to_snake_case(s: &str) -> String {
    let s = s
        .replace("GID", "Gid")
        .replace("UID", "Uid")
        .replace("AVA", "Ava")
        .replace("PA", "Pa")
        .replace("PM", "Pm")
        .replace("NPC", "Npc");

    let mut result = String::new();
    for (i, c) in s.char_indices() {
        if c.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

const USAGE: &'static str = "
Protocol generation tool.

Usage:
  gen <input> <output>

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Deserialize)]
struct Args {
    arg_input: String,
    arg_output: String,
}

fn main() -> std::io::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if Path::new(&args.arg_output).exists() {
        println!("output directory already exists");
        std::process::exit(1);
    }

    Module::new(
        args.arg_input.clone().into(),
        args.arg_output.clone().into(),
        "messages".into(),
    )
    .read_dir()?;

    Module::new(
        args.arg_input.into(),
        args.arg_output.into(),
        "types".into(),
    )
    .read_dir()?;

    Ok(())
}

struct Module {
    input: PathBuf,
    output: PathBuf,
    directory: PathBuf,
    imports: HashSet<String>,
}

impl Module {
    fn new(input: PathBuf, output: PathBuf, directory: PathBuf) -> Self {
        Self {
            input,
            output,
            directory,
            imports: HashSet::new(),
        }
    }

    fn read_dir(&mut self) -> std::io::Result<()> {
        let components: Vec<_> = self
            .directory
            .components()
            .map(|c| to_snake_case(&c.as_os_str().to_string_lossy()))
            .collect();
        let output_dir = self.output.join(components.join("/"));
        std::fs::create_dir_all(&output_dir)?;

        let mut sub_modules = vec![];
        let mut structs = vec![];
        for entry in std::fs::read_dir(&self.input.join(&self.directory))? {
            let entry = entry?;
            let sub_path = entry.path();

            if sub_path.is_dir() {
                let mod_name = to_snake_case(
                    &sub_path
                        .file_name()
                        .expect("cannot parse file name")
                        .to_string_lossy(),
                );
                sub_modules.push(format!("pub mod {};\n", mod_name));

                Module::new(
                    self.input.clone(),
                    self.output.clone(),
                    self.directory.join(sub_path.file_name().unwrap()),
                )
                .read_dir()?;
            } else if sub_path.to_string_lossy().ends_with(".as") {
                if sub_path.ends_with("NetworkDataContainerMessage.as") {
                    continue;
                }
                structs.push(self.read_file(&sub_path)?);
            }
        }

        let mut output = sub_modules.join("");
        if !structs.is_empty() {
            if !output.is_empty() {
                output += "\n";
            }
            output += "use protocol_derive::{Encode, Decode};\n";
            let imports: Vec<_> = self.imports.iter().map(|s| s.as_str()).collect();
            output += &imports.join("");
            output += "\n";
            output += &structs.join("\n");
        }

        let mut output_file = std::fs::File::create(output_dir.join("mod.rs"))?;
        output_file.write_all(output.as_bytes())?;

        Ok(())
    }

    fn read_file(&mut self, path: &Path) -> std::io::Result<String> {
        lazy_static! {
            static ref CLASS: Regex =
                Regex::new(r"public class (\w+) (?:extends|implements) (\w+)").unwrap();
            static ref PROTOCOL_ID: Regex = Regex::new(r"protocolId:uint = (\d+)").unwrap();
            static ref BODY: Regex = Regex::new(include!("body.regex")).unwrap();
        }

        let contents = std::fs::read_to_string(path)?;

        let matches = CLASS.captures(&contents);
        let (class, base_class) = match matches.as_ref() {
            Some(matches) => (&matches[1], &matches[2]),
            None => panic!("could not parse class or base class in {:?}", path),
        };

        let matches = PROTOCOL_ID.captures(&contents);
        let protocol_id = match matches.as_ref() {
            Some(matches) => &matches[1],
            None => panic!("could not parse protocol id in {:?}", path),
        };

        let matches = BODY.captures(&contents);
        let body = match matches.as_ref() {
            Some(matches) => &matches[1],
            None => panic!("could not get serialize function body in {:?}", path),
        };

        let mut fb = FunctionBody::new(&contents, &mut self.imports, &base_class);
        let fields: Vec<_> = body
            .lines()
            .filter_map(|line| fb.read_body_line(line))
            .collect();

        let mut output = "#[derive(Clone, PartialEq, Debug, Encode, Decode)]\n".to_string();
        output += &format!("#[protocol(id = {})]\n", protocol_id);
        output += &format!("pub struct {}<'a> {{\n", class);
        for f in fields {
            if !f.attrs.is_empty() {
                output += &format!("\t#[protocol({})]\n", f.attrs.join(","));
            }
            output += &format!("\tpub {}: {},\n", f.name, f.typ);
        }
        if !fb.has_lifetime {
            output += "\tpub _phantom: std::marker::PhantomData<&'a ()>,\n";
        }
        output += "}\n";
        Ok(output)
    }
}

struct FunctionBody<'a> {
    next_length_is_dynamic: bool,
    has_lifetime: bool,
    file: &'a str,
    imports: &'a mut HashSet<String>,
    base_class: &'a str,
}

impl<'a> FunctionBody<'a> {
    fn new(file: &'a str, imports: &'a mut HashSet<String>, base_class: &'a str) -> Self {
        Self {
            next_length_is_dynamic: false,
            has_lifetime: false,
            file,
            imports,
            base_class,
        }
    }

    fn get_import(&self, typ: &str) -> String {
        let mut import = "use crate::".to_string();

        let regex = Regex::new(&format!(r"dofus\.network\.([\w || \.]+)\.{};", typ)).unwrap();
        let matches = match regex.captures(self.file) {
            Some(matches) => matches,
            None => return String::new(),
        };

        for c in matches[1].split(".") {
            import += &to_snake_case(c);
            import += "::";
        }
        import += typ;
        import += ";\n";

        import
    }

    fn read_body_line(&mut self, line: &str) -> Option<Field> {
        lazy_static! {
            static ref PRIMITIVE: Regex = Regex::new(r"write(\w+)\(this\.(\w+)\)").unwrap();
            static ref FLAG: Regex = Regex::new(r"setFlag\(\w+,\d,this\.(\w+)").unwrap();
            static ref LENGTH: Regex = Regex::new(r"write(\w+)\(this\.\w+\.length").unwrap();
            static ref VECTOR: Regex = Regex::new(r"write(\w+)\(this\.(\w+)\[").unwrap();
            static ref CLASS: Regex = Regex::new(r"this\.(\w+)\.serializeAs_(\w+)").unwrap();
            static ref POLYMORPHIC: Regex = Regex::new(r"this\.(\w+)\.serialize").unwrap();
            static ref VECTOR_CLASS: Regex =
                Regex::new(r"\(this\.(\w+)\[_\w+\] as (\w+)\).serializeAs_").unwrap();
            static ref VECTOR_POLYMORPHIC_CLASS: Regex =
                Regex::new(r"\(this\.(\w+)\[_\w+\] as (\w+)\).s").unwrap();
        }

        let mut attrs = vec![];
        let (name, typ);

        if line.contains("super") {
            name = "base".to_string();
            typ = format!("{}<'a>", self.base_class);
            self.imports.insert(self.get_import(self.base_class));
            self.has_lifetime = true;
        } else if let Some(matches) = PRIMITIVE.captures(line) {
            name = matches[2].to_string();
            let (mapped, var) = self.map_type(&name, &matches[1]);
            typ = mapped.to_string();
            if var {
                attrs.push("var");
            }
        } else if let Some(matches) = FLAG.captures(line) {
            name = matches[1].to_string();
            typ = "bool".to_string();
            attrs.push("flag");
        } else if let Some(matches) = LENGTH.captures(line) {
            let (mapped, var) = self.map_type("", &matches[1]);
            if !((mapped == "u16" && !var) || (mapped == "u32" && var)) {
                panic!("vec length supposed to be `u16` or `Var(u32)`: {}", line);
            }
            self.next_length_is_dynamic = var;
            return None;
        } else if let Some(matches) = VECTOR.captures(line) {
            name = matches[2].to_string();
            let (mapped, var) = self.map_type(&name, &matches[1]);

            if mapped == "u8" || mapped == "i8" {
                typ = format!("&'a [{}]", mapped);
                if self.next_length_is_dynamic {
                    attrs.push("var");
                }
            } else {
                typ = format!("std::borrow::Cow<'a, [{}]>", mapped);
                if var {
                    attrs.push("var_contents");
                }
                if self.next_length_is_dynamic {
                    attrs.push("var_length");
                }
            }
            self.has_lifetime = true;
        } else if let Some(matches) = CLASS.captures(line) {
            name = matches[1].to_string();
            typ = format!("{}<'a>", &matches[2]);
            self.imports.insert(self.get_import(&matches[2]));
            self.has_lifetime = true;
        } else if let Some(matches) = POLYMORPHIC.captures(line) {
            name = matches[1].to_string();
            let find_type = Regex::new(&format!("var {}:(\\w+)", name)).unwrap();
            let matches = match find_type.captures(&self.file) {
                Some(matches) => matches,
                None => panic!("could not find type of field {}", name),
            };
            typ = format!("{}Variant<'a>", &matches[1]);
            self.imports
                .insert(format!("use crate::variants::{}Variant;\n", &matches[1]));
            self.has_lifetime = true;
        } else if let Some(matches) = VECTOR_CLASS.captures(line) {
            name = matches[1].to_string();
            typ = format!("std::borrow::Cow<'a, [{}<'a>]>", &matches[2]);
            self.imports.insert(self.get_import(&matches[2]));
            if self.next_length_is_dynamic {
                attrs.push("var_length");
            }
            self.has_lifetime = true;
        } else if let Some(matches) = VECTOR_POLYMORPHIC_CLASS.captures(line) {
            name = matches[1].to_string();
            typ = format!("std::borrow::Cow<'a, [{}Variant<'a>]>", &matches[2]);
            if self.next_length_is_dynamic {
                attrs.push("var_length");
            }
            self.imports
                .insert(format!("use crate::variants::{}Variant;\n", &matches[2]));
            self.has_lifetime = true;
        } else {
            return None;
        }

        let name = to_snake_case(&name);
        Some(Field {
            name: match name.as_str() {
                "self" => "self_".to_string(),
                "type" => "type_".to_string(),
                _ => name,
            },
            typ: typ,
            attrs,
        })
    }

    fn map_type(&mut self, name: &str, typ: &str) -> (&str, bool) {
        let mut var = false;
        let mapped_type = match typ {
            "UTF" => {
                self.has_lifetime = true;
                "&'a str"
            }

            "Int" => {
                if self.file.contains(&format!("var {}:int", name)) {
                    "i32"
                } else if self.file.contains(&format!("var {}:Vector.<int>", name)) {
                    "i32"
                } else {
                    "u32"
                }
            }

            "UnsignedInt" => "u32",

            "Short" => {
                if self.file.contains(&format!("var {}:int", name)) {
                    "i16"
                } else if self.file.contains(&format!("var {}:Vector.<int>", name)) {
                    "i16"
                } else {
                    "u16"
                }
            }

            "UnsignedShort" => "u16",

            "Byte" => {
                if self.file.contains(&format!("var {}:int", name)) {
                    "i8"
                } else if self.file.contains(&format!("var {}:Vector.<int>", name)) {
                    "i8"
                } else {
                    "u8"
                }
            }

            "Boolean" => "bool",

            "Double" => "f64",

            "Float" => "f32",

            "VarInt" => {
                var = true;
                if self.file.contains(&format!("var {}:int", name)) {
                    "i32"
                } else if self.file.contains(&format!("var {}:Vector.<int>", name)) {
                    "i32"
                } else {
                    "u32"
                }
            }

            "VarShort" => {
                var = true;
                if self.file.contains(&format!("var {}:int", name)) {
                    "i16"
                } else if self.file.contains(&format!("var {}:Vector.<int>", name)) {
                    "i16"
                } else {
                    "u16"
                }
            }

            "VarLong" => {
                var = true;
                if self
                    .file
                    .contains(&format!("this.{} = input.readVarLong()", name))
                {
                    "i64"
                } else {
                    "u64"
                }
            }

            _ => panic!("unknown type: {}", typ),
        };
        (mapped_type, var)
    }
}

struct Field {
    name: String,
    typ: String,
    attrs: Vec<&'static str>,
}
