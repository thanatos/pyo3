// Copyright (c) 2017-present PyO3 Project and Contributors
use syn;
use quote::{Tokens, ToTokens};


pub fn print_err(msg: String, t: Tokens) {
    println!("Error: {} in '{}'", msg, t.to_string());
}

pub fn for_err_msg(i: &ToTokens) -> String {
    let mut tokens = Tokens::new();

    i.to_tokens(&mut tokens);
    tokens.as_str().to_string()
}

pub fn clean_comment(comment: &String) -> String {
    /* NOTE:
     * We're still not correctly handling some cases.
     * e.g., "//// foo" is a normal comment, not a doc comment starting with a
     * slash, according to the reference.
     * Similarly, "/*** comment */" is a normal comment.
     */
    if comment.starts_with("/// ") {
        comment[4..].to_string()
    } else if comment.starts_with("///") {
        comment[3..].to_string()
    } else if comment.starts_with("/**") && comment.ends_with("*/") {
        let mut first_line_was_removed = false;

        let mut comment_interior = &comment[3..comment.len()-2];
        // If the comment started with /**\n, remove the \n too.
        if comment_interior.starts_with("\n") {
            comment_interior = &comment_interior[1..];
            first_line_was_removed = true;
        }
        // If the comment ended with */ on its own line, remove the last line.
        if comment_interior.ends_with("\n") {
            comment_interior = &comment_interior[..comment_interior.len()-1];
        } else if comment_interior.ends_with("\n ") {
            comment_interior = &comment_interior[..comment_interior.len()-2];
        }

        let lines = comment_interior.lines();
        let mut rebuilt = String::new();
        let mut first_line = true;
        for line in lines {
            let stripped_line = if !first_line_was_removed && first_line {
                line
            } else if line.starts_with(" * ") {
                &line[3..]
            } else {
                line
            };
            if !first_line {
                rebuilt.push('\n');
            } else {
                first_line = false;
            }
            rebuilt.push_str(stripped_line);
        }
        rebuilt.to_string()
    } else {
        comment.clone()
    }
}

pub fn get_doc(attrs: &Vec<syn::Attribute>, null_terminated: bool) -> syn::Lit {
    let mut doc = Vec::new();

    for attr in attrs.iter() {
        match attr.value {
            syn::MetaItem::NameValue(ref ident, ref lit) => {
                if ident.as_ref() == "doc" {
                    let s = match lit {
                        &syn::Lit::Str(ref cs, syn::StrStyle::Cooked) => cs,
                        _ => panic!("doc attribute wasn't a string, it was {:?}", lit),
                    };
                    let doc_part = clean_comment(s);
                    doc.push(doc_part)
                }
            }
            _ => (),
        }
    }
    let mut doc = doc.join("\n");
    if null_terminated {
        doc.push('\0');
    }
    syn::Lit::Str(doc, syn::StrStyle::Cooked)
}
