# Security Policy

## Reporting a vulnerability

**Please do not file public GitHub issues for security vulnerabilities.**

If you discover a vulnerability, report it privately via one of:

- **GitHub Security Advisories**: open a [private advisory](https://github.com/socket7/Livediff/security/advisories/new) on this repository
- **Email**: antonin.niv@gmail.com

Please include:

- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Any suggested mitigation

You can expect an initial response within **7 days**. We aim to issue a patched release within **30 days** of confirming a vulnerability.

## Do not paste real credentials in public

When filing regular (non-security) issues or PRs, **always redact** real credential values:

- API keys, access tokens, refresh tokens, passwords
- JWT payloads containing personally identifying information
- Contents of credential/config files

Use placeholders like `<REDACTED>` or `eyJhbGc...REDACTED...` in any shared snippets.

## Supported versions

Security patches are released for the **latest minor version** only. Please keep your installation up to date.
