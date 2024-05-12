pub fn generate_homepage() -> String {
    use rtml::*;

    // Use the macros to generate some HTML
    let document: String = html!{
        .lang = "en",
            head!{
                title!{
                    "Title of the document"
                }
            },
            body!{
                    div!{
                        "text  测试",
                        h1!{
                            "This is a heading"
                        },
                        p!{
                            "This is a paragraph"
                        }
                    },
                    table!{
                        tr!{
                            td!["Cell 1,1"],
                            td!["Cell 1,2"]
                        },
                        tr!{
                            td!["Cell 2,1"],
                            td!["Cell 2,2"]
                        }
                    }
            }
    }.render();

    document
}