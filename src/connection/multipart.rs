#[derive(Debug)]
pub enum Type {
    File(String),
    Value,
}

#[derive(Debug)]
pub struct Data {
    pub _type: Type,
    pub name: String,
    pub content: Vec<u8>,
}

pub fn parse(content: &[u8]) -> Vec<Data> {
    let mut results = Vec::new();

    // Récupère la boundary
    let mut lines = content.split(|&b| b == b'\n');
    let boundary_line = match lines.next() {
        Some(line) if line.starts_with(b"--") => String::from_utf8_lossy(line).trim().to_string(),
        _ => return results,
    };
    let boundary = format!("--{}", boundary_line.trim_start_matches("--"));
    let boundary_bytes = boundary.as_bytes();

    // Découpe les parties
    let mut start = 0;
    while let Some(index) = content[start..]
        .windows(boundary_bytes.len())
        .position(|w| w == boundary_bytes)
    {
        let begin = start + index + boundary_bytes.len();
        let next_start = content[begin..]
            .windows(boundary_bytes.len())
            .position(|w| w == boundary_bytes)
            .map(|i| begin + i)
            .unwrap_or(content.len());

        let part = &content[begin..next_start];
        start = next_start;

        let part = part.strip_prefix(b"\r\n").unwrap_or(part);

        let split_pos = part.windows(4).position(|w| w == b"\r\n\r\n");
        let (headers_bytes, body) = match split_pos {
            Some(pos) => (&part[..pos], &part[(pos + 4)..]),
            None => continue,
        };

        let headers_str = match std::str::from_utf8(headers_bytes) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut name = String::new();
        let mut filename: Option<String> = None;

        for line in headers_str.lines() {
            if line.starts_with("Content-Disposition") {
                for item in line.split(';') {
                    let item = item.trim();
                    if item.starts_with("name=") {
                        name = item.trim_start_matches("name=").trim_matches('"').to_string();
                    } else if item.starts_with("filename=") {
                        filename = Some(item.trim_start_matches("filename=").trim_matches('"').to_string());
                    }
                }
            }
        }

        let data_type = match filename {
            Some(fname) => Type::File(fname),
            None => Type::Value,
        };

        results.push(Data {
            _type: data_type,
            name,
            content: body.to_vec(),
        });
    }

    results
}
