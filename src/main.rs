use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::time::Duration;

const DEFAULT_ZHIHU_USER_AGENT: &str = "Mozilla/5.0 (X11; U; Linux x86_64; en-US) AppleWebKit/540.0 (KHTML, like Gecko) Ubuntu/10.10 Chrome/9.1.0.0 Safari/540.0";
const ANDROID_API_VERSION: &str = "3.0.93";
const ANDROID_APP_VERSION: &str = "10.100.0";
const ANDROID_USER_AGENT: &str = "com.zhihu.android/Futureve/10.100.0 Mozilla/5.0 (Linux; Android 12; sdk_gphone64_arm64 Build/SE1A.220630.001.A1; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/57.0.1000.10 Mobile Safari/537.36";
const ANDROID_APP_ZA: &str = "OS=Android&Release=12&Model=sdk_gphone64_arm64&VersionName=10.100.0&VersionCode=30008&Product=com.zhihu.android&Width=1440&Height=2952&Installer=Market&DeviceType=AndroidPhone&Brand=google";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Web,
    Android,
}

struct Header {
    name: String,
    value: String,
}

struct Cli {
    mode: Mode,
    method: Option<String>,
    headers: Vec<Header>,
    data: Vec<Vec<u8>>,
    include_headers: bool,
    output: Option<PathBuf>,
    jq_filter: Option<String>,
    url: String,
}

#[derive(Clone, Copy)]
struct OutputStyle {
    pretty_json: bool,
    color: bool,
}

