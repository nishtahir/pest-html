#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HtmlParser;

#[cfg(test)]
mod tests {
    use super::{HtmlParser, Rule};
    use pest::Parser;

    /**
     * Awesome testing macro - sourced from Tera
     * https://github.com/Keats/tera/blob/701ed72438b788a1b2809aada80d8ae4c66b7e7d/src/parser/tests/lexer.rs#L5
     */
    macro_rules! assert_lex_rule {
        ($rule:expr, $input:expr) => {
            let res = HtmlParser::parse($rule, $input);
            println!("{:?}", $input);
            println!("{:#?}", res);
            if res.is_err() {
                println!("{}", res.unwrap_err());
                panic!();
            }
            assert!(res.is_ok());
            assert_eq!(res.unwrap().last().unwrap().into_span().end(), $input.len());
        };
    }

    #[test]
    fn html_comment() {
        assert_lex_rule!(Rule::html_comment, "<!-- comment content -->");
    }

    #[test]
    fn html_conditional_comment() {
        assert_lex_rule!(Rule::html_conditional_comment, "<![if !IE]>");
    }

    #[test]
    fn xml_declaration() {
        assert_lex_rule!(
            Rule::xml_declaration,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        );
    }

    #[test]
    fn cdata() {
        assert_lex_rule!(Rule::cdata, "<![CDATA[<sender>John Smith</sender>]]>");
    }

    #[test]
    fn dtd() {
        assert_lex_rule!(Rule::dtd, "<!DOCTYPE html>");
    }

    #[test]
    fn scriptlet() {
        let inputs = vec!["<% if ( hello ) {  } %>", "<? if ( hello ) {  } ?>"];
        for i in inputs {
            assert_lex_rule!(Rule::scriptlet, i);
        }
    }

    #[test]
    fn script_open() {
        assert_lex_rule!(Rule::script_open, "<script type=\"text/javascript\">");
    }

    #[test]
    fn style_open() {
        assert_lex_rule!(Rule::style_open, "<style type=\"text/css\">");
    }

    #[test]
    fn html_text() {
        assert_lex_rule!(Rule::html_text, "Random text");
    }

    #[test]
    fn tag_name() {
        let inputs = vec!["h1", "p", "script", "video"];
        for i in inputs {
            assert_lex_rule!(Rule::tag_name, i);
        }
    }

    #[test]
    fn hex_chars() {
        let inputs = vec!["#0123456789", "#DCDCDC", "#fff"];
        for i in inputs {
            assert_lex_rule!(Rule::hex_chars, i);
        }
    }

    #[test]
    fn dec_chars() {
        let inputs = vec!["12345%", "0%"];
        for i in inputs {
            assert_lex_rule!(Rule::dec_chars, i);
        }
    }

    #[test]
    fn html_attribute() {
        let inputs = vec![
            "href=\"https://www.w3schools.com\"",
            "style=\"color:red\"",
            "width=\"500\"",
            "important",
            "src=\"img_girl.jpg\"",
        ];
        for i in inputs {
            assert_lex_rule!(Rule::html_attribute, i);
        }
    }

    #[test]
    fn html_element() {
        let inputs = vec![
            "<img src=\"img_girl.jpg\" alt=\"Girl with a jacket\">",
            "<title>Title of the document</title>",
            "<html><head><title>Title of the document</title></head></html>",
            "<h1 style=\"font-size:1em;color:#fff\">Example SnowStorm page</h1>",
        ];
        for i in inputs {
            assert_lex_rule!(Rule::html_element, i);
        }
    }

    #[test]
    fn script() {
        let inputs = vec![
            "<script>document.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";</script>",
            "<script type=\"text/javascript\">//<![CDATA[var i = 10; if (i < 5) { }//]]></script>",
        ];
        for i in inputs {
            assert_lex_rule!(Rule::script, i);
        }
    }

    #[test]
    fn html_document() {
        let inputs = vec![
            r##"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN">
<html>
<head>
<title>DHTML SnowStorm: Basic Example (HTML4 DTD)</title>
<meta name="robots" content="noindex" />
<script src="snowstorm-min.js"></script>
</head>
<body style="background:#6699cc">
<h1 style="font-size:1em;color:#fff">Example SnowStorm page</h1>
<p style="font-size:1em;color:#fff">
 A single Javascript reference in the &lt;head&gt; tag is required for SnowStorm to work.<br />
 View the source of this page for reference.
</p>
<p style="font-size:1em;color:#fff">This page uses a HTML 4.01 transitional DTD, and thus Internet Explorer doesn't support position:fixed in "backCompat" rendering mode. The script should handle this case properly.</p>
</body>
</html>"##,
        ];
        for i in inputs {
            assert_lex_rule!(Rule::html_document, i);
        }
    }
}
