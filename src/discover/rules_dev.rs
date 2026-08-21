//! Rules maintained by this fork on top of the upstream rule set.

use super::rules::RtkRule;

pub const DEV_RULES: &[RtkRule] = &[
    RtkRule {
        pattern: r"^dart\s+analyze\b",
        rtk_cmd: "rtk dart analyze",
        rewrite_prefixes: &["dart analyze"],
        category: "Build",
        savings_pct: 75.0,
        ..RtkRule::DEFAULT
    },
    RtkRule {
        pattern: r"^dart\s+pub\s+(get|add|upgrade|downgrade|outdated)\b",
        rtk_cmd: "rtk dart pub",
        rewrite_prefixes: &["dart pub"],
        category: "PackageManager",
        savings_pct: 70.0,
        ..RtkRule::DEFAULT
    },
    RtkRule {
        pattern: r"^dart\s+test\b",
        rtk_cmd: "rtk dart test",
        rewrite_prefixes: &["dart test"],
        category: "Build",
        savings_pct: 75.0,
        ..RtkRule::DEFAULT
    },
    RtkRule {
        pattern: r"^dart\s+run\s+build_runner\b",
        rtk_cmd: "rtk dart run build_runner",
        rewrite_prefixes: &["dart run build_runner"],
        category: "Build",
        savings_pct: 70.0,
        ..RtkRule::DEFAULT
    },
    RtkRule {
        pattern: r"^flutter\s+pub\s+(get|add|upgrade|downgrade|outdated)\b",
        rtk_cmd: "rtk flutter pub",
        rewrite_prefixes: &["flutter pub"],
        category: "PackageManager",
        savings_pct: 70.0,
        ..RtkRule::DEFAULT
    },
    RtkRule {
        pattern: r"^flutter\s+test\b",
        rtk_cmd: "rtk flutter test",
        rewrite_prefixes: &["flutter test"],
        category: "Build",
        savings_pct: 75.0,
        ..RtkRule::DEFAULT
    },
    RtkRule {
        pattern: r"^flutter\s+analyze\b",
        rtk_cmd: "rtk flutter analyze",
        rewrite_prefixes: &["flutter analyze"],
        category: "Build",
        savings_pct: 75.0,
        ..RtkRule::DEFAULT
    },
    RtkRule {
        pattern: r"^flutter\s+build\b",
        rtk_cmd: "rtk flutter build",
        rewrite_prefixes: &["flutter build"],
        category: "Build",
        savings_pct: 75.0,
        ..RtkRule::DEFAULT
    },
];

#[cfg(test)]
mod tests {
    use super::DEV_RULES;
    use crate::discover::registry::{classify_command, rewrite_command, Classification};
    use crate::discover::rules::RULES;

    #[test]
    fn dev_rule_commands_are_unique() {
        for (index, rule) in DEV_RULES.iter().enumerate() {
            assert!(!RULES.iter().any(|base| base.rtk_cmd == rule.rtk_cmd));
            assert!(!DEV_RULES[index + 1..]
                .iter()
                .any(|other| other.rtk_cmd == rule.rtk_cmd));
        }
    }

    #[test]
    fn dev_rules_classify_and_rewrite() {
        let cases = [
            ("dart analyze", "rtk dart analyze"),
            ("dart pub get", "rtk dart pub get"),
            ("dart test", "rtk dart test"),
            (
                "dart run build_runner build",
                "rtk dart run build_runner build",
            ),
            ("flutter pub get", "rtk flutter pub get"),
            ("flutter test", "rtk flutter test"),
            ("flutter analyze", "rtk flutter analyze"),
            ("flutter build apk", "rtk flutter build apk"),
        ];

        for (command, expected) in cases {
            assert!(
                matches!(classify_command(command), Classification::Supported { .. }),
                "{command}"
            );
            assert_eq!(
                rewrite_command(command, &[], &[]).as_deref(),
                Some(expected)
            );
        }
    }
}
