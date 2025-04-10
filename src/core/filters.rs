use minijinja::Environment;
use base64::{Engine, engine::general_purpose::STANDARD};
use regex::Regex;
use sha2::Digest;

pub fn register_filters(env: &mut Environment)
{
    env.add_filter("b64encode", |s: &str| {
        STANDARD.encode(s.as_bytes())
    });

    env.add_filter("b64decode", |s: &str| -> Result<String, minijinja::Error> {
        let decoded = STANDARD.decode(s.as_bytes()).unwrap();
        Ok(String::from_utf8(decoded).unwrap())
    });

    env.add_filter("hash", |s: &str, algo: &str| -> Result<String, minijinja::Error> {
        match algo {
            "md5" => {
                let digest = md5::compute(s.as_bytes());
                Ok(format!("{:x}", digest))
            },
            "sha256" => {
                let mut hasher = sha2::Sha256::new();
                hasher.update(s.as_bytes());
                Ok(format!("{:x}", hasher.finalize()))
            },
            "sha512" => {
                let mut hasher = sha2::Sha512::new();
                hasher.update(s.as_bytes());
                Ok(format!("{:x}", hasher.finalize()))
            },
            _ => Err(minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("Invalid hash algorithm '{}', expected 'md5, 'sha256' or 'sha512'", algo)
            )),
        }
    });

    env.add_filter("regex_match", |s: &str, pattern: &str| -> Result<bool, minijinja::Error> {
        let re = Regex::new(pattern).map_err(|e| minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Invalid regex pattern: {}", e)
        ))?;
        Ok(re.is_match(s))
    });
    
    env.add_filter("regex_replace", |s: &str, pattern: &str, replacement: &str| -> Result<String, minijinja::Error> {
        let re = Regex::new(pattern).map_err(|e| minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Invalid regex pattern: {}", e)
        ))?;
        Ok(re.replace(s, replacement).to_string())
    });

    env.add_filter("regex_split", |s: &str, pattern: &str| -> Result<Vec<String>, minijinja::Error> {
        let re = Regex::new(pattern).map_err(|e| minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Invalid regex pattern: {}", e)
        ))?;
        Ok(re.split(s).map(|s| s.to_string()).collect())
    });

    env.add_filter("regex_search", |s: &str, pattern: &str| -> Result<Vec<String>, minijinja::Error> {
        let re = Regex::new(pattern).map_err(|e| minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Invalid regex pattern: {}", e)
        ))?;
        
        let caps = re.captures(s);
        if let Some(caps) = caps {
            // Skip the first capture (which is the entire match) and collect the rest
            let matches: Vec<String> = caps.iter()
                .skip(1) // Skip the full match (first capture)
                .filter_map(|m| m.map(|m| m.as_str().to_string()))
                .collect();
            Ok(matches)
        } else {
            Ok(vec![])
        }
    });

    env.add_filter("repeat", |s: &str, n: usize| s.repeat(n));
}