use error::Error;
use regex::Regex;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}, process::Command};

pub async fn obfuscate(input: &str) -> Result<String, Error> {
    let pattern = r#"<(script|body)\b[^>]*>([\s\S]*?)<\/(script|body)>"#;
    let re = Regex::new(pattern)?;

    let mut obfuscated_file = input.to_string();

    // Obfuscate front code ( js/html )
    for (_, [block_type, inner_code, _]) in re.captures_iter(&input).map(|c| c.extract()) {
        if !inner_code.is_empty() {
            let mut obfuscated_code = String::new();
            let mut new_inner_code = inner_code.to_string();
            let code_is_js = !["body", "header", "footer", "head"].contains(&block_type);

            // found code is html, obsfucate it into js
            if !code_is_js {
                new_inner_code = obfuscate_html(&inner_code);
            }

            let mut file = File::create("bins/app/src/application/templates/obfuscation.js").await?;
            file.write_all(new_inner_code.as_bytes()).await?;

            obfuscate_js(
                "bins/app/src/application/templates/obfuscation.js", 
                "bins/app/src/application/templates/obfuscation-obfuscated.js"
            ).await?;


            let mut file = File::open("bins/app/src/application/templates/obfuscation-obfuscated.js").await?;

            // Read the file contents into a string
            file.read_to_string(&mut obfuscated_code).await?;

            // Replace part of file that was obfuscated
            if code_is_js {
                obfuscated_file = obfuscated_file.replace(&inner_code, &obfuscated_code);
            }else{
                obfuscated_file = obfuscated_file.replace(&inner_code, &format!("<script>{}</script>",&obfuscated_code));
            }
        }
    }

    Ok(obfuscated_file)
}


async fn obfuscate_js(input: &str, output: &str) -> Result<(), Error> {
    let command_response = Command::new("javascript-obfuscator")
        .arg(input)
        .arg("--compact").arg("false")
        .arg("--control-flow-flattening").arg("true")
        .arg("--control-flow-flattening-threshold").arg("1")
        .arg("--dead-code-injection").arg("true")
        .arg("--dead-code-injection-threshold").arg("0.5")
        .arg("--debug-protection").arg(config::get_default("DEBUG_PROTECTION", "false"))
        .arg("--debug-protection-interval").arg("4000")
        .arg("--domain-lock").arg("localhost:8080")
        .arg("--identifier-names-generator").arg("hexadecimal")
        .arg("--numbers-to-expressions").arg("true")
        .arg("--self-defending").arg("true")
        .arg("--simplify").arg("true")
        .arg("--split-strings").arg("true")
        .arg("--split-strings-chunk-length").arg("10")
        .arg("--string-array").arg("true")
        .arg("--string-array-calls-transform").arg("true")
        .arg("--string-array-calls-transform-threshold").arg("0.4")
        .arg("--string-array-encoding").arg("rc4, base64")
        .arg("--string-array-indexes-type").arg("hexadecimal-number")
        .arg("--string-array-index-shift").arg("true")
        .arg("--string-array-rotate").arg("true")
        .arg("--string-array-shuffle").arg("true")
        .arg("--string-array-wrappers-count").arg("0.4")
        .arg("--string-array-wrappers-chained-calls").arg("true")
        .arg("--string-array-wrappers-parameters-max-count").arg("3")
        .arg("--string-array-wrappers-type").arg("function")
        .arg("--string-array-threshold").arg("0.4")
        .arg("--target").arg("browser")
        .arg("--transform-object-keys").arg("true")
        .arg("--unicode-escape-sequence").arg("true")
        .arg(output)
        .output().await?;

   
    if command_response.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&command_response.stderr);
        Err(Error::InternalError(format!("javascript-obfuscator command failed with error: {}", stderr)))
    }
}

fn obfuscate_html(html_code: &str) -> String {
    // Encode the HTML code as a JavaScript string
    let encoded_html: String = html_code.chars()
        .map(|c| format!("\\x{:02x}", c as u32))
        .collect();

    // Construct the obfuscated JavaScript code
    let obfuscated_script = format!(r#"document.write("{}");"#, encoded_html);

    obfuscated_script
}