pub fn format_boilerplate(filename: &str) -> String {
    format!(
"
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, minimal-ui\">
		<title>GitHub Markdown CSS demo</title>
  <style> 
    .preview-page {{
      margin-top: 64px;
    }}
    /* User-content tweaks */
    .timeline-comment-wrapper > .timeline-comment:after,
    .timeline-comment-wrapper > .timeline-comment:before {{
      content: none;
    }}
    /* User-content overrides */
    .discussion-timeline.wide {{
      width: 920px;
    }}
  </style>
</head> 
<div id=\"preview-page\" class=\"preview-page\" >
  <!-- <div id=\"readme\" 
class=\"Box Box--condensed instapaper_body md js-code-block-container
container new-discussion-timeline experiment-repo-nav\"> -->
  <!-- <div class=\"Box-header d-flex flex-items-center flex-justify-between px-2\"> 
    <h3 class=\"Box-title pr-3\">-->
    <div class=\"top-bar\">
    
    <!-- <svg xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 16 16\"><path fill-rule=\"evenodd\" d=\"M3 5h4v1H3V5zm0 3h4V7H3v1zm0 2h4V9H3v1zm11-5h-4v1h4V5zm0 2h-4v1h4V7zm0 2h-4v1h4V9zm2-6v9c0 .55-.45 1-1 1H9.5l-1 1-1-1H2c-.55 0-1-.45-1-1V3c0-.55.45-1 1-1h5.5l1 1 1-1H15c.55 0 1 .45 1 1zm-8 .5L7.5 3H2v9h6V3.5zm7-.5H9.5l-.5.5V12h6V3z\"/></svg> -->
    
   	<svg style=\"position: absolute; padding: 2px 0 0 1px;\" height=\"12px\" width=\"16px\" viewBox=\"2 2 13 12\" version=\"1.1\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M3 5h4v1H3V5zm0 3h4V7H3v1zm0 2h4V9H3v1zm11-5h-4v1h4V5zm0 2h-4v1h4V7zm0 2h-4v1h4V9zm2-6v9c0 .55-.45 1-1 1H9.5l-1 1-1-1H2c-.55 0-1-.45-1-1V3c0-.55.45-1 1-1h5.5l1 1 1-1H15c.55 0 1 .45 1 1zm-8 .5L7.5 3H2v9h6V3.5zm7-.5H9.5l-.5.5V12h6V3z\"></path></svg>
 
    <div class=\"file-name\">
   {} 
    </h3>
  </div>
  </div>
  <style>
  
 .top-bar {{
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  background-color: #f6f8fa;
  border:1px solid rgba(27,31,35,0.15);
  border-radius:3px 3px 0 0;
  border-style: solid solid none;
  margin: 64px auto 0;
  width: 100%;
  padding: 9px;
  font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\";
  font-weight: bold;
  font-size: 85%;
}}
.file-name {{
	margin-left: 22px;
}}
.markdown-body {{
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  margin: 0 auto 51px auto;
  padding: 45px;
  border:1px solid rgba(27,31,35,0.15);
  border-radius:0 0 3px 3px;
}}
@media (max-width: 767px) {{
  .markdown-body {{
    padding: 15px;
  }}
}}
 
    /*  
     #readme {{
                box-sizing: border-box;
                min-width: 200px;
                margin: 0 auto;
                max-width: 980px;
      }}
      */
    </style>
  <body>
    <article class=\"markdown-body\"> 
 ", filename)
}

pub fn format_boilerplate_no_preview(_filename: &str) -> String {
    format!(
"
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, minimal-ui\">
		<title>GitHub Markdown CSS demo</title>
  <style> 
    .preview-page {{
      margin-top: 64px;
    }}
    /* User-content tweaks */
    .timeline-comment-wrapper > .timeline-comment:after,
    .timeline-comment-wrapper > .timeline-comment:before {{
      content: none;
    }}
    /* User-content overrides */
    .discussion-timeline.wide {{
      width: 920px;
    }}
  </style>
</head>  
  <style>
  
 .top-bar {{
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  background-color: #f6f8fa;
  border:1px solid rgba(27,31,35,0.15);
  border-radius:3px 3px 0 0;
  border-style: solid solid none;
  margin: 64px auto 0;
  width: 100%;
  padding: 9px;
  font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\";
  font-weight: bold;
  font-size: 85%;
}}

.file-name {{
	margin-left: 22px;
}}

.markdown-body {{
  box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  margin: 0 auto 51px auto;
  padding: 45px;
  border:1px solid rgba(27,31,35,0.15);
  border-radius:0 0 3px 3px;
}}

@media (max-width: 767px) {{
  .markdown-body {{
    padding: 15px;
  }}
}}


 
    /*  
     #readme {{
                box-sizing: border-box;
                min-width: 200px;
                margin: 0 auto;
                max-width: 980px;
      }}
      */
    </style>
  <body>
    <article class=\"markdown-body\"> 
 ")
}

/*
pub static boilerplate: &str =
"
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1, minimal-ui\">
        <title>GitHub Markdown CSS demo</title>
  <style>
    .preview-page {{
      margin-top: 64px;
    }
    /* User-content tweaks */
    .timeline-comment-wrapper > .timeline-comment:after,
    .timeline-comment-wrapper > .timeline-comment:before {{
      content: none;
    }
    /* User-content overrides */
    .discussion-timeline.wide {{
      width: 920px;
    }
  </style>
</head>

<div id=\"preview-page\" class=\"preview-page\" >
  <div id=\"readme\"
class=\"Box Box--condensed instapaper_body md js-code-block-container
container new-discussion-timeline experiment-repo-nav\">
  <div class=\"Box-header d-flex flex-items-center flex-justify-between px-2\">
    <h3 class=\"Box-title pr-3\">
    <svg xmlns=\"http://www.w3.org/2000/svg\" width=\"16\" height=\"16\" viewBox=\"0 0 16 16\"><path fill-rule=\"evenodd\" d=\"M3 5h4v1H3V5zm0 3h4V7H3v1zm0 2h4V9H3v1zm11-5h-4v1h4V5zm0 2h-4v1h4V7zm0 2h-4v1h4V9zm2-6v9c0 .55-.45 1-1 1H9.5l-1 1-1-1H2c-.55 0-1-.45-1-1V3c0-.55.45-1 1-1h5.5l1 1 1-1H15c.55 0 1 .45 1 1zm-8 .5L7.5 3H2v9h6V3.5zm7-.5H9.5l-.5.5V12h6V3z\"/></svg>
   {}
    </h3>
  </div>

  <style>
    .markdown-body {
            box-sizing: border-box;
            min-width: 200px;
            max-width: 980px;
            margin: 0 auto;
            padding: 45px;
                }
     #readme {
                box-sizing: border-box;
                min-width: 200px;
                margin: 0 auto;
                max-width: 980px;
      }
    </style>
  <body>
    <article class=\"markdown-body\">
";
*/
pub static FOOTER: &str = "
  </style>
  </body>
  </div>
  </html>
";

pub static CSS: &str = "
<style>

@font-face {
  font-family: octicons-link;
  src: url(data:font/woff;charset=utf-8;base64,d09GRgABAAAAAAZwABAAAAAACFQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABEU0lHAAAGaAAAAAgAAAAIAAAAAUdTVUIAAAZcAAAACgAAAAoAAQAAT1MvMgAAAyQAAABJAAAAYFYEU3RjbWFwAAADcAAAAEUAAACAAJThvmN2dCAAAATkAAAABAAAAAQAAAAAZnBnbQAAA7gAAACyAAABCUM+8IhnYXNwAAAGTAAAABAAAAAQABoAI2dseWYAAAFsAAABPAAAAZwcEq9taGVhZAAAAsgAAAA0AAAANgh4a91oaGVhAAADCAAAABoAAAAkCA8DRGhtdHgAAAL8AAAADAAAAAwGAACfbG9jYQAAAsAAAAAIAAAACABiATBtYXhwAAACqAAAABgAAAAgAA8ASm5hbWUAAAToAAABQgAAAlXu73sOcG9zdAAABiwAAAAeAAAAME3QpOBwcmVwAAAEbAAAAHYAAAB/aFGpk3jaTY6xa8JAGMW/O62BDi0tJLYQincXEypYIiGJjSgHniQ6umTsUEyLm5BV6NDBP8Tpts6F0v+k/0an2i+itHDw3v2+9+DBKTzsJNnWJNTgHEy4BgG3EMI9DCEDOGEXzDADU5hBKMIgNPZqoD3SilVaXZCER3/I7AtxEJLtzzuZfI+VVkprxTlXShWKb3TBecG11rwoNlmmn1P2WYcJczl32etSpKnziC7lQyWe1smVPy/Lt7Kc+0vWY/gAgIIEqAN9we0pwKXreiMasxvabDQMM4riO+qxM2ogwDGOZTXxwxDiycQIcoYFBLj5K3EIaSctAq2kTYiw+ymhce7vwM9jSqO8JyVd5RH9gyTt2+J/yUmYlIR0s04n6+7Vm1ozezUeLEaUjhaDSuXHwVRgvLJn1tQ7xiuVv/ocTRF42mNgZGBgYGbwZOBiAAFGJBIMAAizAFoAAABiAGIAznjaY2BkYGAA4in8zwXi+W2+MjCzMIDApSwvXzC97Z4Ig8N/BxYGZgcgl52BCSQKAA3jCV8CAABfAAAAAAQAAEB42mNgZGBg4f3vACQZQABIMjKgAmYAKEgBXgAAeNpjYGY6wTiBgZWBg2kmUxoDA4MPhGZMYzBi1AHygVLYQUCaawqDA4PChxhmh/8ODDEsvAwHgMKMIDnGL0x7gJQCAwMAJd4MFwAAAHjaY2BgYGaA4DAGRgYQkAHyGMF8NgYrIM3JIAGVYYDT+AEjAwuDFpBmA9KMDEwMCh9i/v8H8sH0/4dQc1iAmAkALaUKLgAAAHjaTY9LDsIgEIbtgqHUPpDi3gPoBVyRTmTddOmqTXThEXqrob2gQ1FjwpDvfwCBdmdXC5AVKFu3e5MfNFJ29KTQT48Ob9/lqYwOGZxeUelN2U2R6+cArgtCJpauW7UQBqnFkUsjAY/kOU1cP+DAgvxwn1chZDwUbd6CFimGXwzwF6tPbFIcjEl+vvmM/byA48e6tWrKArm4ZJlCbdsrxksL1AwWn/yBSJKpYbq8AXaaTb8AAHja28jAwOC00ZrBeQNDQOWO//sdBBgYGRiYWYAEELEwMTE4uzo5Zzo5b2BxdnFOcALxNjA6b2ByTswC8jYwg0VlNuoCTWAMqNzMzsoK1rEhNqByEyerg5PMJlYuVueETKcd/89uBpnpvIEVomeHLoMsAAe1Id4AAAAAAAB42oWQT07CQBTGv0JBhagk7HQzKxca2sJCE1hDt4QF+9JOS0nbaaYDCQfwCJ7Au3AHj+LO13FMmm6cl7785vven0kBjHCBhfpYuNa5Ph1c0e2Xu3jEvWG7UdPDLZ4N92nOm+EBXuAbHmIMSRMs+4aUEd4Nd3CHD8NdvOLTsA2GL8M9PODbcL+hD7C1xoaHeLJSEao0FEW14ckxC+TU8TxvsY6X0eLPmRhry2WVioLpkrbp84LLQPGI7c6sOiUzpWIWS5GzlSgUzzLBSikOPFTOXqly7rqx0Z1Q5BAIoZBSFihQYQOOBEdkCOgXTOHA07HAGjGWiIjaPZNW13/+lm6S9FT7rLHFJ6fQbkATOG1j2OFMucKJJsxIVfQORl+9Jyda6Sl1dUYhSCm1dyClfoeDve4qMYdLEbfqHf3O/AdDumsjAAB42mNgYoAAZQYjBmyAGYQZmdhL8zLdDEydARfoAqIAAAABAAMABwAKABMAB///AA8AAQAAAAAAAAAAAAAAAAABAAAAAA==) format('woff');
}

.markdown-body .octicon {
  display: inline-block;
  fill: currentColor;
  vertical-align: text-bottom;
}

.markdown-body .anchor {
  float: left;
  line-height: 1;
  margin-left: -20px;
  padding-right: 4px;
}

.markdown-body .anchor:focus {
  outline: none;
}

.markdown-body h1 .octicon-link,
.markdown-body h2 .octicon-link,
.markdown-body h3 .octicon-link,
.markdown-body h4 .octicon-link,
.markdown-body h5 .octicon-link,
.markdown-body h6 .octicon-link {
  color: #1b1f23;
  vertical-align: middle;
  visibility: hidden;
}

.markdown-body h1:hover .anchor,
.markdown-body h2:hover .anchor,
.markdown-body h3:hover .anchor,
.markdown-body h4:hover .anchor,
.markdown-body h5:hover .anchor,
.markdown-body h6:hover .anchor {
  text-decoration: none;
}

.markdown-body h1:hover .anchor .octicon-link,
.markdown-body h2:hover .anchor .octicon-link,
.markdown-body h3:hover .anchor .octicon-link,
.markdown-body h4:hover .anchor .octicon-link,
.markdown-body h5:hover .anchor .octicon-link,
.markdown-body h6:hover .anchor .octicon-link {
  visibility: visible;
}

.markdown-body {
  -ms-text-size-adjust: 100%;
  -webkit-text-size-adjust: 100%;
  color: #24292e;
  line-height: 1.5;
  font-family: -apple-system,BlinkMacSystemFont,Segoe UI,Helvetica,Arial,sans-serif,Apple Color Emoji,Segoe UI Emoji,Segoe UI Symbol;
  font-size: 16px;
  line-height: 1.5;
  word-wrap: break-word;
}

.markdown-body .pl-c {
  color: #6a737d;
}

.markdown-body .pl-c1,
.markdown-body .pl-s .pl-v {
  color: #005cc5;
}

.markdown-body .pl-e,
.markdown-body .pl-en {
  color: #6f42c1;
}

.markdown-body .pl-s .pl-s1,
.markdown-body .pl-smi {
  color: #24292e;
}

.markdown-body .pl-ent {
  color: #22863a;
}

.markdown-body .pl-k {
  color: #d73a49;
}

.markdown-body .pl-pds,
.markdown-body .pl-s,
.markdown-body .pl-s .pl-pse .pl-s1,
.markdown-body .pl-sr,
.markdown-body .pl-sr .pl-cce,
.markdown-body .pl-sr .pl-sra,
.markdown-body .pl-sr .pl-sre {
  color: #032f62;
}

.markdown-body .pl-smw,
.markdown-body .pl-v {
  color: #e36209;
}

.markdown-body .pl-bu {
  color: #b31d28;
}

.markdown-body .pl-ii {
  background-color: #b31d28;
  color: #fafbfc;
}

.markdown-body .pl-c2 {
  background-color: #d73a49;
  color: #fafbfc;
}

.markdown-body .pl-c2:before {
  content: \"^M\";
}

.markdown-body .pl-sr .pl-cce {
  color: #22863a;
  font-weight: 700;
}

.markdown-body .pl-ml {
  color: #735c0f;
}

.markdown-body .pl-mh,
.markdown-body .pl-mh .pl-en,
.markdown-body .pl-ms {
  color: #005cc5;
  font-weight: 700;
}

.markdown-body .pl-mi {
  color: #24292e;
  font-style: italic;
}

.markdown-body .pl-mb {
  color: #24292e;
  font-weight: 700;
}

.markdown-body .pl-md {
  background-color: #ffeef0;
  color: #b31d28;
}

.markdown-body .pl-mi1 {
  background-color: #f0fff4;
  color: #22863a;
}

.markdown-body .pl-mc {
  background-color: #ffebda;
  color: #e36209;
}

.markdown-body .pl-mi2 {
  background-color: #005cc5;
  color: #f6f8fa;
}

.markdown-body .pl-mdr {
  color: #6f42c1;
  font-weight: 700;
}

.markdown-body .pl-ba {
  color: #586069;
}

.markdown-body .pl-sg {
  color: #959da5;
}

.markdown-body .pl-corl {
  color: #032f62;
  text-decoration: underline;
}

.markdown-body details {
  display: block;
}

.markdown-body summary {
  display: list-item;
}

.markdown-body a {
  background-color: transparent;
}

.markdown-body a:active,
.markdown-body a:hover {
  outline-width: 0;
}

.markdown-body strong {
  font-weight: inherit;
  font-weight: bolder;
}

.markdown-body h1 {
  font-size: 2em;
  margin: .67em 0;
}

.markdown-body img {
  border-style: none;
}

.markdown-body code,
.markdown-body kbd,
.markdown-body pre {
  font-family: monospace,monospace;
  font-size: 1em;
}

.markdown-body hr {
  box-sizing: content-box;
  height: 0;
  overflow: visible;
}

.markdown-body input {
  font: inherit;
  margin: 0;
}

.markdown-body input {
  overflow: visible;
}

.markdown-body [type=checkbox] {
  box-sizing: border-box;
  padding: 0;
}

.markdown-body * {
  box-sizing: border-box;
}

.markdown-body input {
  font-family: inherit;
  font-size: inherit;
  line-height: inherit;
}

.markdown-body a {
  color: #0366d6;
  text-decoration: none;
}

.markdown-body a:hover {
  text-decoration: underline;
}

.markdown-body strong {
  font-weight: 600;
}

.markdown-body hr {
  background: transparent;
  border: 0;
  border-bottom: 1px solid #dfe2e5;
  height: 0;
  margin: 15px 0;
  overflow: hidden;
}

.markdown-body hr:before {
  content: \"\";
  display: table;
}

.markdown-body hr:after {
  clear: both;
  content: \"\";
  display: table;
}

.markdown-body table {
  border-collapse: collapse;
  border-spacing: 0;
}

.markdown-body td,
.markdown-body th {
  padding: 0;
}

.markdown-body details summary {
  cursor: pointer;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  margin-bottom: 0;
  margin-top: 0;
}

.markdown-body h1 {
  font-size: 32px;
}

.markdown-body h1,
.markdown-body h2 {
  font-weight: 600;
}

.markdown-body h2 {
  font-size: 24px;
}

.markdown-body h3 {
  font-size: 20px;
}

.markdown-body h3,
.markdown-body h4 {
  font-weight: 600;
}

.markdown-body h4 {
  font-size: 16px;
}

.markdown-body h5 {
  font-size: 14px;
}

.markdown-body h5,
.markdown-body h6 {
  font-weight: 600;
}

.markdown-body h6 {
  font-size: 12px;
}

.markdown-body p {
  margin-bottom: 10px;
  margin-top: 0;
}

.markdown-body blockquote {
  margin: 0;
}

.markdown-body ol,
.markdown-body ul {
  margin-bottom: 0;
  margin-top: 0;
  padding-left: 0;
}

.markdown-body ol ol,
.markdown-body ul ol {
  list-style-type: lower-roman;
}

.markdown-body ol ol ol,
.markdown-body ol ul ol,
.markdown-body ul ol ol,
.markdown-body ul ul ol {
  list-style-type: lower-alpha;
}

.markdown-body dd {
  margin-left: 0;
}

.markdown-body code,
.markdown-body pre {
  font-family: SFMono-Regular,Consolas,Liberation Mono,Menlo,Courier,monospace;
  font-size: 12px;
}

.markdown-body pre {
  margin-bottom: 0;
  margin-top: 0;
}

.markdown-body input::-webkit-inner-spin-button,
.markdown-body input::-webkit-outer-spin-button {
  -webkit-appearance: none;
  appearance: none;
  margin: 0;
}

.markdown-body .border {
  border: 1px solid #e1e4e8!important;
}

.markdown-body .border-0 {
  border: 0!important;
}

.markdown-body .border-bottom {
  border-bottom: 1px solid #e1e4e8!important;
}

.markdown-body .rounded-1 {
  border-radius: 3px!important;
}

.markdown-body .bg-white {
  background-color: #fff!important;
}

.markdown-body .bg-gray-light {
  background-color: #fafbfc!important;
}

.markdown-body .text-gray-light {
  color: #6a737d!important;
}

.markdown-body .mb-0 {
  margin-bottom: 0!important;
}

.markdown-body .my-2 {
  margin-bottom: 8px!important;
  margin-top: 8px!important;
}

.markdown-body .pl-0 {
  padding-left: 0!important;
}

.markdown-body .py-0 {
  padding-bottom: 0!important;
  padding-top: 0!important;
}

.markdown-body .pl-1 {
  padding-left: 4px!important;
}

.markdown-body .pl-2 {
  padding-left: 8px!important;
}

.markdown-body .py-2 {
  padding-bottom: 8px!important;
  padding-top: 8px!important;
}

.markdown-body .pl-3,
.markdown-body .px-3 {
  padding-left: 16px!important;
}

.markdown-body .px-3 {
  padding-right: 16px!important;
}

.markdown-body .pl-4 {
  padding-left: 24px!important;
}

.markdown-body .pl-5 {
  padding-left: 32px!important;
}

.markdown-body .pl-6 {
  padding-left: 40px!important;
}

.markdown-body .f6 {
  font-size: 12px!important;
}

.markdown-body .lh-condensed {
  line-height: 1.25!important;
}

.markdown-body .text-bold {
  font-weight: 600!important;
}

.markdown-body:before {
  content: \"\";
  display: table;
}

.markdown-body:after {
  clear: both;
  content: \"\";
  display: table;
}

.markdown-body>:first-child {
  margin-top: 0!important;
}

.markdown-body>:last-child {
  margin-bottom: 0!important;
}

.markdown-body a:not([href]) {
  color: inherit;
  text-decoration: none;
}

.markdown-body blockquote,
.markdown-body dl,
.markdown-body ol,
.markdown-body p,
.markdown-body pre,
.markdown-body table,
.markdown-body ul {
  margin-bottom: 16px;
  margin-top: 0;
}

.markdown-body hr {
  background-color: #e1e4e8;
  border: 0;
  height: .25em;
  margin: 24px 0;
  padding: 0;
}

.markdown-body blockquote {
  border-left: .25em solid #dfe2e5;
  color: #6a737d;
  padding: 0 1em;
}

.markdown-body blockquote>:first-child {
  margin-top: 0;
}

.markdown-body blockquote>:last-child {
  margin-bottom: 0;
}

.markdown-body kbd {
  background-color: #fafbfc;
  border: 1px solid #c6cbd1;
  border-bottom-color: #959da5;
  border-radius: 3px;
  box-shadow: inset 0 -1px 0 #959da5;
  color: #444d56;
  display: inline-block;
  font-size: 11px;
  line-height: 10px;
  padding: 3px 5px;
  vertical-align: middle;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  font-weight: 600;
  line-height: 1.25;
  margin-bottom: 16px;
  margin-top: 24px;
}

.markdown-body h1 {
  font-size: 2em;
}

.markdown-body h1,
.markdown-body h2 {
  border-bottom: 1px solid #eaecef;
  padding-bottom: .3em;
}

.markdown-body h2 {
  font-size: 1.5em;
}

.markdown-body h3 {
  font-size: 1.25em;
}

.markdown-body h4 {
  font-size: 1em;
}

.markdown-body h5 {
  font-size: .875em;
}

.markdown-body h6 {
  color: #6a737d;
  font-size: .85em;
}

.markdown-body ol,
.markdown-body ul {
  padding-left: 2em;
}

.markdown-body ol ol,
.markdown-body ol ul,
.markdown-body ul ol,
.markdown-body ul ul {
  margin-bottom: 0;
  margin-top: 0;
}

.markdown-body li {
  word-wrap: break-all;
}

.markdown-body li>p {
  margin-top: 16px;
}

.markdown-body li+li {
  margin-top: .25em;
}

.markdown-body dl {
  padding: 0;
}

.markdown-body dl dt {
  font-size: 1em;
  font-style: italic;
  font-weight: 600;
  margin-top: 16px;
  padding: 0;
}

.markdown-body dl dd {
  margin-bottom: 16px;
  padding: 0 16px;
}

.markdown-body table {
  display: block;
  overflow: auto;
  width: 100%;
}

.markdown-body table th {
  font-weight: 600;
}

.markdown-body table td,
.markdown-body table th {
  border: 1px solid #dfe2e5;
  padding: 6px 13px;
}

.markdown-body table tr {
  background-color: #fff;
  border-top: 1px solid #c6cbd1;
}

.markdown-body table tr:nth-child(2n) {
  background-color: #f6f8fa;
}

.markdown-body img {
  background-color: #fff;
  box-sizing: content-box;
  max-width: 100%;
}

.markdown-body img[align=right] {
  padding-left: 20px;
}

.markdown-body img[align=left] {
  padding-right: 20px;
}

.markdown-body code {
  background-color: rgba(27,31,35,.05);
  border-radius: 3px;
  font-size: 85%;
  margin: 0;
  padding: .2em .4em;
}

.markdown-body pre {
  word-wrap: normal;
}

.markdown-body pre>code {
  background: transparent;
  border: 0;
  font-size: 100%;
  margin: 0;
  padding: 0;
  white-space: pre;
  word-break: normal;
}

.markdown-body .highlight {
  margin-bottom: 16px;
}

.markdown-body .highlight pre {
  margin-bottom: 0;
  word-break: normal;
}

.markdown-body .highlight pre,
.markdown-body pre {
  background-color: #f6f8fa;
  border-radius: 3px;
  font-size: 85%;
  line-height: 1.45;
  overflow: auto;
  padding: 16px;
}

.markdown-body pre code {
  background-color: transparent;
  border: 0;
  display: inline;
  line-height: inherit;
  margin: 0;
  max-width: auto;
  overflow: visible;
  padding: 0;
  word-wrap: normal;
}

.markdown-body .commit-tease-sha {
  color: #444d56;
  display: inline-block;
  font-family: SFMono-Regular,Consolas,Liberation Mono,Menlo,Courier,monospace;
  font-size: 90%;
}

.markdown-body .blob-wrapper {
  border-bottom-left-radius: 3px;
  border-bottom-right-radius: 3px;
  overflow-x: auto;
  overflow-y: hidden;
}

.markdown-body .blob-wrapper-embedded {
  max-height: 240px;
  overflow-y: auto;
}

.markdown-body .blob-num {
  -moz-user-select: none;
  -ms-user-select: none;
  -webkit-user-select: none;
  color: rgba(27,31,35,.3);
  cursor: pointer;
  font-family: SFMono-Regular,Consolas,Liberation Mono,Menlo,Courier,monospace;
  font-size: 12px;
  line-height: 20px;
  min-width: 50px;
  padding-left: 10px;
  padding-right: 10px;
  text-align: right;
  user-select: none;
  vertical-align: top;
  white-space: nowrap;
  width: 1%;
}

.markdown-body .blob-num:hover {
  color: rgba(27,31,35,.6);
}

.markdown-body .blob-num:before {
  content: attr(data-line-number);
}

.markdown-body .blob-code {
  line-height: 20px;
  padding-left: 10px;
  padding-right: 10px;
  position: relative;
  vertical-align: top;
}

.markdown-body .blob-code-inner {
  color: #24292e;
  font-family: SFMono-Regular,Consolas,Liberation Mono,Menlo,Courier,monospace;
  font-size: 12px;
  overflow: visible;
  white-space: pre;
  word-wrap: normal;
}

.markdown-body .pl-token.active,
.markdown-body .pl-token:hover {
  background: #ffea7f;
  cursor: pointer;
}

.markdown-body kbd {
  background-color: #fafbfc;
  border: 1px solid #d1d5da;
  border-bottom-color: #c6cbd1;
  border-radius: 3px;
  box-shadow: inset 0 -1px 0 #c6cbd1;
  color: #444d56;
  display: inline-block;
  font: 11px SFMono-Regular,Consolas,Liberation Mono,Menlo,Courier,monospace;
  line-height: 10px;
  padding: 3px 5px;
  vertical-align: middle;
}

.markdown-body :checked+.radio-label {
  border-color: #0366d6;
  position: relative;
  z-index: 1;
}

.markdown-body .tab-size[data-tab-size=\"1\"] {
  -moz-tab-size: 1;
  tab-size: 1;
}

.markdown-body .tab-size[data-tab-size=\"2\"] {
  -moz-tab-size: 2;
  tab-size: 2;
}

.markdown-body .tab-size[data-tab-size=\"3\"] {
  -moz-tab-size: 3;
  tab-size: 3;
}

.markdown-body .tab-size[data-tab-size=\"4\"] {
  -moz-tab-size: 4;
  tab-size: 4;
}

.markdown-body .tab-size[data-tab-size=\"5\"] {
  -moz-tab-size: 5;
  tab-size: 5;
}

.markdown-body .tab-size[data-tab-size=\"6\"] {
  -moz-tab-size: 6;
  tab-size: 6;
}

.markdown-body .tab-size[data-tab-size=\"7\"] {
  -moz-tab-size: 7;
  tab-size: 7;
}

.markdown-body .tab-size[data-tab-size=\"8\"] {
  -moz-tab-size: 8;
  tab-size: 8;
}

.markdown-body .tab-size[data-tab-size=\"9\"] {
  -moz-tab-size: 9;
  tab-size: 9;
}

.markdown-body .tab-size[data-tab-size=\"10\"] {
  -moz-tab-size: 10;
  tab-size: 10;
}

.markdown-body .tab-size[data-tab-size=\"11\"] {
  -moz-tab-size: 11;
  tab-size: 11;
}

.markdown-body .tab-size[data-tab-size=\"12\"] {
  -moz-tab-size: 12;
  tab-size: 12;
}

.markdown-body .task-list-item {
  list-style-type: none;
}

.markdown-body .task-list-item+.task-list-item {
  margin-top: 3px;
}

.markdown-body .task-list-item input {
  margin: 0 .2em .25em -1.6em;
  vertical-align: middle;
}

.markdown-body hr {
  border-bottom-color: #eee;
}

.markdown-body .pl-0 {
  padding-left: 0!important;
}

.markdown-body .pl-1 {
  padding-left: 4px!important;
}

.markdown-body .pl-2 {
  padding-left: 8px!important;
}

.markdown-body .pl-3 {
  padding-left: 16px!important;
}

.markdown-body .pl-4 {
  padding-left: 24px!important;
}

.markdown-body .pl-5 {
  padding-left: 32px!important;
}

.markdown-body .pl-6 {
  padding-left: 40px!important;
}

.markdown-body .pl-7 {
  padding-left: 48px!important;
}

.markdown-body .pl-8 {
  padding-left: 64px!important;
}

.markdown-body .pl-9 {
  padding-left: 80px!important;
}

.markdown-body .pl-10 {
  padding-left: 96px!important;
}

.markdown-body .pl-11 {
  padding-left: 112px!important;
}

.markdown-body .pl-12 {
  padding-left: 128px!important;
}




.Box {
    background-color: #fff;
    border: 1px solid #d1d5da;
    border-radius: 3px;
}



// change box size here
.Box--condensed { 
      box-sizing: border-box;
  min-width: 200px;
  max-width: 980px;
  background-color: #f6f8fa;
  border:1px solid rgba(27,31,35,0.15);
  border-radius:3px 3px 0 0;
  border-style: solid solid none;
  margin: 64px auto 0;
  width: 100%;
  padding: 9px;
  font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\";
  font-weight: bold;
  font-size: 85%;

    padding: 4px 8px;
}







.Box-header {
    height: 15px;
    padding: 16px;
    margin: -1px -1px 0;
    background-color: #f6f8fa;
    border: 1px solid #d1d5da;
    border-top-left-radius: 3px;
    border-top-right-radius: 3px;
}

h3 {
    font-size: 15px;
    line-height: 20px;
    margin: 0;
}




.top {
  padding: 8px 16px;
  display: flex!important;
  padding-right: 8px!important;
  padding-left: 8px!important;
  align-items: center!important;
  justify-content: space-between!important;
    margin: -1px -1px 0;


</style>
";

/*
pub static darkcss: &str = "
<style>
    /* ==UserStyle==
@name           GitHub Dark v2.6.3
@namespace      github.com/cquanu/github-dark
@version        2.6.22
@description    GitHub Dark 2.0
@author         Quan You, Tyler Thrailkill, Lance Jordan, Lexa Hall, Jared Allard, seanysean
@license        MIT
==/UserStyle== */
@-moz-document domain(\"github.com\") {
    /** Github Actions Start Block **/
    /* Colors */

    :root {
        --body-color: #1a2632;
        --accent-color: #243447;
        --depth-color: #151f29;
        --border-color: #304251;
        --link-color: #03A9F4;
        --icon-color: #8899a6;
        --primary-text-color: #ffffff;
        --secondary-text-color: #607d8b;
        --code-blob-color: #537698;
        --code-color: #6d6d72;
    }
    /** Github Actions End Block **/
    /* HTML Global */

    body {
        background-color: var(--body-color);
        color: var(--primary-text-color);
    }

    p {
        color: var(--secondary-text-color) !important;
    }

    h4 {
        color: var(--primary-text-color);
    }

    h4 > a {
        color: var(--primary-text-color);
    }

    /* Class Global */

    .lead {
        color: #929292;
    }

    /* Main Site and New User */

    .Header {
        background-color: var(--accent-color);
    }

    .site-header-nav .nav-item {
        color: var(--primary-text-color);
    }

    .jumbotron-home {
        background-image: none;
        background-color: var(--body-color);
    }

    .features-list .list-divider,
    .setup-form .tos-info,
    .setup-form .setup-organization-next {
        border-color: var(--border-color);
    }

    .radio-label {
        border-color: var(--border-color);
    }

    .tag-input-tag {
        background: var(--accent-color);
    }

    .tag-input-tag .remove {
        background: var(--body-color);
    }

    .home-hero-signup .form-control-lg {
        border-color: var(--border-color);
    }

    .shade-gray {
        background-color: #121b23 !important;
    }

    .tile-block {
        border-color: var(--border-color);
    }

    .tile-bordered:not(:last-child) {
        border-color: var(--border-color);
    }

    .js-notice {
        background-image: none;
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .Box-row--hover-blue:hover {
        background-color: var(--accent-color);
    }

    /* Main Site Login */

    .session-authentication {
        background-color: var(--body-color);
    }

    .session-authentication .header-logo {
        color: var(--primary-text-color);
    }

    .session-authentication .auth-form-body {
        border-color: var(--border-color);
    }

    .auth-form-body {
        background-color: var(--body-color);
    }

    .session-authentication .create-account-callout {
        border-color: var(--border-color);
    }

    .session-authentication .auth-form-header {
        color: var(--primary-text-color);
    }

    /* Site Wide Global */

    .header {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .header-logo-invertocat {
        color: var(--primary-text-color);
    }

    .header-search-scope {
        border-color: var(--border-color);
    }

    .header-nav-link {
        color: var(--primary-text-color);
    }

    /*.scoped-search .form-control .header-search-scope {
    background-color: var(--body-color);
    border-color: var(--border-color);
    }*/

    .form-control,
    .form-select {
        color: var(--primary-text-color);
        background-color: var(--depth-color);
        border-color: var(--border-color);
    }

    .form-control.focus,
    .form-control:focus,
    .form-select.focus,
    .form-select:focus {
        box-shadow: none;
    }

    .form-group .form-control:focus {
        background-color: var(--depth-color);
    }

    .btn {
        color: var(--primary-text-color);
        background-color: var(--body-color);
        background-image: none;
        border-color: var(--border-color);
    }

    .btn:hover,
    .btn:active,
    .btn.zeroclipboard-is-hover,
    .btn.zeroclipboard-is-active {
        text-decoration: none;
        background-color: var(--depth-color);
        background-image: none;
        border-color: var(--border-color);
    }

    .btn:active,
    .btn.selected,
    .btn.zeroclipboard-is-active,
    [open] > .btn {
        background-color: var(--body-color);
        background-image: none;
        border-color: var(--border-color);
    }

    .btn-primary {
        color: var(--primary-text-color);
        background-color: #57BB5C;
        background-image: none;
        border-color: #57BB5C;
    }

    .btn-theme-green {
        background-color: #6cc644;
    }

    .site-footer {
        border-color: var(--border-color);
    }

    .dropdown-menu {
        background-color: var(--body-color);
        border: 1px solid var(--border-color);
        box-shadow: none;
        color: var(--primary-text-color);
    }

    .header-nav-current-user .user-profile-link {
        color: var(--primary-text-color);
    }

    .dropdown-divider {
        background-color: var(--border-color);
    }

    .dropdown-item {
        color: #9e9e9e;
    }

    .dropdown-item:hover {
        background-color: var(--accent-color);
    }

    button.dropdown-item.dropdown-signout {
        color: #df3e3e;
    }

    /* Site Related */

    .setup-header {
        border-color: var(--border-color);
    }

    /* Home */

    .news .alert {
        border-color: var(--border-color);
    }

    .news .alert .octicon {
        color: var(--icon-color);
    }

    .news .alert .branch-link,
    .news .alert .pull-info {
        color: var(--depth-color);
        background: #627790;
    }

    .ajax-pagination-form .ajax-pagination-d {
        background: #babdcb;
    }
    .boxed-group>h3,
    .boxed-group .heading,
    .Box,
    .Box-header,
    .dashboard .js-all-activity-header + div {
        color: var(--primary-text-color);
        background-color: var(--accent-color);
        border: 1px solid var(--border-color);
    }

    .protected-branch-orgs-and-repo-admins {
        background-color: var(--body-color)
    }

    .dashboard-rollup-items .body {
        border-color: var(--border-color) !important;
    }
    .dashboard .js-all-activity-header + div {
        background-color: var(--body-color) !important;
        border: 1px solid var(--border-color) !important;
    }
    .boxed-group-inner {
        background: var(--body-color);
        border: 1px solid var(--border-color);
    }

    .mini-repo-list-item {
        border-color: var(--border-color);
    }

    .mini-repo-list-item .repo-icon {
        color: var(--icon-color);
    }

    .private .mini-repo-list-item {
        background-color: var(--accent-color);
    }

    .private .mini-repo-list-item .repo-icon {
        color: #fff9c4;
    }

    .filter-bar {
        background-color: var(--body-color);
        border-bottom: 1px solid var(--border-color);
    }

    .boxed-group .counter {
        color: var(--primary-text-color);
        background-color: var(--body-color);
        border: 1px var(--border-color) solid;
    }

    .subscribe-feed {
        color: var(--code-color);
    }

    .protip-callout {
        border-color: var(--border-color);
    }

    .octofication .broadcast-icon-mask {
        background-color: var(--body-color);
    }

    .link-gray-dark {
        color: #4078c0 !important;
    }

    .more-repos {
        text-align: center;
        background-color: var(--accent-color);
        box-shadow: none;
        border-top: 1px solid var(--border-color);
    }

    .github-jobs-logo strong {
        filter: invert(40%);
    }
    /* New Repo */

    .outline-box-highlighted {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .outline-box {
        border: solid 1px var(--border-color);
    }

    .copyable-terminal {
        background-color: var(--depth-color);
    }

    .reponame-suggestion {
        color: #4caf50;
    }

    .select-menu-item.selected, details-menu .select-menu-item[aria-checked=\"true\"], details-menu .select-menu-item[aria-selected=\"true\"] {
        color: var(--secondary-text-color);
    }

    /* Repo Page (Main) */

    .repohead.experiment-repo-nav {
        background-color: var(--depth-color);
    }

    .pagehead {
        border-color: var(--border-color);
    }

    .repository-meta {
        color: #9e9e9e;
    }

    .social-count {
        color: var(--secondary-text-color);
        background-color: var(--depth-color);
        border: 1px solid var(--border-color);
        border-left: 0;
    }

    .user-mention,
    .team-mention {
        color: #00aba5;
    }

    .reponav-item {
        color: #9e9e9e;
    }

    .reponav-item:hover,
    .reponav-item:focus {
        color: var(--primary-text-color);
    }

    .counter {
        background-color: var(--depth-color);
        border: 1px var(--border-color) solid;
    }

    .reponav-item .counter {
        color: #f9f9f9;
    }

    .reponav-item.selected {
        color: var(--primary-text-color);
        background-color: var(--body-color);
        border-color: #d28e5d var(--border-color) transparent;
    }

    .btn.selected:hover {
        background-color: var(--depth-color);
    }

    .overall-summary {
        border-color: var(--border-color);
    }

    .text-emphasized {
        color: var(--primary-text-color);
    }

    .numbers-summary a,
    .numbers-summary .nolink {
        color: #969ab0;
    }

    .repository-lang-stats-graph {
        border-color: var(--border-color);
    }

    .repository-lang-stats ol.repository-lang-stats-numbers li .language-color {
        border: 1px var(--border-color) solid;
    }

    .repository-lang-stats ol.repository-lang-stats-numbers li .lang {
        color: var(--primary-text-color);
    }

    table.files {
        background: var(--body-color);
    }

    table.files tr.navigation-focus td {
        background: var(--depth-color);
    }

    table.files td {
        border-color: var(--border-color);
    }

    table.files td.icon {
        color: var(--primary-text-color);
    }

    .warning {
        padding: 0.5em;
        margin-bottom: 0.8em;
        font-weight: bold;
        background-color: #d28e5d;
    }

    .file-wrap {
        border-color: var(--border-color);
    }

    .commit-author-section {
        color: #ccc;
    }

    .commit-tease {
        color: #8485ce;
        background-color: var(--accent-color);
        border-color: #3e5570;
    }

    .commits-list-item .commit-author {
        color: #00aba5;
    }

    .commit-tease-sha {
        color: #4078c0;
    }

    .commit-tease-contributors {
        background-color: var(--depth-color);
        border-color: #3e5570;
    }

    .commit .commit-title a {
        color: #4078c0;
    }

    .branch-infobar {
        background-color: var(--body-color);
        border-color: #3e5570;
    }

    .full-commit {
        background: var(--body-color);
        border: 1px solid var(--border-color);
    }

    .full-commit p.commit-title {
        color: #00aba5;
    }

    .full-commit .commit-meta {
        background: var(--accent-color);
        border-color: var(--border-color);
    }

    .full-commit .sha-block {
        color: var(--primary-text-color);
    }

    .full-commit .sha-block>.sha {
        color: #9e9e9e;
    }

    .full-commit .btn-outline,
    .full-commit .btn-outline:disabled {
        border-color: var(--border-color);
    }

    .commits-list-item.navigation-focus {
        background: var(--depth-color);
    }

    .file-navigation .get-repo-btn {
        border-color: var(--border-color);
    }

    .file-navigation .get-repo-btn:first-child {
        border-color: var(--border-color);
    }

    .signed-commit-badge {
        border-color: var(--border-color);
    }

    .signed-commit-signer-name .signer {
        display: block;
        font-weight: bold;
        color: #00aba5;
    }

    .flex-table-item-primary {
        color: var(--primary-text-color);
    }

    span.d-block {
        color: var(--primary-text-color);
    }

    .signed-commit-header {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    /* Repo find file */

    .tree-finder .tree-browser,
    .tree-browser td {
        border-color: var(--border-color) !important;
    }
    .tree-browser td {
        background: var(--background-color);
    }
    .tree-browser tr[aria-selected=\"true\"] td,
    .tree-browser tr.navigation-focus td {
        background: var(--depth-color);
    }

    /* Repo Blame */

    .blame-commit-title,
    .blame-commit-title .message {
        color: #4078c0;
    }

    .blame-commit,
    .blame-commit+.blame-line {
        border-color: var(--border-color);
    }

    .blame-blob-num {
        background-color: var(--accent-color);
    }

    .muted-link {
        color: #00aba5 !important;
    }

    /* README Markdown Live */

    .readme .markdown-body,
    .readme .plain {
        background-color: var(--depth-color);
        border-color: var(--border-color);
    }

    .markdown-body .highlight pre,
    .markdown-body pre {
        background-color: var(--body-color);
        border: 1px var(--border-color) solid;
    }

    .markdown-body h1 {
        border-color: var(--border-color);
    }

    .markdown-body img {
        background-color: transparent;
    }

    .markdown-body h1 .octicon-link,
    .markdown-body h2 .octicon-link,
    .markdown-body h3 .octicon-link,
    .markdown-body h4 .octicon-link,
    .markdown-body h5 .octicon-link,
    .markdown-body h6 .octicon-link {
        color: var(--primary-text-color);
    }

    .markdown-body table tr {
        background-color: var(--body-color);
    }

    .markdown-body table th,
    .markdown-body table td {
        border-color: var(--border-color);
    }

    .markdown-body table tr:nth-child(2n) {
        background-color: var(--accent-color);
    }

    .markdown-body pre code,
    .markdown-body pre tt {
        color: #969ab0;
    }

    .markdown-body code,
    .markdown-body tt {
        color: var(--depth-color);
        background-color: #627790;
    }

    .blob-num {
        color: #5e6b77;
        border-color: var(--border-color);
    }

    .blob-code-inner, .pl-mi {
        color: var(--code-color);
    }

    .blob-code-hunk,
    .blob-code-expandable {
        background-color: var(--accent-color);
    }

    .blob-num-hunk,
    .blob-num-expandable {
        background-color: var(--accent-color);
    }

    .blob-num-expandable {
        background-color: var(--accent-color);
    }

    .ace_editor .ace_gutter-active-line {
        background-color: var(--code-blob-color);
    }

    .ace_editor .ace_marker-layer .ace_active-line {
        background-color: var(--accent-color);
    }

    /* Code syntax Highlighting */

    .pl-pds,
    .pl-s,
    .pl-s .pl-pse .pl-s1,
    .pl-sr,
    .pl-sr .pl-cce,
    .pl-sr .pl-sra,
    .pl-sr .pl-sre {
        color: #4078c0;
    }

    .pl-s .pl-s1,
    .pl-smi {
        color: #bbb;
    }

    .pl-k {
        color: #b73999;
    }

    .ace_string {
        color: #4670d8;
    }

    .ace_support.ace_type {
        color: #b73999;
    }

    .ace_keyword {
        color: #8485ce;
    }

    .ace_variable {
        color: #d28e5d;
    }

    .ace_focus {
        background-color: #000;
    }

    .ace_markup.ace_heading {
        color: #4670d8;
    }

    /* Markdown Standalone */

    .markdown-body h2 {
        border-color: var(--border-color);
    }

    .markdown-body hr {
        background-color: var(--border-color);
    }

    .markdown-body blockquote {
        border-color: var(--border-color);
    }

    /* Repo Code Tab Live | Markdown */

    .file {
        border-color: var(--border-color);
    }

    .file-header {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .breadcrumb strong.final-path {
        color: #929292;
    }

    .ace_gutter {
        background: var(--body-color);
        border-color: var(--border-color);
    }

    .ace-github-light {
        color: #9e9e9e;
        background-color: var(--body-color);
    }

    .file-commit-form .commit-form {
        border-color: var(--border-color);
    }

    .commit-sha {
        background-color: inherit;
        border-color: var(--border-color);
    }

    /* Repo | Issues Tab */

    .table-list-header {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .Box-body-row--highlight .Box-row-link {
        color: #00aba5;
    }

    .Box-body-row {
        border-top: 1px solid var(--border-color);
    }

    .Box-body-row--highlight.navigation-focus,
    .Box-body-row--highlight:hover {
        background-color: var(--depth-color);
    }

    .issues-listing .table-list-issues .navigation-focus {
        background-color: var(--body-color);
    }

    .table-list {
        border-color: var(--border-color);
    }

    .table-list-cell {
        border-top-color: var(--border-color);
    }

    .table-list-bordered .table-list-cell:first-child {
        border-left-color: var(--border-color);
    }

    .table-list-bordered .table-list-cell:last-child {
        border-right-color: var(--border-color);
    }

    .table-list-header-toggle .btn-link {
        color: #9e9e9e;
    }

    .table-list-header-toggle .btn-link:hover {
        color: var(--primary-text-color);
    }

    .table-list-header-toggle .btn-link.selected,
    .table-list-header-toggle .btn-link.selected:hover {
        color: var(--primary-text-color);
    }

    .issue-title-link {
        color: #00aba5;
    }

    .protip {
        color: var(--code-color);
    }

    .milestone-title-link a {
        color: #cecece;
    }

    .Box-body {
        border-color: var(--border-color);
    }

    /* Dropdown Menu */

    .select-menu-modal {
        border-color: var(--border-color);
        box-shadow: none;
    }

    .select-menu-header {
        background: var(--accent-color);
        border-color: var(--border-color);
    }

    .select-menu-header .select-menu-title {
        color: #9e9e9e;
        text-shadow: none;
    }

    .select-menu-header {
        background: var(--accent-color);
        border-bottom: 1px solid var(--border-color);
    }

    .select-menu-item {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .select-menu-item.selected {
        color: var(--primary-text-color);
    }

    .select-menu-filters {
        background-color: var(--body-color);
    }

    .select-menu-text-filter:first-child:last-child {
        border-color: var(--border-color);
    }

    .select-menu-text-filter input {
        border-color: var(--border-color);
    }

    .select-menu-item[aria-checked=\"true\"] .select-menu-item-heading,
    .select-menu-item[aria-checked=\"true\"] svg {
        color: var(--primary-text-color) !important;
    }

    /* Repo | Pull Request Tab */

    .border-left,
    .border-right,
    .border-bottom {
        border-color: var(--border-color) !important;
    }

    .pr-toolbar.is-stuck::after {
        border-color: var(--border-color);
        box-shadow: none;
    }

    /* Repo Issues & Pull Request Convo */

    .js-comment-edit-history > .details-overlay > summary {
        background-color: var(--accent-color);
    }
    .gh-header {
        background-color: var(--body-color)
    }
    .gh-header .gh-header-sticky.is-stuck + .gh-header-shadow {
      background-color: var(--accent-color);
    }
    .gh-header-edit .edit-issue-title,
    .gh-header-edit .edit-issue-title:focus {
        background-color: var(--depth-color);
    }

    .gh-header-meta {
        border-color: var(--border-color);
    }

    .gh-header-meta .author {
        color: #00aba5;
    }

    .commit-ref {
        color: var(--depth-color);
        background-color: #627790;
    }

    .commit-ref .user {
        color: var(--depth-color);
    }

    .tabnav {
        border-color: var(--border-color);
    }

    .tabnav-tab.selected {
        color: var(--primary-text-color);
        border-color: var(--border-color);
        background-color: var(--body-color);
    }
    .tabnav-tab.selected, .tabnav-tab[aria-selected=\"true\"] {
      background-color: var(--body-color);
      color: var(--primary-text-color);
      border-color: var(--border-color);
    }

    .tabnav-pr .tabnav-tab.selected {
        color: var(--primary-text-color);
        border-color: var(--border-color);
    }

    .tabnav-pr {
        border-color: var(--border-color);
    }

    .tabnav-tab:hover {
        color: var(--primary-text-color)
    }

    .timeline-comment-wrapper {
        border: 0;
    }

    .timeline-comment-header {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .timeline-comment-header .author {
        color: #00aba5;
    }

    .timeline-comment,
    .timeline-comment .Box {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .timeline-comment-wrapper>.timeline-comment::before,
    .timeline-new-comment .timeline-comment::before {
        border-right-color: var(--border-color);
    }
    .timeline-comment {
        color: var(--primary-text-color)
    }

    timeline-comment::before {
        border-right-color: var(--accent-color) !important;
    }

    .timeline-comment::after {
        border-right-color: var(--accent-color) !important;
    }

    .timeline-comment.current-user .timeline-comment-header {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .timeline-comment.current-user {
        border-color: var(--border-color);
    }

    .timeline-comment.current-user .timeline-comment-label {
        border-color: var(--border-color);
    }

    .timeline-comment-wrapper .timeline-comment.current-user::before {
        border-width: 8px;
        border-right-color: var(--border-color);
    }

    .timeline-comment-label {
        border-color: var(--border-color);
    }

    .comment-reactions.has-reactions {
        border-color: var(--border-color);
    }

    .reaction-summary-item {
        border-color: var(--border-color);
    }

    .timeline-commits .commit-message>code a {
        color: #8485ce;
    }

    .discussion-timeline::before {
        width: 1px;
        background-color: var(--border-color);
    }

    .discussion-timeline-actions {
        background-color: var(--body-color);
        border: 0;
    }

    .discussion-item-ref-title .title-link {
        color: #8485ce;
    }

    .discussion-item-entity {
        color: #4078c0;
    }

    .discussion-item+.discussion-item {
        border-color: var(--border-color);
    }

    .discussion-sidebar-item+.discussion-sidebar-item {
        border-color: var(--border-color);
    }

    .discussion-item-icon {
        color: var(--icon-color);
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .discussion-item+.discussion-item-review {
        border-color: var(--border-color);
    }

    .discussion-item-review .file-header {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .discussion-item-review+.discussion-item {
        border-top: 1px solid var(--border-color);
    }

    .discussion-item-review .blob-wrapper {
        border-color: var(--border-color);
    }

    .review-comment-contents::after,
    .review-comment.is-comment-editing::after {
        background-color: var(--border-color);
    }

    .previewable-comment-form .comment-form-head.tabnav {
        background-color: var(--accent-color);
    }

    .new-discussion-timeline .previewable-comment-form .comment-form-head.tabnav {
        background: var(--accent-color);
    }

    .new-discussion-timeline .previewable-comment-form .comment-body {
        border-color: var(--border-color);
    }

    .new-discussion-timeline .closed-banner {
        background: var(--border-color);
        border-bottom: 18px solid var(--body-color);
    }

    .discussion-item .renamed-was,
    .discussion-item .renamed-is {
        color: #ccc;
    }

    .suggester {
        background: var(--accent-color);
        border-color: var(--border-color);
    }

    .suggester li {
        border-color: var(--border-color);
    }

    .previewable-comment-form textarea {
        background-color: var(--body-color);
    }

    .input-contrast:focus {
        background: var(--depth-color);
    }

    .upload-enabled textarea {
        border-color: var(--border-color);
    }

    .drag-and-drop {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .commit-icon .octicon {
        background-color: var(--body-color);
    }

    .issues-listing .table-list-issues .navigation-focus {
        background-color: var(--depth-color);
    }

    .branch-action-state-clean .branch-action-body::after {
        border-right-color: #6cc644;
    }

    .branch-action-state-unstable .branch-action-body::after {
        border-right-color: #cea61b;
    }

    .branch-action-body {
        background-color: var(--body-color);
    }

    .branch-action-body .merge-message,
    .branch-action-body .merge-branch-form {
        background-color: var(--accent-color);
        border-top: solid 1px var(--border-color);
    }

    .merge-branch-heading {
        color: var(--primary-text-color);
    }

    .branch-action-state-merged .branch-action-body {
        border-color: #6e5494;
    }

    .merge-status-list {
        border-color: var(--border-color);
    }

    .merge-status-item {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .branch-group-heading {
        background: var(--accent-color);
        border-color: var(--border-color);
    }

    .branch-action-item+.branch-action-item {
        border-color: var(--border-color);
    }

    .status-heading {
        color: var(--secondary-text-color);
    }

    .review-summary-form-wrapper {
        background-color: var(--body-color)
    }
    .TimelineItem-break {
        background-color: var(--body-color);
        border-top: 4px solid var(--accent-color);
    }

    /* Site | Issue Tab */

    .Box-row--focus-gray.navigation-focus {
        background-color: var(--accent-color);
    }

    .Box-row:first-of-type {
        border-color: var(--border-color);
    }

    .Box-row {
        border-color: var(--border-color);
    }

    a.Box-row-link.h4.js-navigation-open {
        color: #4078c0;
    }

    /* Code Tab | Branches */

    .branch-summary,
    .branch-group-heading+.branch-summary,
    .branch-summary {
        border-color: var(--border-color);
    }

    .branch-a-b-count .meter {
        background-color: var(--accent-color);
    }

    .branch-a-b-count .count-half:last-child {
        border-color: var(--border-color);
    }

    .count-value.count-behind {
        color: #dc3c3c;
    }

    .count-value.count-ahead {
        color: #52bd58;
    }

    a.branch-name {
        color: var(--depth-color);
        background-color: #627790;
    }

    .table-of-contents li+li {
        border-color: var(--border-color);
    }

    .commit-desc+.commit-branches {
        border-color: var(--border-color);
    }

    /* File Changed | Compare | Reviews */

    .diffbar {
        background-color: var(--body-color);
    }

    .diff-table tr:not(:last-child) .line-comments {
        border-color: var(--border-color);
    }

    .pr-toolbar {
        background-color: var(--body-color);
    }

    .range-editor {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .compare-pr-placeholder {
        background-color: var(--body-color);
        border-color: #cea61b;
    }

    ul.comparison-list {
        background: var(--body-color);
        border-color: var(--border-color);
    }

    ul.comparison-list>li.title {
        color: #9e9e9e;
        background: var(--accent-color);
    }

    ul.comparison-list>li {
        border-color: var(--border-color);
    }

    .file-diff-split .empty-cell {
        background-color: var(--body-color);
        border-right-color: var(--border-color);
    }

    .feature-callout-octicon {
        border-color: var(--border-color);
    }

    .feature-callout {
        border-color: var(--border-color);
    }

    .review-thread {
        border-color: var(--border-color);
    }

    .review-thread-reply {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .review-thread-reply .inline-comment-form {
        background-color: var(--body-color);
    }

    .review-comment .reaction-summary-item:not(.add-reaction-btn) {
        border-color: var(--border-color);
    }

    .last-review-thread {
        border-color: var(--border-color);
    }

    .inline-comment-form {
        border-color: var(--border-color);
    }

    /* Release & Tags */

    .release-timeline {
        border-color: var(--border-color);
        border-width: 1px;
    }

    .release-body {
        border-color: var(--border-color);
        border-width: 1px;
    }

    .releases-tag-list {
        border-color: var(--border-color);
    }

    .releases-tag-list tr {
        border-color: var(--border-color);
    }

    .release-show {
        border-color: var(--border-color);
    }

    .tabnav-extra {
        padding-right: 15px;
        color: #9e9e9e;
    }

    .tag-info h3 a .tag-name {
        color: #4078c0;
    }

    .release-downloads {
        border-color: var(--border-color);
    }

    .release-downloads li {
        border-color: var(--border-color);
    }

    /* Commits Tab */

    .commit-group-title .octicon-git-commit {
        margin-right: 17px;
        color: var(--border-color);
        background: var(--body-color);
    }

    .commits-listing::before {
        background-color: var(--border-color);
    }

    /* Nav */

    .subnav-item {
        border-color: var(--border-color);
    }

    .subnav-item:hover,
    .subnav-item:focus {
        background-color: var(--depth-color);
    }

    mark {
        color: var(--secondary-text-color);
    }
    .jump-to-field-active {
        color: var(--primary-text-color) !important;
    }

    .Box>#jump-to-results>.navigation-item>.jump-to-suggestions-path {
        color: var(--primary-text-color);
    }

    .Box>#jump-to-results>.navigation-item>.jump-to-suggestions-path:hover {
        color: var(--secondary-text-color);
    }

    .Box .jump-to-suggestions-results-container [aria-selected=\"true\"] .jump-to-suggestions-path,
    .jump-to-suggestions-results-container .navigation-focus .jump-to-suggestions-path {
        background-color: var(--depth-color);
    }

    /* Repo Wiki Tab */

    .blankslate {
        background-color: var(--depth-color) !important;
        border-color: var(--border-color);
        box-shadow: none;
    }

    .wiki-wrapper .wiki-auxiliary-content {
        background: var(--accent-color);
    }

    .wiki-rightbar .boxed-group.collapsed>h3 {
        border-color: var(--border-color);
    }

    .wiki-wrapper .wiki-auxiliary-content-no-bg {
        background: var(--body-color);
    }

    .wiki-wrapper .wiki-custom-sidebar {
        border-color: var(--border-color);
    }

    .boxed-group-table td {
        border-color: var(--border-color);
    }

    .boxed-group-table td {
        border-color: var(--border-color);
    }

    /* Repo Project Tab */

    .border {
        border-color: var(--border-color) !important;
    }

    .bg-gray {
        background-color: var(--body-color) !important;
    }

    .project-header {
        background-color: var(--body-color);
    }

    .project-columns {
        background-color: var(--body-color);
    }

    .project-column {
        background-color: #151f29;
    }

    .project-issue-body-blur {
        background: linear-gradient(0deg, var(--accent-color), hsla(0, 0%, 100%, 0));
    }
    .bg-white {
        background-color: var(--accent-color) !important;
    }

    .bg-gray-light {
        background-color: var(--body-color) !important;
    }

    .bg-blue-light {
        background-color: var(--accent-color) !important;
    }

    .js-project-column-cards {
        -ms-overflow-style: none;
    }

    .js-project-column-cards::-webkit-scrollbar {
        display: none;
    }

    /* Repo Pulse Tab */

    .pulse-graph:first-child {
        border-color: var(--border-color);
    }

    .pulse-graph {
        border-bottom-color: var(--border-color);
    }

    .conversation-list-heading .inner {
        background: var(--body-color);
    }

    .conversation-list-heading {
        border-color: var(--border-color);
    }

    .simple-conversation-list>li {
        border-color: var(--border-color);
    }

    .summary-stats li {
        border-left-color: var(--border-color);
    }

    .summary-stats li .num {
        color: #969ab0;
    }

    .summary-stats li a:hover {
        background: var(--depth-color);
    }

    .diffstat-summary strong {
        color: #84aaf7;
    }

    .axis line {
        stroke: var(--border-color);
    }

    .subhead {
        border: 0;
    }

    .Box {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .Box-header {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    /* Repo Graph | Contributors */

    .tint-box {
        background: var(--body-color);
    }

    .capped-card {
        border-color: var(--border-color);
    }

    .capped-card-content {
        background: var(--body-color);
    }

    .graphs .area {
        fill: #00aba5;
    }

    .capped-card h3 {
        border-color: var(--border-color);
    }

    /* Repo Graph | Punch Card */

    .day-name {
        fill: var(--icon-color);
    }

    circle.day {
        fill: #969ab0;
        stroke-width: 0;
    }

    :not(img) text,
    circle.day {
        fill: #969ab0;
    }

    line.axis {
        stroke: var(--border-color);
    }

    line.axis.even {
        stroke: var(--border-color);
    }

    :not(img) text,
    circle.day {
        fill: var(--icon-color);
    }

    /* Repo Graph | Traffic */

    table.capped-list td {
        border-color: var(--border-color);
    }

    table.capped-list th {
        border-color: var(--border-color);
    }

    table.capped-list tr:nth-child(even) {
        background-color: var(--accent-color);
    }

    /* Repo Graph | Code Frequency */

    .code-frequency .addition {
        fill: #52bd58;
    }

    .code-frequency .deletion {
        fill: #dc3c3c;
    }

    .cadd {
        font-weight: bold;
        color: #52bd58;
    }

    .cdel {
        font-weight: bold;
        color: #dc3c3c;
    }

    .graphs .dir {
        color: #9e9e9e;
    }

    /* Repo Settings Tab */

    .menu {
        margin-bottom: 15px;
        list-style: none;
        background-color: var(--body-color);
        border: 1px solid var(--border-color);
        border-radius: 3px;
    }

    .menu-item {
        text-shadow: none;
        border-bottom: 1px solid var(--border-color);
    }

    .menu-item:hover {
        background-color: var(--depth-color);
    }

    .menu-item.selected {
        color: var(--primary-text-color);
        background-color: var(--depth-color);
    }

    .form-group .form-control {
        background-color: var(--depth-color);
    }

    hr,
    .rule {
        border-bottom: 1px solid var(--border-color);
    }

    .boxed-group.dangerzone>h3 {
        text-shadow: none;
        border-color: #df3e3e;
    }

    .Box {
        background-color: var(--body-color);
        border: 1px solid var(--border-color);
    }

    .btn:disabled,
    .btn.disabled {
        color: white;
        background-color: var(--body-color);
        background-image: none;
        border-color: rgb(42, 58, 72);
        box-shadow: none;
    }

    .integrations-callout-standalone .integration-settings-callout {
        border-color: var(--border-color);
    }

    /* Repo Settings | Collaborators */

    .access-form-wrapper {
        background-color: var(--depth-color);
        border-color: var(--border-color);
    }
    .autocomplete-results {
        background-color: var(--depth-color);
    }

    /* Profile */

    .border-top {
        border-color: var(--border-color) !important;
    }

    .profilecols .filter-bar {
        background-color: var(--body-color);
    }

    .pinned-repo-list-item {
        border-color: var(--border-color);
    }

    .pinned-repo-filters {
        border-color: var(--border-color);
    }

    /* Not selected pinned item*/
    .pinned-item-name {
        color: var(--secondary-text-color);
    }

    /* selected pinned item */
    .pinned-item-checkbox:checked + .pinned-item-name {
        color: var(--primary-text-color);
        background-color: var(--body-color);
    }

    .user-profile-nav {
        background-color: var(--accent-color);
        border-color: var(--border-color);
        border-radius: 5px 5px 0px 0px;
    }
    .UnderlineNav-item {
        color: var(--primary-text-color);
    }

    .UnderlineNav-item.selected {
        color: var(--primary-text-color);
    }

    .UnderlineNav-item:hover {
        color: var(--secondary-text-color);
    }

    .border-gray-dark {
        border-color: var(--border-color) !important;
    }

    .underline-nav-item.selected {
        color: var(--primary-text-color);
    }

    .user-profile-sticky-bar::after {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .underline-nav-item:hover {
        color: var(--secondary-text-color);
    }

    .profile-timeline-card-wrapper {
        border: 0;
    }

    .profile-timeline.discussion-timeline::before {
        left: 11px;
        background-color: var(--border-color);
    }

    .profile-timeline.discussion-timeline .profile-timeline-month-heading::after {
        background-color: var(--border-color);
    }

    .profile-rollup-wrapper+.profile-rollup-wrapper {
        border-color: var(--border-color);
    }

    /* Profile | Contribution */

    #contributions-calendar rect[fill=\"#eeeeee\"] {
        fill: var(--depth-color);
    }

    /* Profile | Repo Tab */

    .profilecols .filter-bar .filter-selected {
        color: #8485ce;
    }

    /* Notification */

    .list-group-item.navigation-focus {
        background-color: var(--depth-color);
    }

    .list-group-item-link {
        color: #fdfdfd;
    }

    .boxed-group-list>li {
        border-color: var(--border-color);
    }

    .flash {
        color: white;
        background-color: #364952;
        border-color: #4c85a0;
    }

    .notifications-list .notifications .list-group-item:not(.confirmation):first-child {
        border-top: 0px solid var(--border-color);
    }

    .notifications-list .notifications .list-group-item:not(.confirmation) {
        border-top: 1px solid var(--border-color);
    }

    /* Organisation */

    .org-name {
        color: var(--primary-text-color);
    }

    .orghead {
        background-color: var(--depth-color);
        border-color: var(--border-color);
    }

    .simple-box {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .org-module-link {
        color: #00aba5;
    }

    .member-username {
        color: #00aba5;
    }

    .org-link {
        color: #00aba5;
    }

    .subnav-bordered {
        border-color: var(--border-color);
    }

    .org-module-title {
        border-color: var(--border-color);
    }

    .member-row {
        border-color: var(--border-color);
    }

    .pagehead-tabs-item.selected {
        color: var(--primary-text-color);
        background-color: var(--body-color);
        border-color: #d26911 var(--border-color) transparent;
    }

    /* Create Organisation */

    .setup-header {
        text-shadow: none;
    }

    .steps {
        border-color: var(--border-color);
    }

    .steps li {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .steps li.current {
        color: #9e9e9e;
        background-color: var(--accent-color);
    }

    .setup-info-module {
        background-color: var(--body-color);
        border-color: var(--border-color);
        box-shadow: none;
    }

    .setup-info-module h2 {
        border-color: var(--border-color);
    }

    .plan-choice.open,
    .plan-choice.selected {
        background-color: var(--body-color);
    }

    .plan-choice {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    /* Personal Settings (Global) */

    .menu-heading {
        color: var(--primary-text-color);
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .listgroup {
        border-color: var(--border-color);
        background-color: var(--accent-color);
    }

    .listgroup-overflow {
      background-color: var(--body-color);
    }

    .listgroup-item+.listgroup-item {
        border-color: var(--border-color);
    }

    .oauth-border {
        border-color: var(--border-color);
    }

    .saved-reply-form {
        border-color: var(--border-color);
    }

    .user-key-badge {
        border-color: var(--border-color);
    }

    dl.new-email-form {
        border-color: var(--border-color);
    }

    .Subhead {
        border-color: var(--border-color);
    }

    /* Settings | Security */

    .session-device.session-current,
    .session-device {
        background-color: var(--body-color);
    }

    .mute,
    .text-gray-dark {
        color: #aeaeae !important;
    }

    .boxed-group-list>li:first-child {
        border-color: var(--border-color);
    }

    .boxed-group-inner .help {
        border-color: var(--border-color);
    }

    .auth-form-header {
        background-color: var(--accent-color);
        border-color: var(--border-color);
    }

    .Box-row--gray {
        background-color: var(--accent-color);
    }

    /* Settings | Billing */

    .billing-section {
        border-color: var(--border-color);
    }

    .billing-section .section-content {
        color: #9e9e9e;
    }

    .radio-label {
        color: #666;
    }

    .boxed-group-table th {
        background-color: var(--accent-color);
    }

    .payment-history .refunded, .payment-history .failed {
        background-color: var(--accent-color);
    }

    /* Settings | Developer Settings | Personal Access Tokens */
    .token-scope {
        color: var(--primary-text-color)
    }

    /* Modal */

    .facebox-popup {
        background-color: var(--body-color);
        border: 1px solid var(--border-color);
    }

    .facebox-alert {
        color: #a9a495;
        background-color: #35342c;
        border-color: #6b5c20;
    }

    .facebox-header {
        border-color: var(--border-color);
    }

    .facebox-footer {
        margin: 0 -15px -15;
        background: var(--accent-color);
    }

    .facebox-footer {
        background: var(--accent-color);
        border-color: var(--border-color);
    }

    /* Scrollbar */

    ::-webkit-scrollbar {
        width: 10px;
    }

    ::-webkit-scrollbar-thumb {
        background-color: var(--accent-color);
        border-radius: 0;
    }

    ::-webkit-scrollbar-thumb:hover {
        background-color: #C4C4C4;
    }

    ::-webkit-scrollbar-track {
        background-color: var(--body-color);
    }

    /* Search Results */

    .repo-list-item {
        border-color: var(--border-color);
    }

    .codesearch-results .repo-list-name em,
    .codesearch-results .repo-list-description em {
        background-color: var(--accent-color);
    }

    .participation-graph rect {
        fill: var(--accent-color);
    }

    .code-list .file-box {
        border-color: var(--border-color);
    }

    .code-list .code-list-item+.code-list-item {
        border-color: var(--border-color);
    }

    .code-list .language {
        color: #9e9e9e;
    }

    .codesearch-aside .menu .octicon {
        color: var(--icon-color);
    }

    .codesearch-aside .filter-list {
        border-color: var(--border-color);
    }

    .codesearch-aside .filter-list li span.bar {
        background: var(--accent-color);
    }

    .filter-item:hover {
        background-color: var(--accent-color);
    }

    /* Helper */

    .timeline-commits .hidden-text-expander .ellipsis-expander {
        color: var(--icon-color);
        background: var(--accent-color);
    }

    .hidden-text-expander a,
    .ellipsis-expander {
        color: var(--icon-color);
        background: var(--accent-color);
    }

    /* Experimental | Enchancement */

    /* Blob Minus */

    .blob-num-deletion {
        background-color: #3b2343;
    }

    .blob-code-deletion {
        background-color: var(--body-color);
        text-decoration: line-through;
        color: #ef9a9a;
    }

    /* Blob Plus */

    .blob-num-addition {
        background-color: #a5d6a7;
    }

    .blob-code-addition {
        background-color: var(--body-color);
    }

    .blob-expanded .blob-num,
    .blob-expanded .blob-code {
        background-color: var(--accent-color);
    }

    /* End of Experimental */

    /* GitHub Gists */

    .header-logo-wordmark {
        color: var(--primary-text-color);
    }

    .gist-snippet-meta .description {
        color: var(--code-color);
    }

    /* Rework Buttons | Icons */

    .ajax-pagination-form .ajax-pagination-btn {
        color: #9e9e9e;
        background: var(--accent-color);
        border-color: var(--border-color);
    }

    .ajax-pagination-form .ajax-pagination-btn:hover {
        background-color: var(--depth-color);
    }

    .btn-outline {
        background-color: var(--body-color);
    }

    .btn-outline:disabled,
    .btn-outline:disabled:hover,
    .btn-outline.disabled,
    .btn-outline.disabled:hover {
        border-color: var(--border-color);
    }

    .btn-octicon {
        color: var(--icon-color);
    }

    .toolbar-item {
        color: var(--icon-color);
    }

    .toolbar-item .menu-target {
        color: var(--icon-color);
    }

    .timeline-comment-actions {
        color: var(--icon-color);
    }

    .discussion-sidebar-toggle .octicon {
        color: var(--icon-color);
    }

    h2 span {
        color: var(--secondary-text-color);
    }

    div.search-form.big button, div.search-form.big input {
        background-color: var(--accent-color);
    }

    /* Github Pricing */

    .shade-gradient {
        background-image: none;
    }

    .display-heading-1,
    .display-heading-2,
    .display-heading-3,
    .display-heading-4 {
        color: var(--primary-text-color);
    }

    .bg-white {
        background-color: var(--body-color) !important;
    }

    .pricing-card {
        background-color: var(--body-color);
        border-color: var(--border-color);
        box-shadow: none;
    }

    .pricing-card-vertical .pricing-card-name {
        border-color: var(--border-color);
    }

    .pricing-card-vertical .pricing-card-cta {
        border-color: var(--border-color);
    }

    /* GitHub Explore */

    .sort-bar {
        border-color: var(--border-color);
    }

    .pagehead-nav-item {
        color: #00aba5;
    }

    .pagehead-nav-item:hover {
        color: #8485ce;
    }

    .pagehead-nav-item.selected {
        color: #8485ce;
        border-color: #d28e5d;
    }

    .pagehead.explore-head {
        border-bottom-color: var(--border-color);
    }

    .exploregrid-item-title {
        color: var(--primary-text-color);
    }

    .exploregrid-item {
        color: #969ab0;
        border-color: var(--border-color);
        box-shadow: none;
    }

    .thread-subscription-status {
        background-color: var(--body-color);
        border: 1px solid var(--border-color);
    }

    .explore-section {
        border-color: var(--border-color);
    }

    .explore-section:nth-child(even) {
        background: var(--accent-color);
    }

    .explore-collection h2 {
        color: var(--primary-text-color);
    }

    .repo-collection>ul {
        background: var(--depth-color);
        border-color: var(--border-color);
    }

    .showcase-page-header {
        border-color: var(--border-color);
    }

    .showcase-page-repo-list {
        border-color: var(--border-color);
    }

    .showcase-page-pattern::after {
        background: none var(--body-color);
    }

    .intgrs-lstng-item {
        border-color: var(--border-color);
    }

    div.application-main > div.border-bottom,
    div.application-main div.container-xl > div.position-relative div[style] {
        background: var(--body-color) !important;
    }

    /* GitHub Blog */

    .blog-title {
        color: var(--primary-text-color);
    }

    /* Octotree Addon */

    .octotree_github_sidebar .octotree_views {
        background-color: var(--body-color);
        border-color: var(--border-color);
    }

    .octotree_github_sidebar .octotree_views .octotree_view .octotree_view_header {
        background-color: var(--body-color);
        border-color: var(--border-color);
        border-right-color: var(--border-color) !important;
        border-bottom-color: var(--border-color) !important;
    }

    .octotree_sidebar .ui-resizable-e {
        background-color: var(--border-color);
    }

    .markdown-body .task-list-item.hovered {
        background: #424a7e !important;
    }

    .discussion-item .renamed-was,
    .discussion-item .renamed-is {
        color: #607d8b;
    }

    .octotree-sidebar.octotree-github-sidebar .octotree-views {
        background-color: var(--body-color);
    }
    .octotree-sidebar.octotree-github-sidebar .octotree-views .octotree-tree-view .jstree-default .jstree-wholerow-clicked,
    .octotree-sidebar.octotree-github-sidebar .octotree-views .octotree-tree-view .jstree-default .jstree-wholerow-hovered {
        color: var(--primary-text-color);
        background-color: var(--accent-color) !important;
    }

    .octotree-sidebar.octotree-github-sidebar .octotree-views .octotree-tree-view .jstree-anchor,
    .octotree-sidebar.octotree-github-sidebar .octotree-views .octotree-view .jstree-anchor {
        color: var(--primary-text-color);
    }

    /** arrows for folders **/
    .octotree-sidebar.octotree-github-sidebar .octotree-views .octotree-tree-view .jstree-default .jstree-no-dots .jstree-open > .jstree-ocl::before,
    .octotree-sidebar.octotree-github-sidebar .octotree-views .octotree-tree-view .jstree-default .jstree-no-dots .jstree-closed > .jstree-ocl::before {
        color: var(--icon-color) !important;
    }
    .octotree-sidebar.octotree-github-sidebar .octotree-footer {
        background-color: var(--accent-color);
    }

    .octotree-sidebar.octotree-github-sidebar .octotree-footer .octotree-ad-small a {
        color: var(--secondary-text-color) !important;
    }
    /* End */

    /* Fix new additions to github. */

    .topic-tag {
        background-color: var(--border-color);
    }

    .topic-tag-action {
        background-color: var(--accent-color);
    }

    .delete-topic-button {
        background-color: var(--accent-color);
        border-left-color: var(--border-color);
    }

    .simple-box-footer {
        background-color: var(--border-color);
        border-top: var(--border-color);
    }

    .pagehead-tabs-item:hover {
        color: var(--primary-text-color);
        text-decoration: none;
    }

    .pagehead-tabs-item .counter {
        color: var(--primary-text-color);
        opacity: .85;
    }

    .reaction-summary-item {
        background-color: var(--body-color) !important;
    }

    /* calender */

    .calendar-graph rect[fill=\"#ebedf0\"],
    .calendar-graph rect[fill=\"#c6e48b\"],
    .calendar-graph rect[fill=\"#7bc96f\"],
    .calendar-graph rect[fill=\"#239a3b\"],
    .calendar-graph rect[fill=\"#196127\"],
    .heat {
        background-color: #4183C4 !important;
        fill: #4183C4 !important;
        opacity: .15;
    }

    .contrib-legend {
        display: none !important;
    }

    .calendar-graph .days-selected rect.day.active {
        stroke: #ddd !important;
    }

    .calendar-graph rect.day:hover {
        stroke: #fff !important;
    }

    /* reg colors */

    .calendar-graph rect[fill=\"#c6e48b\"] {
        opacity: .3;
    }

    .calendar-graph rect[fill=\"#7bc96f\"] {
        opacity: .5;
    }

    .calendar-graph rect[fill=\"#239a3b\"] {
        opacity: .7;
    }

    .calendar-graph rect[fill=\"#196127\"] {
        opacity: 1;
    }

    /* user fix */

    .underline-nav-item {
        color: #6a737d;
    }

    .underline-nav-item .counter {
        color: var(--primary-text-color);
        opacity: .65;
    }

    .underline-nav-item:hover {
        color: var(--primary-text-color);
    }

    .notification-indicator .mail-status {
        background-color: #00aba5 !important;
        background-image: none !important;
    }

    .vcard-username {
        color: var(--primary-text-color);
        opacity: .65;
    }

    /* User Circled Images */

    .vcard-avatar img {
        border-radius: 100% !important;
        border: 3px solid var(--border-color) !important;
    }

    /* settings user fix */

    .Box .Box-row .listgroup-item-body {
        color: var(--secondary-text-color) !important;
    }

    .Box .Box-row .listgroup-item-title {
        opacity: .85;
    }

    /* feed fix, user & owner color */

    .simple .title a:first-child,
    .body .title a:first-child,
    .public .author a,
    .owner {
        color: #00aba5 !important;
    }

    .member-name {
        color: #FFF;
        opacity: .75;
    }

    .member-name:hover {
        opacity: 1;
    }

    /* org fix */
    .UnderlineNav {
        border-color: var(--border-color);
    }
    .unstyled-members-count,
    .js-repositories-count {
        color: #FFF;
    }

    .js-stat-label {}

    .note-emphasis {
        color: #FFF !important;
    }

    .team-actions,
    .team-info-card,
    .stats-group-stat {
        border-color: var(--border-color) !important;
    }
    /* blog */

    .blog-post-title a {
        color: #FFF !important;
        opacity: .85;
    }

    .blog-post-body h2 {
        border-color: var(--border-color) !important;
    }

    .blog-post-body h2,
    .blog-post-body h3 {
        color: #FFF;
    }

    .blog-post-body ul,
    .blog-post-body ol {
        color: var(--secondary-text-color);
    }

    .pagination,
    .pagination a,
    .pagination span {
        border-color: var(--border-color) !important;
        background-color: var(--body-color) !important;
    }

    .pagination a,
    .pagination .next_page,
    .pagination .previous_page {
        color: #FFF;
    }

    .meta-item:nth-child(2) a {
        color: #00aba5 !important;
    }

    .site-footer-marketing-col ul li a {
        color: #0366d6 !important;
    }

    /* fix #39 */

    .code-list .divider .blob-num,
    .code-list .divider .blob-code {
        background-color: var(--depth-color);
    }

    /* codemirror vs ace fix */

    .CodeMirror,
    .CodeMirror-lines,
    .CodeMirror-gutters {
        background-color: var(--body-color) !important;
    }

    /* editor: line nums */

    .CodeMirror-linenumber {
        color: #5e6b77 !important;
        border-color: var(--border-color);
    }

    /* editor: syntax */

    .CodeMirror-line span {
        color: var(--code-color);
    }

    .cm-string {
        color: #4078c0 !important;
    }

    .cm-keyword {
        color: #b73999 !important;
    }

    .cm-property,
    .cm-atom,
    .cm-number,
    .cm-meta {
        color: #0086b3 !important;
    }

    .cm-qualifier,
    .cm-variable-3 {
        color: #795da3 !important;
    }

    .cm-tag {
        color: #63a35c !important;
    }

    /** Marketplace **/

    .MarketplaceSideNav {
        background-color: var(--body-color);
        border-right: 1px solid var(--accent-color);
    }

    .MarketplaceBackground-buffer {
        background-color: var(--body-color);
    }

    div.application-main > div.border-bottom,
    div.application-main div.container-xl > div.position-relative div[style] {
        background: var(--body-color) !important;
    }
}
</style>
";
*/
