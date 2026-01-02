use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{
    AstNode,
    context::LintContext,
    fixer::{RuleFix, RuleFixer},
    rule::{DefaultRuleConfig, Rule},
};

fn require_unicode_regexp_diagnostic(span: Span) -> OxcDiagnostic {
    // See <https://oxc.rs/docs/contribute/linter/adding-rules.html#diagnostics> for details
    OxcDiagnostic::warn("Should be an imperative statement about what is wrong.")
        .with_help("Should be a command-like statement that tells the user how to fix the issue.")
        .with_label(span)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename_all = "camelCase")]
pub struct RequireUnicodeRegexpConfig {
    require_flag: RequireFlag,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename_all = "camelCase")]
pub enum RequireFlag {
    #[default]
    U,
    V,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, JsonSchema)]
pub struct RequireUnicodeRegexp(RequireUnicodeRegexpConfig);

impl Deref for RequireUnicodeRegexp {
    type Target = RequireUnicodeRegexpConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// See <https://github.com/oxc-project/oxc/issues/6050> for documentation details.
declare_oxc_lint!(
    /// ### What it does
    ///
    /// Briefly describe the rule's purpose.
    ///
    /// ### Why is this bad?
    ///
    /// Explain why violating this rule is problematic.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```js
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```js
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    RequireUnicodeRegexp,
    eslint,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details
    pending, // TODO: describe fix capabilities. Remove if no fix can be done,
             // keep at 'pending' if you think one could be added but don't know how.
             // Options are 'fix', 'fix_dangerous', 'suggestion', and 'conditional_fix_suggestion'
    config = RequireUnicodeRegexp,
);

impl Rule for RequireUnicodeRegexp {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        dbg!(&value);
        Ok(serde_json::from_value::<DefaultRuleConfig<Self>>(value)
            .unwrap_or_default()
            .into_inner())
    }

    // fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
    fn run_once(&self, ctx: &LintContext) {
        dbg!(&self.require_flag);
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        ("/foo/u", None),
        ("/foo/gimuy", None),
        ("RegExp('', 'u')", None),
        ("RegExp('', `u`)", None),
        ("new RegExp('', 'u')", None),
        ("RegExp('', 'gimuy')", None),
        ("RegExp('', `gimuy`)", None),
        ("RegExp(...patternAndFlags)", None),
        ("new RegExp('', 'gimuy')", None),
        ("const flags = 'u'; new RegExp('', flags)", None),
        ("const flags = 'g'; new RegExp('', flags + 'u')", None),
        ("const flags = 'gimu'; new RegExp('foo', flags[3])", None),
        ("new RegExp('', flags)", None),
        ("function f(flags) { return new RegExp('', flags) }", None),
        ("function f(RegExp) { return new RegExp('foo') }", None),
        ("function f(patternAndFlags) { return new RegExp(...patternAndFlags) }", None),
        ("new globalThis.RegExp('foo')", None), // { "ecmaVersion": 6 },
        ("new globalThis.RegExp('foo')", None), // { "ecmaVersion": 2017 },
        ("new globalThis.RegExp('foo', 'u')", None), // { "ecmaVersion": 2020 },
        ("globalThis.RegExp('foo', 'u')", None), // { "ecmaVersion": 2020 },
        ("const flags = 'u'; new globalThis.RegExp('', flags)", None), // { "ecmaVersion": 2020 },
        ("const flags = 'g'; new globalThis.RegExp('', flags + 'u')", None), // { "ecmaVersion": 2020 },
        ("const flags = 'gimu'; new globalThis.RegExp('foo', flags[3])", None), // { "ecmaVersion": 2020 },
        ("class C { #RegExp; foo() { new globalThis.#RegExp('foo') } }", None), // { "ecmaVersion": 2022 },
        ("/foo/u", Some(serde_json::json!([{ "requireFlag": "u" }]))),
        ("new RegExp('foo', 'u')", Some(serde_json::json!([{ "requireFlag": "u" }]))),
        ("/foo/v", None),                  // { "ecmaVersion": 2024 },
        ("/foo/gimvy", None),              // { "ecmaVersion": 2024 },
        ("RegExp('', 'v')", None),         // { "ecmaVersion": 2024 },
        ("RegExp('', `v`)", None),         // { "ecmaVersion": 2024 },
        ("new RegExp('', 'v')", None),     // { "ecmaVersion": 2024 },
        ("RegExp('', 'gimvy')", None),     // { "ecmaVersion": 2024 },
        ("RegExp('', `gimvy`)", None),     // { "ecmaVersion": 2024 },
        ("new RegExp('', 'gimvy')", None), // { "ecmaVersion": 2024 },
        ("const flags = 'v'; new RegExp('', flags)", None), // { "ecmaVersion": 2024 },
        ("const flags = 'g'; new RegExp('', flags + 'v')", None), // { "ecmaVersion": 2024 },
        ("const flags = 'gimv'; new RegExp('foo', flags[3])", None), // { "ecmaVersion": 2024 },
        ("/foo/v", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        ("new RegExp('foo', 'v')", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 }
    ];

    let fail = vec![
        (r#"/\a/"#, None),
        ("/foo/", None),
        ("/foo/gimy", None),
        ("RegExp()", None),
        ("RegExp('foo')", None),
        (r#"RegExp('\\a')"#, None),
        ("RegExp('foo', '')", None),
        ("RegExp('foo', 'gimy')", None),
        ("RegExp('foo', `gimy`)", None),
        ("new RegExp('foo')", None),
        ("new RegExp('foo',)", None), // {  "ecmaVersion": 2017,  },
        ("new RegExp('foo', false)", None),
        ("new RegExp('foo', 1)", None),
        ("new RegExp('foo', '')", None),
        ("new RegExp('foo', 'gimy')", None),
        ("new RegExp(('foo'))", None),
        ("new RegExp(('unrelated', 'foo'))", None),
        ("const flags = 'gi'; new RegExp('foo', flags)", None),
        ("const flags = 'gi'; new RegExp('foo', ('unrelated', flags))", None),
        ("let flags; new RegExp('foo', flags = 'g')", None),
        ("const flags = `gi`; new RegExp(`foo`, (`unrelated`, flags))", None),
        ("const flags = 'gimu'; new RegExp('foo', flags[0])", None),
        ("new window.RegExp('foo')", None), // { "globals": globals.browser },
        ("new global.RegExp('foo')", None), // { "sourceType": "commonjs" },
        ("new globalThis.RegExp('foo')", None), // { "ecmaVersion": 2020 },
        ("/foo/", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        ("/foo/u", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        ("/foo/u", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 6 },
        ("/[[a]/u", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        ("new RegExp('foo', 'u')", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        ("new RegExp('[[a]', 'u')", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        (r#"new RegExp("foo", "\u0067")"#, Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        (r#"new RegExp("foo", `\u0067`)"#, Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        (r#"new RegExp("foo", "\u0075")"#, Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        (r#"new RegExp("foo", `\u0075`)"#, Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        (
            r#"const regularFlags = "sm"; new RegExp("foo", `${regularFlags}g`)"#,
            Some(serde_json::json!([{ "requireFlag": "v" }])),
        ), // { "ecmaVersion": 2024 },
        (
            r#"const regularFlags = "smu"; new RegExp("foo", `${regularFlags}g`)"#,
            Some(serde_json::json!([{ "requireFlag": "v" }])),
        ), // { "ecmaVersion": 2024 },
        ("/foo/v", Some(serde_json::json!([{ "requireFlag": "u" }]))), // { "ecmaVersion": 2024 },
        ("new RegExp('foo')", Some(serde_json::json!([{ "requireFlag": "v" }]))), // { "ecmaVersion": 2024 },
        ("new RegExp('foo', 'v')", Some(serde_json::json!([{ "requireFlag": "u" }]))), // { "ecmaVersion": 2024 }
    ];

    Tester::new(RequireUnicodeRegexp::NAME, RequireUnicodeRegexp::PLUGIN, pass, fail)
        .test_and_snapshot();
}