#[derive(Deserialize)]
struct AccountSession {
    #[serde(default)]
    login: bool,
    #[serde(default, rename = "userAgent")]
    user_agent: Option<String>,
    #[serde(default)]
    cookies: BTreeMap<String, String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("zhurl: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1).peekable();
    if args.peek().is_none() {
        print_usage();
        return Ok(());
    }

    let cli = parse_args(args)?;
    let account = load_account()?;
    validate_account(&account, cli.mode)?;

    let body = request_body(&cli.data);
    let method = request_method(&cli, body.is_some())?;
    let body_text = if cli.mode == Mode::Web {
        body.as_ref()
            .map(|bytes| {
                String::from_utf8(bytes.clone())
                    .map_err(|_| "signed request body must be UTF-8".to_string())
            })
            .transpose()?
    } else {
        None
    };
    let headers = build_headers(&cli, &account, body.is_some(), body_text.as_deref())?;
    let stdout = run_request(&cli, &method, headers, body)?;
    let output_style = output_style(&cli);

    let output = if let Some(filter) = &cli.jq_filter {
        apply_jq_filter(filter, &stdout, output_style)?
    } else {
        format_response_output(stdout, output_style)
    };
    write_output(&output, cli.output.as_ref())?;
    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Cli, String> {
    let mut mode = Mode::Web;
    let mut method = None;
    let mut headers = Vec::new();
    let mut data = Vec::new();
    let mut include_headers = false;
    let mut output = None;
    let mut jq_filter = None;
    let mut url = None;
    let mut positional_only = false;
    let mut iter = args.into_iter().peekable();

    while let Some(arg) = iter.next() {
        if positional_only {
            set_url(&mut url, arg)?;
            continue;
        }
        if arg == "--" {
            positional_only = true;
        } else if arg == "--web" {
            mode = Mode::Web;
        } else if arg == "--android" {
            mode = Mode::Android;
        } else if arg == "-i" || arg == "--include" {
            include_headers = true;
        } else if arg == "-I" || arg == "--head" {
            method = Some("HEAD".to_string());
        } else if arg == "-X" || arg == "--request" {
            method = Some(next_value(&mut iter, &arg)?);
        } else if let Some(value) = arg.strip_prefix("--request=") {
            method = Some(value.to_string());
        } else if arg == "-H" || arg == "--header" {
            headers.push(parse_header(&next_value(&mut iter, &arg)?)?);
        } else if let Some(value) = arg.strip_prefix("--header=") {
            headers.push(parse_header(value)?);
        } else if matches!(arg.as_str(), "-d" | "--data" | "--data-binary") {
            data.push(read_data_argument(&next_value(&mut iter, &arg)?, true)?);
        } else if let Some(value) = arg.strip_prefix("--data=") {
            data.push(read_data_argument(value, true)?);
        } else if let Some(value) = arg.strip_prefix("--data-binary=") {
            data.push(read_data_argument(value, true)?);
        } else if arg == "--data-raw" {
            data.push(read_data_argument(&next_value(&mut iter, &arg)?, false)?);
        } else if let Some(value) = arg.strip_prefix("--data-raw=") {
            data.push(read_data_argument(value, false)?);
        } else if arg == "-o" || arg == "--output" {
            output = Some(PathBuf::from(next_value(&mut iter, &arg)?));
        } else if let Some(value) = arg.strip_prefix("--output=") {
            output = Some(PathBuf::from(value));
        } else if arg == "--jq" {
            jq_filter = Some(next_value(&mut iter, &arg)?);
        } else if let Some(value) = arg.strip_prefix("--jq=") {
            jq_filter = Some(value.to_string());
        } else if arg == "-h" || arg == "--help" {
            print_usage();
            std::process::exit(0);
        } else if arg.starts_with('-') {
            return Err(format!("unsupported option {arg}"));
        } else {
            set_url(&mut url, arg)?;
        }
    }

    if include_headers && jq_filter.is_some() {
        return Err("--jq cannot be used with --include".to_string());
    }

    Ok(Cli {
        mode,
        method,
        headers,
        data,
        include_headers,
        output,
        jq_filter,
        url: url.ok_or_else(|| "missing URL".to_string())?,
    })
}

fn next_value(iter: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn set_url(url: &mut Option<String>, value: String) -> Result<(), String> {
    if url.replace(value).is_some() {
        return Err("multiple URLs are not supported".to_string());
    }
    Ok(())
}

fn parse_header(raw: &str) -> Result<Header, String> {
    let (name, value) = raw
        .split_once(':')
        .ok_or_else(|| format!("invalid header {raw:?}; expected 'Name: value'"))?;
    let name = name.trim();
    if name.is_empty() {
        return Err("header name cannot be empty".to_string());
    }
    Ok(Header {
        name: name.to_string(),
        value: value.trim_start().to_string(),
    })
}

fn read_data_argument(value: &str, allow_file: bool) -> Result<Vec<u8>, String> {
    if allow_file && let Some(path) = value.strip_prefix('@') {
        return fs::read(path).map_err(|error| format!("failed to read {path}: {error}"));
    }
    Ok(value.as_bytes().to_vec())
}

fn request_body(parts: &[Vec<u8>]) -> Option<Vec<u8>> {
    let mut iter = parts.iter();
    let first = iter.next()?.clone();
    Some(iter.fold(first, |mut acc, bytes| {
        acc.push(b'&');
        acc.extend_from_slice(bytes);
        acc
    }))
}

fn request_method(cli: &Cli, has_body: bool) -> Result<String, String> {
    let method = cli
        .method
        .as_deref()
        .unwrap_or(if has_body { "POST" } else { "GET" })
        .trim()
        .to_ascii_uppercase();
    if method.is_empty() || !method.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return Err(format!("invalid method {method:?}"));
    }
    Ok(method)
}

fn load_account() -> Result<AccountSession, String> {
    let path = account_file()?;
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn account_file() -> Result<PathBuf, String> {
    let home = env::var_os("HOME").ok_or_else(|| "HOME is not set".to_string())?;
    Ok(PathBuf::from(home)
        .join(".zhihu-plus-plus")
        .join("account.json"))
}

fn validate_account(account: &AccountSession, mode: Mode) -> Result<(), String> {
    if !account.login {
        return Err(
            "account is not logged in: ~/.zhihu-plus-plus/account.json has login=false".to_string(),
        );
    }
    if account
        .cookies
        .get("z_c0")
        .filter(|value| !value.is_empty())
        .is_none()
    {
        return Err("account is missing login cookie z_c0".to_string());
    }
    if mode == Mode::Web
        && account
            .cookies
            .get("d_c0")
            .filter(|value| !value.is_empty())
            .is_none()
    {
        return Err("account is missing signing cookie d_c0".to_string());
    }
    Ok(())
}

fn build_headers(
    cli: &Cli,
    account: &AccountSession,
    has_body: bool,
    body: Option<&str>,
) -> Result<Vec<Header>, String> {
    let mut headers = Vec::new();
    set_header(&mut headers, "Accept", "application/json, text/plain, */*");
    set_header(&mut headers, "Cookie", &cookie_header(&account.cookies));
    if let Some(xsrf) = account
        .cookies
        .get("_xsrf")
        .filter(|value| !value.is_empty())
    {
        set_header(&mut headers, "x-xsrftoken", xsrf);
    }

    match cli.mode {
        Mode::Web => {
            let user_agent = account
                .user_agent
                .as_deref()
                .filter(|value| !value.is_empty())
                .unwrap_or(DEFAULT_ZHIHU_USER_AGENT);
            set_header(&mut headers, "User-Agent", user_agent);
            set_header(&mut headers, "Referer", "https://www.zhihu.com/");
            set_header(&mut headers, "Origin", "https://www.zhihu.com");
            for (name, value) in zhihu_sign::sign_zhihu_request(
                &path_and_query(&cli.url)?,
                account.cookies["d_c0"].as_str(),
                body,
            ) {
                set_header(&mut headers, &name, &value);
            }
        }
        Mode::Android => {
            set_header(&mut headers, "User-Agent", ANDROID_USER_AGENT);
            set_header(&mut headers, "x-page-id", "132");
            set_header(&mut headers, "x-api-version", ANDROID_API_VERSION);
            set_header(&mut headers, "x-app-version", ANDROID_APP_VERSION);
            set_header(&mut headers, "x-app-za", ANDROID_APP_ZA);
            set_header(&mut headers, "x-app-bundleid", "com.zhihu.android");
            set_header(&mut headers, "x-app-flavor", "zhihuwap64");
            set_header(&mut headers, "x-app-build", "release");
            set_header(&mut headers, "x-network-type", "3G");
        }
    }

    if has_body && !contains_header(&headers, "Content-Type") {
        set_header(
            &mut headers,
            "Content-Type",
            "application/x-www-form-urlencoded",
        );
    }
    for header in &cli.headers {
        if header
            .name
            .bytes()
            .any(|byte| byte == b'\r' || byte == b'\n')
        {
            return Err(format!("invalid header name {:?}", header.name));
        }
        if header
            .value
            .bytes()
            .any(|byte| byte == b'\r' || byte == b'\n')
        {
            return Err(format!("invalid value for header {}", header.name));
        }
        set_header(&mut headers, &header.name, &header.value);
    }
    Ok(headers)
}

fn set_header(headers: &mut Vec<Header>, name: &str, value: &str) {
    headers.retain(|header| !header.name.eq_ignore_ascii_case(name));
    headers.push(Header {
        name: name.to_string(),
        value: value.to_string(),
    });
}

fn contains_header(headers: &[Header], name: &str) -> bool {
    headers
        .iter()
        .any(|header| header.name.eq_ignore_ascii_case(name))
}

fn cookie_header(cookies: &BTreeMap<String, String>) -> String {
    cookies
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join("; ")
}

fn path_and_query(url: &str) -> Result<String, String> {
    let without_fragment = url.split('#').next().unwrap_or(url);
    let after_scheme = without_fragment
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(without_fragment);
    let path_start = after_scheme.find(['/', '?']).unwrap_or(after_scheme.len());
    let path = &after_scheme[path_start..];
    if path.is_empty() {
        Ok("/".to_string())
    } else if path.starts_with('/') {
        Ok(path.to_string())
    } else if path.starts_with('?') {
        Ok(format!("/{path}"))
    } else {
        Err(format!("invalid URL path in {url}"))
    }
}

fn run_request(
    cli: &Cli,
    method: &str,
    headers: Vec<Header>,
    body: Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    let agent = request_agent();
    let mut response = if let Some(body) = body {
        let request = request_with_body(&agent, method, &cli.url)?;
        add_headers(request, &headers)
            .send(body.as_slice())
            .map_err(|error| format!("request failed: {error}"))?
    } else {
        let request = request_without_body(&agent, method, &cli.url)?;
        add_headers(request, &headers)
            .call()
            .map_err(|error| format!("request failed: {error}"))?
    };
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
        .body_mut()
        .read_to_vec()
        .map_err(|error| format!("failed to read response body: {error}"))?;

    let output = if cli.include_headers {
        response_with_headers(status, &headers, body)
    } else {
        body
    };
    Ok(output)
}

fn output_style(cli: &Cli) -> OutputStyle {
    let terminal_json = cli.output.is_none() && !cli.include_headers && io::stdout().is_terminal();
    OutputStyle {
        pretty_json: terminal_json,
        color: terminal_json,
    }
}

fn request_agent() -> ureq::Agent {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .http_status_as_error(false)
        .build();
    ureq::Agent::new_with_config(config)
}

fn request_without_body(
    agent: &ureq::Agent,
    method: &str,
    url: &str,
) -> Result<ureq::RequestBuilder<ureq::typestate::WithoutBody>, String> {
    Ok(match method {
        "GET" => agent.get(url),
        "DELETE" => agent.delete(url),
        "HEAD" => agent.head(url),
        "OPTIONS" => agent.options(url),
        "TRACE" => agent.trace(url),
        method => return Err(format!("unsupported HTTP method without body: {method}")),
    })
}

fn request_with_body(
    agent: &ureq::Agent,
    method: &str,
    url: &str,
) -> Result<ureq::RequestBuilder<ureq::typestate::WithBody>, String> {
    Ok(match method {
        "POST" => agent.post(url),
        "PUT" => agent.put(url),
        "PATCH" => agent.patch(url),
        "GET" => agent.get(url).force_send_body(),
        "DELETE" => agent.delete(url).force_send_body(),
        "OPTIONS" => agent.options(url).force_send_body(),
        "TRACE" => agent.trace(url).force_send_body(),
        "HEAD" => return Err("HEAD cannot send a request body".to_string()),
        method => return Err(format!("unsupported HTTP method with body: {method}")),
    })
}

fn add_headers<B>(
    mut request: ureq::RequestBuilder<B>,
    headers: &[Header],
) -> ureq::RequestBuilder<B> {
    for header in headers {
        request = request.header(header.name.as_str(), header.value.as_str());
    }
    request
}

fn response_with_headers(
    status: ureq::http::StatusCode,
    headers: &ureq::http::HeaderMap,
    mut body: Vec<u8>,
) -> Vec<u8> {
    let mut output = Vec::new();
    let reason = status.canonical_reason().unwrap_or("");
    output.extend_from_slice(format!("HTTP/1.1 {} {}\r\n", status.as_u16(), reason).as_bytes());
    for (name, value) in headers {
        output.extend_from_slice(name.as_str().as_bytes());
        output.extend_from_slice(b": ");
        output.extend_from_slice(value.as_bytes());
        output.extend_from_slice(b"\r\n");
    }
    output.extend_from_slice(b"\r\n");
    output.append(&mut body);
    output
}

fn format_response_output(output: Vec<u8>, style: OutputStyle) -> Vec<u8> {
    if !style.pretty_json && !style.color {
        return output;
    }
    format_json_bytes(&output, style).unwrap_or(output)
}

fn apply_jq_filter(filter: &str, input: &[u8], style: OutputStyle) -> Result<Vec<u8>, String> {
    let filter = jaq_all::data::compile(filter).map_err(format_jq_reports)?;
    let inputs = jaq_all::fmts::read::json::parse_many(input);
    let mut runner = jaq_all::data::Runner::default();
    runner.writer.pp = json_pretty_printer(style);
    let vars = Default::default();
    let mut output = Vec::new();

    jaq_all::data::run(
        &runner,
        &filter,
        vars,
        inputs,
        |error| error,
        |value| {
            let value = jaq_all::jaq_core::unwrap_valr(value).map_err(|error| error.to_string())?;
            jaq_all::fmts::write::write(&mut output, &runner.writer, &value)
                .map_err(|error| error.to_string())
        },
    )?;
    Ok(output)
}

fn format_json_bytes(input: &[u8], style: OutputStyle) -> Result<Vec<u8>, String> {
    let pp = json_pretty_printer(style);
    let mut output = Vec::new();
    let mut values = 0;
    for value in jaq_all::fmts::read::json::parse_many(input) {
        let value = value.map_err(|error| error.to_string())?;
        jaq_all::fmts::write::json::write(&mut output, &pp, 0, &value)
            .map_err(|error| error.to_string())?;
        output.push(b'\n');
        values += 1;
    }
    if values == 0 {
        return Err("no JSON value in response".to_string());
    }
    Ok(output)
}

fn json_pretty_printer(style: OutputStyle) -> jaq_all::json::write::Pp {
    let mut pp = jaq_all::json::write::Pp::default();
    if style.pretty_json {
        pp.indent = Some("  ".to_string());
        pp.sep_space = true;
    }
    if style.color {
        pp.styles = jaq_all::json::write::Styles::ansi();
    }
    pp
}

fn format_jq_reports(reports: Vec<jaq_all::load::FileReports>) -> String {
    reports
        .iter()
        .map(|report| jaq_all::load::FileReportsDisp::new(report).to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn write_output(bytes: &[u8], output: Option<&PathBuf>) -> Result<(), String> {
    if let Some(path) = output {
        fs::write(path, bytes)
            .map_err(|error| format!("failed to write {}: {error}", path.display()))
    } else {
        io::stdout()
            .lock()
            .write_all(bytes)
            .map_err(|error| format!("failed to write stdout: {error}"))
    }
}

fn print_usage() {
    eprintln!(
        r#"usage: zhurl [--web|--android] [--jq FILTER] [-X METHOD] [-H 'Name: value'] [-d DATA] [-i] [-o FILE] URL

Authenticated Zhihu API client using ~/.zhihu-plus-plus/account.json.

Modes:
  --web       Use Zhihu web headers and zhihu_sign x-zse-* signing (default)
  --android   Use Zhihu Android headers; no x-zse-* signing

Options:
  --jq FILTER       Run an embedded jaq filter over the JSON response
  -X, --request M   HTTP method. Supported: GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS, TRACE
  -H, --header H    Add or replace a header, e.g. -H 'Accept: application/json'
  -d, --data DATA   Send request body; repeated -d values are joined with '&'
  --data-binary D   Same as -d; @file reads body from file
  --data-raw DATA   Same as -d but does not treat @file specially
  -i, --include     Include response status line and headers in output
  -o, --output FILE Write output to file
  -h, --help        Show this help

Output:
  * When stdout is a TTY, JSON output is pretty-printed and colorized by default.
  * Piped output and -o/--output stay uncolored and machine-oriented.
  * -i/--include disables JSON formatting because headers are not JSON.

Differences from curl/jq:
  * zhurl does not execute system curl or system jq; HTTP is handled by ureq and --jq by embedded jaq.
  * Only common API-oriented HTTP methods and options listed above are supported.
  * No curl config, .netrc, cookie jar, proxy flags, multipart upload, retry flags, HTTP/2 controls, or shell pipeline behavior.
  * 4xx/5xx responses are printed like curl without --fail; transport/protocol errors still fail the command.
  * --jq accepts jaq-compatible jq filters for JSON responses, not every jq CLI flag or module-loading feature.
  * --jq cannot be combined with -i/--include because response headers are not JSON.
"#
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header_value<'a>(headers: &'a [Header], name: &str) -> Option<&'a str> {
        headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case(name))
            .map(|header| header.value.as_str())
    }

    #[test]
    fn extracts_signed_path_and_query() {
        assert_eq!(
            path_and_query("https://www.zhihu.com/api/v4/me?include=foo#fragment").unwrap(),
            "/api/v4/me?include=foo",
        );
        assert_eq!(
            path_and_query("https://www.zhihu.com?foo=bar").unwrap(),
            "/?foo=bar"
        );
        assert_eq!(path_and_query("/api/v4/me").unwrap(), "/api/v4/me");
    }

    #[test]
    fn rejects_jq_with_included_headers() {
        let error = match parse_args(
            ["--jq", ".", "-i", "https://www.zhihu.com/api/v4/me"]
                .into_iter()
                .map(String::from),
        ) {
            Ok(_) => panic!("--jq with --include should fail"),
            Err(error) => error,
        };
        assert_eq!(error, "--jq cannot be used with --include");
    }

    #[test]
    fn parses_jq_and_defaults_to_web_mode() {
        let cli = parse_args(
            ["--jq", ".id", "https://www.zhihu.com/api/v4/me"]
                .into_iter()
                .map(String::from),
        )
        .unwrap();
        assert_eq!(cli.mode, Mode::Web);
        assert_eq!(cli.jq_filter.as_deref(), Some(".id"));
    }

    #[test]
    fn only_web_mode_requires_signing_cookie() {
        let account = AccountSession {
            login: true,
            user_agent: None,
            cookies: BTreeMap::from([("z_c0".to_string(), "token".to_string())]),
        };

        assert!(validate_account(&account, Mode::Android).is_ok());
        assert_eq!(
            validate_account(&account, Mode::Web).unwrap_err(),
            "account is missing signing cookie d_c0",
        );
    }

    #[test]
    fn android_mode_sets_current_mobile_headers() {
        let cli = parse_args(
            [
                "--android",
                "https://api.zhihu.com/people/id/profile?profile_new_version=1&profile_v4=1",
            ]
            .into_iter()
            .map(String::from),
        )
        .unwrap();
        let account = AccountSession {
            login: true,
            user_agent: None,
            cookies: BTreeMap::from([("z_c0".to_string(), "token".to_string())]),
        };
        let headers = build_headers(&cli, &account, false, None).unwrap();

        assert_eq!(
            header_value(&headers, "User-Agent"),
            Some(ANDROID_USER_AGENT)
        );
        assert_eq!(header_value(&headers, "x-page-id"), Some("132"));
        assert_eq!(
            header_value(&headers, "x-api-version"),
            Some(ANDROID_API_VERSION)
        );
        assert_eq!(
            header_value(&headers, "x-app-version"),
            Some(ANDROID_APP_VERSION)
        );
        assert_eq!(header_value(&headers, "x-app-za"), Some(ANDROID_APP_ZA));
        assert_eq!(
            header_value(&headers, "x-app-bundleid"),
            Some("com.zhihu.android")
        );
        assert_eq!(header_value(&headers, "x-app-flavor"), Some("zhihuwap64"));
        assert_eq!(header_value(&headers, "x-app-build"), Some("release"));
        assert_eq!(header_value(&headers, "x-network-type"), Some("3G"));
    }

    #[test]
    fn terminal_json_output_is_pretty_and_colored() {
        let output = format_json_bytes(
            br#"{"answer":42}"#,
            OutputStyle {
                pretty_json: true,
                color: true,
            },
        )
        .unwrap();
        let text = String::from_utf8(output).unwrap();

        assert!(text.contains('\n'));
        assert!(text.contains("  "));
        assert!(text.contains("\x1b["));
    }
}
