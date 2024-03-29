use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, Element, Event, HtmlInputElement, InputEvent};

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
pub fn instantiate_rust_listener() -> Result<(), JsValue> {
    let window = window().expect("window should exist");
    let document = window.document().expect("document should exist");
    let input = document
        .query_selector(".block__rust .input")
        .expect("input textarea should exist")
        .unwrap();

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let input_event = event.dyn_into::<InputEvent>().unwrap();
        let target = input_event.target().unwrap();
        let input_element: HtmlInputElement = target.unchecked_into();
        let value = input_element.value();
        console::time_with_label("RustExtractCode");
        let comments = extract_comments(&value).expect("Should return vector of comments");
        console::time_end_with_label("RustExtractCode");
        let output = document
            .query_selector(".block__rust .output__list")
            .unwrap()
            .unwrap();

        console::time_with_label("RustManipulateDom");
        output.set_inner_html("");
        for comment in comments {
            let li: Element = document.create_element("li").unwrap().dyn_into().unwrap();
            li.set_attribute("class", "output__item").unwrap();
            li.set_inner_html(&comment);
            output.append_child(&li).unwrap();
        }
        console::time_end_with_label("RustManipulateDom");
    }) as Box<dyn FnMut(_)>);

    input
        .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();

    Ok(())
}

// #[wasm_bindgen]
pub fn extract_comments(code: &str) -> Result<Vec<String>, ()> {
    let mut is_in_single_str = false;
    let mut is_in_double_str = false;
    let mut prev_char = ' ';
    let mut is_comment = false;
    let mut is_line_comment = false;
    let mut comments: Vec<String> = vec![];
    let mut current_comment: String = String::new();

    for char in code.chars().into_iter() {
        // If we are in a string we do nothing about comments
        if !is_comment && !is_line_comment {
            if char == '\'' && !is_in_double_str {
                is_in_single_str = !is_in_single_str;
            } else if char == '"' && !is_in_single_str {
                is_in_double_str = !is_in_double_str;
            }
        }

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
            println!("current_comment: {:?}", current_comment);
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

    Ok(filtered_comments)
    // Ok(serde_wasm_bindgen::to_value(&filtered_comments)?)
}
