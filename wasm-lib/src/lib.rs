use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn extract_comments(code: &str) -> Result<JsValue, JsValue> {
    let mut is_in_single_str = false;
    let mut is_in_double_str = false;
    let mut prev_char = ' ';
    let mut is_comment = false;
    let mut is_line_comment = false;
    let mut comments: Vec<String> = vec![];
    let mut current_comment: String = String::new();
    // let mut is_afer_line_comment = false;

    for char in code.chars().into_iter() {
        // If we are in a string we do nothing about comments
        if !is_comment && !is_line_comment {
            if char == '\'' && !is_in_double_str {
                is_in_single_str = !is_in_single_str;
            } else if char == '"' && !is_in_single_str {
                is_in_double_str = !is_in_double_str;
            }
        }

        // if is_afer_line_comment && !is_line_comment && char != '/' {
        //     is_afer_line_comment = false
        // }

        if is_in_single_str || is_in_double_str {
            continue;
        }

        if prev_char == '/' {
            if char == '/' {
                if !is_line_comment {
                    is_line_comment = true;
                    continue;
                }
            } else if char == '*' {
                if !is_comment {
                    is_comment = true;
                    continue;
                }
            }
        }

        // End of line comment, we extract the comment, re intiantiate and continue
        if is_line_comment && char == '\n' {
            comments.push(current_comment.clone());
            current_comment = String::new();
            is_line_comment = false;
            // is_afer_line_comment = true;
            continue;
        }

        // End of regular comment, we remove * from the comment and extract it
        if prev_char == '*' && char == '/' && is_comment {
            current_comment.pop();
            comments.push(current_comment.clone());
            current_comment = String::new();
            is_comment = false;
            continue;
        }

        // We are currently in comment, simply concatenating each char
        if is_comment || is_line_comment {
            current_comment = current_comment + &char.to_string();
        }

        prev_char = char;
    }

    let filtered_comments: Vec<String> = comments
        .into_iter()
        .filter(|comment| !comment.is_empty())
        .collect();

    Ok(serde_wasm_bindgen::to_value(&filtered_comments)?)
}
