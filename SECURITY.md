# Security Policy

This policy applies to the code, release packages, update logic, and build workflows maintained in this fork: [martian53058/coolapk-desktop](https://github.com/martian53058/coolapk-desktop). It is separate from the upstream project's security policy.

## Supported Versions

This is a personal-maintained fork. Security fixes are handled on a best-effort basis, with priority given to the latest stable release.

| Version | Handling |
| --- | --- |
| Latest stable release | Best-effort assessment and fixes |
| Older releases | No ongoing support guarantee |
| Development builds | Use at your own risk |

There is no guaranteed response or remediation timeline.

## Reporting a Vulnerability

Please do **not** disclose exploitable details in a public issue, pull request, discussion, or release comment.

1. If GitHub Private Vulnerability Reporting is enabled for this repository, use the [private reporting page](https://github.com/martian53058/coolapk-desktop/security/advisories/new).
2. If that page is unavailable, contact the maintainer through the [GitHub profile](https://github.com/martian53058) first and ask for a private reporting channel. Do not post the vulnerability details publicly.

A useful report includes:
- The affected release, commit, and operating system.
- The security impact and a concise reproduction procedure.
- Relevant logs or screenshots only after sensitive information has been removed.
- A proposed mitigation, if you already have one.

Before sharing any material, remove Cookie values, access tokens, authorization codes, account identifiers, private messages, personal files, and other sensitive data.

## Scope

Examples of issues that may be in scope:
- Exposure or unsafe storage/handling of local session credentials or private account data.
- Unintended access to native capabilities through Tauri commands, WebView content, or permission configuration.
- Cross-site scripting or untrusted content that can lead to a meaningful client-side security impact.
- Unsafe deep-link handling, command execution, path traversal, or unauthorized file access.
- Update logic that could accept tampered or unexpected packages.
- Build or release workflows that expose secrets or publish compromised artifacts.
- Sensitive information leaking through diagnostic logs or generated support files.

Issues in the following areas are outside this repository's direct control:
- Vulnerabilities in Coolapk's own servers or APIs.
- Account bans, service-side rate limits, or ordinary server-side business rules.
- Security issues confined to unrelated third-party services.
- A dependency advisory without a demonstrated impact on this application.

If a report concerns the upstream project rather than a change or release specific to this fork, it may also need to be reported to the [upstream repository](https://github.com/daimiaopeng/coolapk-desktop). Do not assume that reporting it here automatically notifies upstream maintainers.

## Response and Disclosure

The maintainer will make a reasonable effort to review reports, reproduce the issue, assess its impact, and prepare a fix or mitigation when feasible. Because this is a personal-maintained project, a response, fix, or release date cannot be guaranteed. Disclosure timing should be coordinated privately until users have a reasonable opportunity to update.

There is currently no bug bounty program.

## Safe Research

Please limit testing to systems and accounts you own or are authorized to test. Do not access other users' data, disrupt Coolapk services, run denial-of-service tests, or publish exploit details before coordinated disclosure.
