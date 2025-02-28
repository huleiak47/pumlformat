use anyhow::Result;
use clap::Parser;
use fancy_regex::Regex;
use log::debug;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

/// Regex patterns for block start
const BLOCK_START_PATTERNS: &[&str] = &[
    r"^(if|while|fork|package|namespace|note(?!.*:)|group|loop|repeat|alt|opt|critical|else|elseif).*|\w+.*{$",
];

/// Regex patterns for block end
const BLOCK_END_PATTERNS: &[&str] = &[
    r"^(endif|endwhile|endfork|end\s*note|endgroup|end|stop|endrepeat|endpackage|endnamespace|else|elseif)\b.*$|}$",
];

const MULTI_BLANK_LINE_PATTERN: &str = r"(^|\n)(\s*\n){2,}";

/// Format PlantUML code
fn format_plantuml(text: &str, indent_size: usize) -> Result<String> {
    let block_start_re = Regex::new(&format!("(?i){}", BLOCK_START_PATTERNS.join("|"))).unwrap();
    let block_end_re = Regex::new(&format!("(?i){}", BLOCK_END_PATTERNS.join("|"))).unwrap();
    let multi_blank_re = Regex::new(MULTI_BLANK_LINE_PATTERN).unwrap();

    // Replace multiple blank lines with a single newline
    let new_text = multi_blank_re.replace(text.trim(), "$1\n").to_string();

    let lines = new_text.lines().collect::<Vec<_>>();
    let mut formatted_lines = Vec::with_capacity(lines.len());
    let mut indent_level: usize = 0;

    for line in lines {
        let stripped = line.trim();

        if stripped.is_empty() {
            formatted_lines.push(stripped.to_string());
            continue;
        }

        // handle comment line
        if stripped.starts_with("'") {
            let indent = " ".repeat(indent_level * indent_size);
            debug!("line: {}, indent_level: {}", line, indent_level);
            formatted_lines.push(format!("{}{}", indent, stripped));
            continue;
        }

        // Handle block end (decrease indent first)
        let is_end = block_end_re.is_match(stripped)?;
        if is_end {
            indent_level = indent_level.saturating_sub(1);
        }

        // Apply indentation
        debug!("line: {}, indent_level: {}", line, indent_level);
        let indent = " ".repeat(indent_level * indent_size);
        formatted_lines.push(format!("{}{}", indent, stripped));

        // Handle block start (increase indent after)
        if block_start_re.is_match(stripped)? {
            indent_level += 1;
        }
    }

    // Ensure the output ends with a newline
    let mut ret = formatted_lines.join("\n");
    if !ret.ends_with("\n") {
        ret.push('\n');
    }

    Ok(ret)
}

/// Command line arguments structure
#[derive(Parser)]
#[command(
    name = "pumlformat",
    version,
    about = "Formats PlantUML code with consistent indentation and spacing"
)]
struct CliArgs {
    /// Input file (default: stdin)
    input: Option<PathBuf>,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Number of spaces for indentation
    #[arg(short, long, default_value_t = 4, value_parser = clap::value_parser!(usize))]
    indent: usize,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = CliArgs::parse();

    // Read input content
    let mut content = String::new();
    match &args.input {
        Some(path) => File::open(path)?.read_to_string(&mut content)?,
        None => io::stdin().read_to_string(&mut content)?,
    };

    // Format code
    let formatted = format_plantuml(&content, args.indent);

    // Write output
    match &args.output {
        Some(path) => File::create(path)?.write_all(formatted?.as_bytes())?,
        None => io::stdout().write_all(formatted?.as_bytes())?,
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"
    
    @startuml
actor User
@enduml



"#;
    const OUTPUT1: &str = r#"@startuml
actor User
@enduml
"#;
    #[test]
    fn test_1() {
        assert_eq!(format_plantuml(INPUT1, 4).unwrap(), OUTPUT1);
    }

    const INPUT2: &str = r#"@startuml
alt
A-->B:TODO
end
@enduml
"#;

    const OUTPUT2: &str = r#"@startuml
alt
    A-->B:TODO
end
@enduml
"#;
    #[test]
    fn test_2() {
        assert_eq!(format_plantuml(INPUT2, 4).unwrap(), OUTPUT2);
    }

    const INPUT3: &str = r#"@startuml
alt
A-->B:TODO
     
  
       
end
@enduml"#;
    const OUTPUT3: &str = r#"@startuml
alt
    A-->B:TODO

end
@enduml
"#;

    #[test]
    fn test_3() {
        env_logger::init();
        assert_eq!(format_plantuml(INPUT3, 4).unwrap(), OUTPUT3);
    }

    const INPUT4: &str = r#"@startuml

' class Test <<Interface>>{
'     + test()
' }

@enduml"#;
    const OUTPUT4: &str = r#"@startuml

' class Test <<Interface>>{
'     + test()
' }

@enduml
"#;

    #[test]
    fn test_4() {
        assert_eq!(format_plantuml(INPUT4, 4).unwrap(), OUTPUT4);
    }
}
