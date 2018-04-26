extern crate web_view;

use web_view::*;

fn main() {
	  let size = (900, 600);
	  let resizable = true;
	  let debug = false;
	  let titlebar_transparent = true;
	  let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
	  let userdata = ();

    let html = r#"
    <html>
    <head>
    <style>
        html {
            margin: 0; padding: 0;
            width: 100%; height: 100%;
            overflow: hide;
            box-sizing: border-box;
        }

        *, *:after, *:before {
            box-sizing: inherit;
        }

        body {
            margin: 0; padding: 0;
            width: 100%; height: 100%;
        }

        iframe {
            width: 100%; height: 100%;
            margin: 0; padding: 0;
            border: 0;
        }
    </style>
    </head>
    <body>
        <iframe src="https://thefullsnack.com/kanban"></iframe>
    </body>
    </html>
    "#.to_string();

	  run(
		    "Snacky Slack",
		    Content::Html(html),
        // Or you can load the app directly from any URL
		    //Content::Url("https://thefullsnack.com/kanban"),
		    Some(size),
		    resizable,
		    debug,
        titlebar_transparent,
		    move |mut webview| {
            webview.set_background_color(1.0, 1.0, 1.0, 1.0);
        },
		    frontend_cb,
		    userdata
	  );
}
